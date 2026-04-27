use log::info;

use crate::full_indexing::chunk_extractor::Language;


mod file_handler;
mod chunk_extractor;

pub fn start_indexing(src_dir: &str) -> Result<(), std::io::Error> {
    let data = file_handler::read_file(src_dir);
    if let Ok(files) = data {
        files
            .flat_map(|file_data| chunk_extractor::extract_chunks(&file_data.content, Language::from_extension(&file_data.extension).unwrap_or(Language::JavaScript)))
            .map(|chunk| chunk.trim().to_string())
            .filter(|chunk| chunk.len() > 50) // drop noise
            .map(|chunk| chunk.replace("\n", " "))
            // .map(|chunk| /*  Embedding*/)
            .for_each(|embedding| {
                // store embedding
                info!("Indexed chunk: {}", embedding);
            });

    }
    Ok(())
}