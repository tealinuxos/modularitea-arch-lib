pub mod design;
pub mod network;
pub mod program;

#[cfg(test)]
mod module_design {

    use crate::design::Design;
    use serde_json::Value;

    #[test]
    fn blender() {
        let blender = Design::blender();
        let json: Value = serde_json::from_str(&blender).unwrap();
        let blender_json = json["package"].as_str().unwrap();

        assert_eq!(String::from("Blender"), blender_json);
    }

    #[test]
    fn gimp() {
        let gimp = Design::gimp();
        let json: Value = serde_json::from_str(&gimp).unwrap();
        let gimp_json = json["package"].as_str().unwrap();

        assert_eq!(
            String::from("GIMP (GNU Image Manipulation Program)"),
            gimp_json
        );
    }

    #[test]
    fn openshot() {
        let openshot = Design::openshot();
        let json: Value = serde_json::from_str(&openshot).unwrap();
        let openshot_json = json["package"].as_str().unwrap();

        assert_eq!(String::from("OpenShot"), openshot_json);
    }
}
