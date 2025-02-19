use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub package: String,
    pub description: String,
    pub command: String,
}

impl Network {
    pub fn docker() -> String {
        let docker = Network {
            package: "Docker".to_string(),
            description: "Container service including docker & docker-compose".to_string(),
            command: "docker docker-compose".to_string(),
        };

        serde_json::to_string_pretty(&docker).unwrap()
    }

    pub fn burpsuite() -> String {
        let burp = Network {
            package: "Burp Suite".to_string(),
            description: "Making sure of security layer web application".to_string(),
            command: "burpsuite".to_string(),
        };

        serde_json::to_string_pretty(&burp).unwrap()
    }

    pub fn postman() -> String {
        let post = Network {
            package: "Postman".to_string(),
            description: "Tool for testing all request and response API".to_string(),
            command: "postman".to_string(),
        };

        serde_json::to_string_pretty(&post).unwrap()
    }

    pub fn list() -> Vec<String> {
        let lists: Vec<String> = vec![Network::docker(), Network::burpsuite(), Network::postman()];

        lists
    }
}
