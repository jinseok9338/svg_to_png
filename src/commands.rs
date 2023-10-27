pub struct Commands {
    pub commands: Vec<String>,
}

impl Commands {
    pub fn new() -> Self {
        Commands {
            commands: Vec::new(),
        }
    }

    pub fn default() -> Self {
        Commands {
            commands: vec!["ls".to_string(), "pwd".to_string()],
        }
    }

    pub fn add(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn run(&self) {
        for command in &self.commands {
            println!("Running command: {}", command);
        }
    }
}
