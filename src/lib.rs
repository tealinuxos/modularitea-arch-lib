pub mod design;
pub mod network;
pub mod program;

#[cfg(test)]
mod module_design {

    use crate::design::Design;

    #[test]
    fn blender() {
        let blender: Design = serde_json::from_str(&Design::blender()).unwrap();

        assert_eq!(String::from("Blender"), blender.package);
    }

    #[test]
    fn gimp() {
        let gimp: Design = serde_json::from_str(&Design::gimp()).unwrap();

        assert_eq!(
            String::from("GIMP (GNU Image Manipulation Program)"),
            gimp.package
        );
    }

    #[test]
    fn openshot() {
        let openshot: Design = serde_json::from_str(&Design::openshot()).unwrap();

        assert_eq!(String::from("OpenShot"), openshot.package);
    }
}
