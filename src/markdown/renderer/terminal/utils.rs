pub const CODE_BLOCK_COUNTER_SPACE: usize = 3;
pub const CODE_BLOCK_LINE_CHAR: [char; 4] = ['─', '│', '┬', '┴'];
pub const CODE_BLOCK_MARGIN: usize = 1;
pub const TEXT_BULLETS: [char; 4] = ['•', '◦', '▪', '▫'];

pub fn repeat_char(c: char, n: usize) -> String {
    // let mut s = String::with_capacity(n);
    let mut s = String::new();
    for _ in 0..n {
        s.push(c);
    }
    s
}
