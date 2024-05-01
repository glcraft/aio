use std::ops::{Deref, DerefMut};

use llama_cpp::{
    LlamaModel as Model,
    Token
};

macro_rules! stop_manager {
    ($($x:expr),*) => {{
        let mut x = StopManager::new();
        $(x.add_stop_from_string($x);)*
        x
    }};
}
pub(crate) use stop_manager;

pub struct StopToken(String);

impl StopToken {
    pub fn new(text: String) -> Self {
        Self(text,)
    }
    pub fn from_tokens(model: Model, tokens: &[Token]) -> Self {
        Self(model.decode_tokens(tokens),)
    }
    pub fn inspect(&self) -> StopTokenInspector {
        StopTokenInspector::new(self)
    }
}

enum StopTokenState {
    NotFound,
    InProgress,
    Found,
}

enum Index<'a> {
    Ref(&'a mut usize),
    Val(usize),
}

impl AsRef<usize> for Index<'_> {
    fn as_ref(&self) -> &usize {
        match self {
            Index::Ref(x) => x,
            Index::Val(x) => x,
        }
    }
}

impl AsMut<usize> for Index<'_> {
    fn as_mut(&mut self) -> &mut usize {
        match self {
            Index::Ref(x) => x,
            Index::Val(x) => x,
        }
    }
}
impl Deref for Index<'_> {
    type Target = usize;
    fn deref(&self) -> &usize {
        match self {
            Index::Ref(x) => x,
            Index::Val(x) => x,
        }
    }
}

impl DerefMut for Index<'_> {
    fn deref_mut(&mut self) -> &mut usize {
        match self {
            Index::Ref(x) => x,
            Index::Val(x) => x,
        }
    }
}

impl<'a> From<&'a mut usize> for Index<'a> {
    fn from(x: &'a mut usize) -> Self {
        Self::Ref(x)
    }
}

impl From<usize> for Index<'_> {
    fn from(x: usize) -> Self {
        Self::Val(x)
    }
}

pub struct StopTokenInspector<'a, 'b> {
    stop: &'a StopToken,
    index: Index<'b>,
}

impl<'a, 'b> StopTokenInspector<'a, 'b> {
    pub fn new(stop: &'a StopToken) -> Self {
        Self {
            stop,
            index: 0.into(),
        }
    }
    pub fn reset(&mut self) {
        self.index = 0.into();
    }
    pub fn check(&mut self, text: &str) -> StopTokenState {
        for c_other in text.chars() {
            let c_self = self.stop.0.chars().nth(*self.index).unwrap();
            if c_self != c_other {
                *self.index = 0;
            } else {
                *self.index += 1;
                if *self.index == self.stop.0.len() {
                    return StopTokenState::Found;
                }
            }
        }
        StopTokenState::InProgress
    }
}

pub struct StopManager {
    stops: Vec<(StopToken, usize)>,
}

impl StopManager {
    pub fn new() -> Self {
        Self {
            stops: Vec::new(),
        }
    }
    pub fn add_stop(&mut self, stop: StopToken) {
        self.stops.push((stop, 0));
    }
    pub fn add_stop_from_string<S: Into<String>>(&mut self, stop: S) {
        self.stops.push((StopToken::new(stop.into()), 0));
    }

    pub fn check(&mut self, text: &str) -> bool {
        for (stop, index) in &mut self.stops {
            let mut stop_inspector = StopTokenInspector { stop, index: index.into() };
            match stop_inspector.check(text) {
                StopTokenState::Found => {
                    self.reset();
                    return true;
                }
                StopTokenState::InProgress | StopTokenState::NotFound => continue,
            }
        }
        false
    }
    pub fn reset(&mut self) {
        for stop in &mut self.stops {
            stop.1 = 0;
        }
    }
}