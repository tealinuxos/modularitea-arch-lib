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

    pub fn krita() -> String {
        let value = Design {
            package: "Krita".to_string(),
            description: "Professional free and open source painting program".to_string(),
            command: "krita".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn figma() -> String {
        let value = Design {
            package: "Figma".to_string(),
            description: "The collaborative interface design tool".to_string(),
            command: "figma-linux-bin".to_string(),
            source: Source::AUR,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn freecad() -> String {
        let value = Design {
            package: "FreeCAD".to_string(),
            description: "Feature based parametric 3D CAD modeler".to_string(),
            command: "freecad".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn pinta() -> String {
        let value = Design {
            package: "Pinta".to_string(),
            description: "Drawing/editing program modeled after Paint.NET".to_string(),
            command: "pinta".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn inkscape() -> String {
        let value = Design {
            package: "Inkscape".to_string(),
            description: "Professional vector graphics editor".to_string(),
            command: "inkscape".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn kdenlive() -> String {
        let value = Design {
            package: "Kdenlive".to_string(),
            description: "A non-linear video editor for Linux using the MLT video framework".to_string(),
            command: "kdenlive".to_string(),
            source: Source::Official,
        };

        serde_json::to_string_pretty(&value).unwrap()
    }

    pub fn list() -> Vec<String> {
        vec![
            Design::blender(),
            Design::gimp(),
            Design::openshot(),
            Design::krita(),
            Design::figma(),
            Design::freecad(),
            Design::pinta(),
            Design::inkscape(),
            Design::kdenlive(),
        ]
    }
}
