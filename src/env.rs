
use std::path::PathBuf;

struct Env {
    items: Vec<EnvItem>,
}

impl Env {
    pub fn new() -> Env {
    }

    pub fn new_minimal() -> Env {
        let env = Env::new();
        env.add_item("os.kernel", "linux");
        env.add_item("vars.SSL_CERT_FILE", "true");
        env.add_item("vars.TERM", "true");
        env.add_item("vars.HOME", "true");
    }

    pub fn add_item(&mut self, item: EnvItem) {
        self.items.push(option)
    }

    pub fn run(&mut self, path: PathBuf) -> VicResult<()> {
        for item in self.items {
            //match item.path {
                //""
            //}
        }
    }
}
