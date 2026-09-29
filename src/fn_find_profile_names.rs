use std::fs;
use std::io;

pub fn find_profile_names(path: &str) -> io::Result<Vec<String>> {
    let items = fs::read_dir(path)?;
    let mut profiles = vec![];
    for item in items {
        let item = match item.ok() {
            Some(v) => v,
            None => continue
        };
        let item_type = match item.file_type().ok() {
            Some(v) => v,
            None => continue
        };
        if !item_type.is_file() {
            continue;
        }
        let mut name = format!("{}", item.file_name().to_string_lossy());
        let ext = ".json";
        if !name.to_lowercase().ends_with(ext) {
            continue;
        }
        name.truncate(name.len() - ext.len());
        profiles.push(name);
    }
    Ok(profiles)
}