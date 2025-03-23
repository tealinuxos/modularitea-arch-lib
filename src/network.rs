use serde::{Deserialize, Serialize};

use crate::skeleton::Source;

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub package: String,
    pub description: String,
    pub command: String,
    source: Source,
}

impl Network {
    pub fn docker() -> String {
        let docker = Network {
            package: "Docker".to_string(),
            description: "Container service including docker & docker-compose".to_string(),
            command: "docker docker-compose".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&docker).unwrap()
    }

    pub fn burpsuite() -> String {
        let burp = Network {
            package: "Burp Suite".to_string(),
            description: "Making sure of security layer web application".to_string(),
            command: "burpsuite".to_string(),
            source: Source::AUR,
        };

        serde_json::to_string_pretty(&burp).unwrap()
    }

    pub fn postman() -> String {
        let post = Network {
            package: "Postman".to_string(),
            description: "Tool for testing all request and response API".to_string(),
            command: "postman".to_string(),
            source: Source::AUR,
        };

        serde_json::to_string_pretty(&post).unwrap()
    }

    pub fn jd() -> String {
        let value = Network {
            package: "Java Decompiler".to_string(),
            description: "A standalone graphical utility that displays Java source codes of .class files".to_string(),
            command: "jd-gui".to_string(),
            source: Source::AUR,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn wireshark() -> String {
        let value = Network {
            package: "Wireshark".to_string(),
            description: "Network traffic and protocol analyzer/sniffer".to_string(),
            command: "wireshark-qt".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn jadx() -> String {
        let value = Network {
            package: "Dex to Java decompiler ".to_string(),
            description: "Command line and GUI tools to produce Java source code from Android Dex and APK files".to_string(),
            command: "jadx".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn metasploit() -> String {
        let value = Network {
            package: "Metasploit ".to_string(),
            description: "Advanced open-source platform for developing, testing, and using exploit code".to_string(),
            command: "metasploit".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn ghidra() -> String {
        let value = Network {
            package: "Ghidra ".to_string(),
            description: "Software reverse engineering framework".to_string(),
            command: "ghidra".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn havoc() -> String {
        let value = Network {
            package: "Havoc ".to_string(),
            description: "Modern and malleable post-exploitation command and control framework".to_string(),
            command: "havoc-c2-git".to_string(),
            source: Source::AUR,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn list() -> Vec<String> {
        vec![
            Network::docker(),
            Network::burpsuite(),
            Network::postman(),
            Network::jd(),
            Network::wireshark(),
            Network::jadx(),
            Network::metasploit(),
            Network::ghidra(),
            Network::havoc(),
        ]
    }
}
