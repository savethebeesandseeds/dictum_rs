use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::utils;

use rust_bert::pipelines::sentence_embeddings::{
  SentenceEmbeddingsModel,
  SentenceEmbeddingsBuilder, 
  // SentenceEmbeddingsModelType,
};


lazy_static! {
  static ref TRANSFORMER: Mutex<SentenceEmbeddingsModel> = Mutex::new(
    SentenceEmbeddingsBuilder::local(
      utils::config_model_path()
  ).with_device(tch::Device::cuda_if_available()).create_model().expect(utils::error_message("E0006").as_str()));
}

pub fn transform_sentences(sentences: &Vec<String>) -> Vec<Vec<f32>> {
  // Generate Embeddings
  TRANSFORMER.lock().unwrap().encode(sentences).expect(utils::error_message("E0007").as_str())
}

pub fn transform_sentence(sentence: String) -> Vec<f32> {
  transform_sentences(&Vec::from([sentence])).get(0).unwrap().to_vec()
}