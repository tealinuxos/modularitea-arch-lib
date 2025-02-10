use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub package: String,
    pub description: String,
    pub command: String,
}

impl Program {
    pub fn vsc() -> String {
        let vsc = Program {
            package: "Codium IDE".to_string(),
            description: "Text Editor / IDE with Open Source License without proprietary tracking"
                .to_string(),
            command: "sudo pacman -S code".to_string(),
        };

        serde_json::to_string_pretty(&vsc).unwrap()
    }

    pub fn laravel() -> String {
        let lambo = Program {
            package: "Laravel".to_string(),
            description: "Fullstack framework with powered by PHP programming language".to_string(),
            command: "/bin/bash -c '$(curl -fsSL https://php.new/install/linux/8.4)'".to_string(),
        };

        serde_json::to_string_pretty(&lambo).unwrap()
    }

    pub fn nodejs() -> String {
        let node = Program {
            package: "Node.js".to_string(),
            description: "JavaScript runtime built on Chrome's V8 engine".to_string(),
            command: "sudo pacman -S node npm".to_string(),
        };

        serde_json::to_string_pretty(&node).unwrap()
    }

    pub fn list() -> Vec<String> {
        let lists: Vec<String> = vec![Program::vsc(), Program::laravel(), Program::nodejs()];
        lists
    }
}
