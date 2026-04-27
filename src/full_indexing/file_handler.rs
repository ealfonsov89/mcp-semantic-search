pub struct FileData {
    pub content: String,
    pub extension: String,
    pub path: String,
}

pub(crate) fn read_file(src_dir: &str) -> Result<impl Iterator<Item = FileData>, std::io::Error> {

    let iter = std::fs::read_dir(src_dir)?
        .filter_map(|item| item.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| generate_file_data(entry))
        .filter_map(|file_data| std::fs::read_to_string(&file_data.path).map(|content| FileData { content, ..file_data }).ok());
    Ok(iter)
}

fn generate_file_data(entry: std::fs::DirEntry) -> FileData {
        let path = entry.path();
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();
        FileData {
            content: String::new(),
            extension,
            path: path.to_string_lossy().into_owned(),
        }
    }
