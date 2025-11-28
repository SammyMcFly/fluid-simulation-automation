use std::path::{Path, PathBuf};



pub fn create_folder_or_error(path: &str) -> std::io::Result<()> {
    let folder = Path::new(path);

    if folder.exists() {
        // Folder already exists: return an error
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Folder '{}' already exists", path),
        ))
    } else {
        // Folder does not exist: create it
        std::fs::create_dir_all(folder)?;
        Ok(())
    }
}

pub fn add_file_name_to_folder(folder: &str, file: &str) -> String {
    let folder = Path::new(folder);
    let file_path = folder.join(file);
    file_path.to_string_lossy().into_owned()
}

fn add_suffix_to_file_name(original: &str, suffix: &str) -> std::path::PathBuf {
    let path = Path::new(original);

    // Split filename and extension
    let stem = path.file_stem().unwrap().to_string_lossy();
    let ext  = path.extension().unwrap_or_default().to_string_lossy();

    // Build new filename: name.suffix.ext
    let new_filename = format!("{}.{}.{}", stem, suffix, ext);

    // Return new path in the same directory
    path.with_file_name(new_filename)
}

pub fn get_temporary_config_file_path(config_file: &str) -> PathBuf {
    let extension = "temp".to_string();
    let temp_file_path = add_suffix_to_file_name(config_file, &extension);
    if temp_file_path.as_path().exists() {
        panic!("Parameter variation config file already exists!");
    }
    temp_file_path
}
