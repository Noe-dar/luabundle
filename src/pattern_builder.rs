pub struct LuaPathBuilder {
    pattern: String,
}

impl LuaPathBuilder {
    pub fn new() -> Self {
        Self {
            pattern: Default::default(),
        }
    }

    pub fn add<S: AsRef<str>>(&mut self, pattern: S) {
        self.pattern.push_str(pattern.as_ref());
        self.pattern.push_str(";");
    }

    pub fn build(self) -> String {
        self.pattern
    }
}
