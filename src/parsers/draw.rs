use crossterm::{style::*, terminal::{Clear, ClearType}, queue, cursor::MoveToColumn};
use lazy_static::lazy_static;


pub enum LineKind {
    Top,
    Middle,
    Bottom,
    None
}
const LINE_COUNTER_SPACE: usize = 3;

pub fn clear_line() {
    queue!(std::io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).unwrap();
}

pub fn plainline() {
    lazy_static!{
        static ref LINE_STR: String = "─".repeat(crossterm::terminal::size().unwrap().0 as usize);
    };
    queue!(std::io::stdout(), SetForegroundColor(Color::DarkGrey), Print(LINE_STR.as_str()), ResetColor).unwrap();
}
pub fn line(linekind: LineKind) {
    const PADDING: usize = 3;
    const SIZE_CHAR_SEP: usize = 1;

    lazy_static!{
        static ref LINE_SPACE_STR: String = "─".repeat(LINE_COUNTER_SPACE + PADDING);
        static ref LINE_END_STR: String = "─".repeat(crossterm::terminal::size().unwrap().0 as usize - (LINE_COUNTER_SPACE + PADDING + SIZE_CHAR_SEP));
    };
    let char_sep = match linekind {
        LineKind::Top => '┬',
        LineKind::Middle => '┼',
        LineKind::Bottom => '┴',
        LineKind::None => '─'
    };
    queue!(std::io::stdout(), SetForegroundColor(Color::DarkGrey), Print(LINE_SPACE_STR.as_str()), Print(char_sep), Print(LINE_END_STR.as_str()), ResetColor).unwrap();
    // println!("{}──{}─{}{}{}", SetForegroundColor(Color::DarkGrey), LINE_SPACE_STR.as_str(), char_sep, LINE_END_STR.as_str(), ResetColor)
}
pub fn code_block_line(line_count: u32, line: &str) {
    // println!("{0}  {1:>2$} │{3} {4}", SetForegroundColor(Color::DarkGrey), line_count, LINE_COUNTER_SPACE, ResetColor, line);
    queue!(std::io::stdout(), SetForegroundColor(Color::DarkGrey), Print(format!("  {0:>1$} │", line_count, LINE_COUNTER_SPACE)), ResetColor, Print(line)).unwrap();
}
pub fn code_block(block: &str) {
    line(LineKind::Top);
    block.lines().enumerate().for_each(|(nb, line)| code_block_line((nb + 1) as u32, line));
    line(LineKind::Bottom);
}
pub fn text(line: &str) {
    queue!(std::io::stdout(), Print(line)).unwrap();
    // println!("{}", line);
}
pub fn title(level: usize, title: &str) {
    let term_width = crossterm::terminal::size().unwrap().0 as usize;
    let line_left_len = term_width / (1<<level) - 2 - title.len() / 2;
    let line_right_len = term_width - line_left_len - 2 - title.len();
    queue!(
        std::io::stdout(),
        ResetColor,
        Print("─".repeat(line_left_len)),
        // Blue foreground
        SetAttribute(Attribute::Reverse),
        SetAttribute(Attribute::Bold),
        // Print text
        Print(format!(" {} ", title)),
        // Reset to default colors
        ResetColor,
        Print("─".repeat(line_right_len))
    ).unwrap();
}


#[derive(Clone, Copy)]
enum CommandType {
    Colors(Option<Color>, Option<Color>),
    Attributes(Attribute),
}

impl CommandType {
    fn apply(&self) -> Result<(), std::io::Error>{
        match self {
            CommandType::Colors(Some(fg), Some(bg)) => queue!(std::io::stdout(), SetForegroundColor(*fg), SetBackgroundColor(*bg)),
            CommandType::Colors(Some(fg), None) => queue!(std::io::stdout(), SetForegroundColor(*fg)),
            CommandType::Colors(None, Some(bg)) => queue!(std::io::stdout(), SetBackgroundColor(*bg)),
            CommandType::Colors(None, None) => queue!(std::io::stdout(), ResetColor),
            CommandType::Attributes(attr) => queue!(std::io::stdout(), SetAttribute(*attr)),
        }
    }
    fn reset_color() -> Self {
        CommandType::Colors(None, None)
    }
    fn foreground_color(color: Color) -> Self {
        CommandType::Colors(Some(color), None)
    }
    fn background_color(color: Color) -> Self {
        CommandType::Colors(None, Some(color))
    }
    fn attribute(attr: Attribute) -> Self {
        CommandType::Attributes(attr)
    }
}

struct MarkdownHelper {
    commands: Vec<CommandType>,
    prefix: String,
}

pub fn markdown(blocks: Vec<markdown::Block>) {
    let mut md_helper = MarkdownHelper::new();
    md_helper.blocks(blocks);
}

macro_rules! mdhelper_push_and_pop {
    ($md_helper:ident, $spans: ident, $command:expr $(, $other_commands:expr)* $(,)?) => {
        {
            $md_helper.push($command);
            mdhelper_push_and_pop!($md_helper, $spans$(,$other_commands)*);
            $md_helper.pop();
        }
    };
    // ($md_helper:ident, $spans:ident, $command:expr) => {
    //     $md_helper.push($command);
    //     mdhelper_push_and_pop!($md_helper);
    //     $md_helper.pop();
    // };
    ($md_helper:ident, $spans:ident) => {
        $md_helper.spans($spans);
    };
}
macro_rules! mdhelper_apply {
    ($md_helper:ident, $spans: ident, $command:expr $(, $other_commands:expr)* $(,)?) => {
        {
            mdhelper_push_and_pop!( $md_helper, $spans, $command $(, $other_commands)* );
            queue!(std::io::stdout(), ResetColor, SetAttribute(Attribute::Reset)).unwrap();
            $md_helper.apply();
        }
    }
}

impl MarkdownHelper {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            prefix: String::new(),
        }
    }
    fn push(&mut self, command: CommandType) {
        self.commands.push(command);
        command.apply();
    }
    fn pop(&mut self) -> Option<CommandType> {
        self.commands.pop()
    }
    fn apply(&self) -> Result<(), std::io::Error> {
        self.commands.iter().map(CommandType::apply).fold(Ok(()), Result::and)
    }
    fn apply_last(&self) -> Result<(), std::io::Error> {
        self.commands.last().map(CommandType::apply).unwrap_or(Ok(()))
    }

    pub fn blocks(&mut self, blocks: Vec<markdown::Block>) {
        blocks.into_iter().for_each(|block| self.block(block));
    }
    pub fn block(&mut self, block: markdown::Block) {
        match block {
            markdown::Block::Paragraph(spans) => self.spans(spans),
            markdown::Block::CodeBlock(language, code) => code_block(&code),
            markdown::Block::Header(title, level) => self.title(level, title).unwrap(),
            markdown::Block::Blockquote(blocks) => {
                let prev = self.prefix.clone();
                self.prefix.push_str("  ");
            },
            markdown::Block::UnorderedList(blocks) => todo!(),
            markdown::Block::OrderedList(blocks, kind) => todo!(),
            _ => ()
        }
    }
    pub fn spans(&mut self, spans: Vec<markdown::Span>) {
        spans.into_iter().for_each(|span| self.span(span));
    }
    pub fn span(&mut self, span: markdown::Span) {
        match span {
            markdown::Span::Text(text) => {
                self::text(&text);
            },
            markdown::Span::Code(code) => {
                queue!(std::io::stdout(), PrintStyledContent(code.with(Color::Yellow))).unwrap();
            },
            markdown::Span::Break => self::plainline(),
            markdown::Span::Link(_, _, _) => todo!(),
            markdown::Span::Image(_, alt, _) => queue!(std::io::stdout(), Print(format!("[Image alt: {}]", alt))).unwrap(),
            markdown::Span::Emphasis(spans) => mdhelper_apply!(self, spans, CommandType::Attributes(Attribute::Italic)),
            markdown::Span::Strong(spans) => mdhelper_apply!(self, spans, CommandType::Attributes(Attribute::Bold)),
        }
    }
    fn title(&mut self, level: usize, title: Vec<markdown::Span>) -> Result<(), std::io::Error> {
        let term_width = crossterm::terminal::size().unwrap().0 as usize;
        let spans_length = Self::markdown_spans_len(&title);
        let line_left_len = term_width / (1<<level) - 2 - spans_length / 2;
        let line_right_len = term_width - line_left_len - 2 - spans_length;
        queue!(
            std::io::stdout(),
            ResetColor,
            Print("─".repeat(line_left_len)),
            Print(" "),
        )?;
        mdhelper_apply!(self, title, CommandType::attribute(Attribute::Bold), CommandType::attribute(Attribute::Reverse));
        queue!(
            std::io::stdout(),
            Print(" "),
            Print("─".repeat(line_right_len))
        )
    }
    
    fn markdown_spans_len(spans: &Vec<markdown::Span>) -> usize {
        spans.iter().map(|span| Self::markdown_span_len(span)).sum()
    }
    fn markdown_span_len(span: &markdown::Span) -> usize {
        match span {
            markdown::Span::Text(text) => text.len(),
            markdown::Span::Code(code) => code.len(),
            markdown::Span::Break => 0,
            markdown::Span::Link(_, _, _) => todo!(),
            markdown::Span::Image(_, _, _) => todo!(),
            markdown::Span::Emphasis(spans) => Self::markdown_spans_len(spans),
            markdown::Span::Strong(spans) => Self::markdown_spans_len(spans),
        }
    }

}