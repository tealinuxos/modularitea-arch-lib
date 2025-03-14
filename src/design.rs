use serde::{Deserialize, Serialize};

use crate::skeleton::Source;

#[derive(Debug, Serialize, Deserialize)]
pub struct Design {
    pub package: String,
    pub description: String,
    pub command: String,
    source: Source,
}

impl Design {
    pub fn blender() -> String {
        let blender = Design {
            package: "Blender".to_string(),
            description: "Make a design with 3D designing".to_string(),
            command: "blender".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&blender).unwrap()
    }

    pub fn gimp() -> String {
        let gimp = Design {
            package: "GIMP (GNU Image Manipulation Program)".to_string(),
            description: "Editing image kinde like Adobe Photosop".to_string(),
            command: "gimp".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&gimp).unwrap()
    }

    pub fn openshot() -> String {
        let openshot = Design {
            package: "OpenShot".to_string(),
            description: "An award-wining open source video editor".to_string(),
            command: "openshot".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&openshot).unwrap()
    }

    pub fn list() -> Vec<String> {
        let lists: Vec<String> = vec![Design::blender(), Design::gimp(), Design::openshot()];

        lists
    }
}
