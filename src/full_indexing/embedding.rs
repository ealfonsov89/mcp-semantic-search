use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub fn init_embedding_model() -> Result<TextEmbedding, fastembed::Error> {
    TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
    )
}

pub fn create_embedding(model: &mut TextEmbedding, text: Vec<String>) -> Result<Vec<Vec<f32>>, fastembed::Error> {
    model.embed(text, None)
}
