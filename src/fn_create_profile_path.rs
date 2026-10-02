pub fn create_profile_path(root: &str, name: &str) -> String {
    if root.ends_with("/") {
        format!("{}{}.json", root, name)
    }
    else {
        format!("{}/{}.json", root, name)
    }
}