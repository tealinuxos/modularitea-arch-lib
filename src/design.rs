use serde::Serialize;

#[derive(Serialize)]
pub struct Design {
    package: String,
    description: String,
    command: String,
}

impl Design {
    pub fn blender() -> String {
        let blender = Design {
            package: "Blender".to_string(),
            description: "Make a design with 3D designing".to_string(),
            command: "sudo pacman -S blender".to_string(),
        };

        serde_json::to_string_pretty(&blender).unwrap()
    }

    pub fn gimp() -> String {
        let gimp = Design {
            package: "GIMP (GNU Image Manipulation Program)".to_string(),
            description: "Editing image kinde like Adobe Photosop".to_string(),
            command: "sudo pacman -S gimp".to_string(),
        };

        serde_json::to_string_pretty(&gimp).unwrap()
    }

    pub fn list() -> Vec<String> {
        let mut lists = Vec::new();

        lists.push(Design::blender());
        lists.push(Design::gimp());

        lists
    }
}
