use serde::{Deserialize, Serialize};

use crate::skeleton::Source;

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub package: String,
    pub description: String,
    pub command: String,
    source: Source,
}

impl Program {
    pub fn vsc() -> String {
        let vsc = Program {
            package: "Visual Studio Code".to_string(),
            description: "Editor for building and debugging modern web and cloud applications"
                .to_string(),
            command: "visual-studio-code-bin".to_string(),
            source: Source::AUR,
        };

        serde_json::to_string_pretty(&vsc).unwrap()
    }

    pub fn arduino_ide() -> String {
        let value = Program {
            package: "Arduino IDE".to_string(),
            description: "Open-source electronics prototyping platform"
                .to_string(),
            command: "arduino-ide".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn netbeans() -> String {
        let value = Program {
            package: "Apache NetBeans".to_string(),
            description: "IDE for Java, HTML5, PHP, Groovy, C and C++"
                .to_string(),
            command: "netbeans".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn codeblock() -> String {
        let value = Program {
            package: "Code::Blocks".to_string(),
            description: "Cross-platform C/C++ IDE"
                .to_string(),
            command: "codeblocks".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn obsidian() -> String {
        let value = Program {
            package: "Obsidian".to_string(),
            description: "A powerful knowledge base that works on top of a local folder of plain text Markdown files"
                .to_string(),
            command: "obsidian".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn list() -> Vec<String> {
        vec![
            Program::vsc(),
            Program::arduino_ide(),
            Program::netbeans(),
            Program::codeblock(),
            Program::obsidian(),
        ]
    }
}
