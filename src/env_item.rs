
struct EnvItem {
    pub path: String,
    pub value: String,
}

impl EnvItem {
    pub fn new<S: Into<String>>(path: S, value: S) -> EnvItem {
        EnvItem { path: path.into(), value: value.into() }
    }
}

