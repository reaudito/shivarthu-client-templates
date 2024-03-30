use std::fs;
use std::path::Path;
use std::io;

pub fn list_files_recursive(base_path: &Path, current_path: &Path) -> io::Result<Vec<(String, String, String)>> {
    let mut file_names = Vec::new();

    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let mut subdir_files = list_files_recursive(base_path, &path)?;
            file_names.append(&mut subdir_files);
        } else {
            let relative_path = path.strip_prefix(base_path)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .to_string_lossy()
                .into_owned();
            let absolute_path = path.to_string_lossy().into_owned();
            file_names.push((absolute_path.clone(), relative_path.clone(), path.file_name().unwrap().to_string_lossy().into_owned()));
        }
    }

    Ok(file_names)
}