//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Add,
    Delete,
    Show,
    All,
}

#[derive(Debug, Clone)]
pub struct ArpOptions {
    pub mode: Mode,
    pub interface: Option<String>,
}

impl Default for ArpOptions {
    fn default() -> Self { Self { mode: Mode::Default, interface: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(ArpOptions::default().mode, Mode::Default); }
}
