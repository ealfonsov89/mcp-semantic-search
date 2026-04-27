use log::info;

use crate::full_indexing::chunk_extractor::Language;
use itertools::Itertools;

mod chunk_extractor;
mod embedding;
mod file_handler;

pub fn start_indexing(src_dir: &str) -> Result<(), std::io::Error> {
    let data = file_handler::read_file(src_dir);
    if let Ok(files) = data
        && let Ok(mut model) = embedding::init_embedding_model()
    {
        files
            .flat_map(|file_data| {
                chunk_extractor::extract_chunks(
                    &file_data.content,
                    Language::from_extension(&file_data.extension).unwrap_or(Language::JavaScript),
                )
            })
            .map(|chunk| chunk.trim().to_string())
            .filter(|chunk| chunk.len() > 50) // drop noise
            .map(|chunk| chunk.replace("\n", " "))
            .chunks(30)
            .into_iter()
            .map(|batch| {
                let batch: Vec<String> = batch.collect();
                embedding::create_embedding(&mut model, batch)
            })
            .for_each(|embedding| {
                // store embedding
                match embedding {
                    Ok(embedding_vec) => info!("Generated embedding: {:?}", embedding_vec),
                    Err(err) => info!("Failed to generate embedding: {}", err),
                }
            });
            Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read files or initialize embedding model"))
    }
}
