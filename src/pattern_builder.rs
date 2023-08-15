#[derive(Clone, Debug, Default)]
pub struct LuaPathBuilder {
    pattern: String,
}

impl LuaPathBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<S: AsRef<str>>(&mut self, pattern: S) {
        self.pattern.push_str(pattern.as_ref());
        self.pattern.push(';');
    }

    pub fn build(self) -> String {
        self.pattern
    }
}
