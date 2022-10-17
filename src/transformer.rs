use lazy_static::lazy_static;
use std::sync::Mutex;
use rocket::serde::{Serialize, Deserialize};

use crate::mathematics;
use crate::utils;
use crate::language;

use rust_bert::pipelines::sentence_embeddings::{
  SentenceEmbeddingsModel,
  SentenceEmbeddingsBuilder, 
  // SentenceEmbeddingsModelType,
};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum EmbeddingType {
  Total,
  Average
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Embedding {
  pub etype: EmbeddingType,
  pub vector: Option<Vec<f32>>
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Meaning {
  pub phrase: language::Phrase,
  pub embedding: Embedding
}

lazy_static! {
  static ref TRANSFORMER: Mutex<SentenceEmbeddingsModel> = Mutex::new(
    SentenceEmbeddingsBuilder::local(
      utils::config_model_path()
  ).with_device(tch::Device::cuda_if_available()).create_model().expect(utils::error_message("E0006").as_str()));
}
pub fn embeddings_vectors_distance(vec_a: &Vec<f32>, vec_b: &Vec<f32>) -> f32 {
  mathematics::vector_euclidean_distance::<f32>(vec_a, vec_b)
}
pub fn transform_sentences(sentences: &Vec<String>) -> Vec<Vec<f32>> {
  // Generate Embeddings
  TRANSFORMER.lock().unwrap().encode(sentences).expect(utils::error_message("E0007").as_str())
    .iter().map(|x| mathematics::vec1d_normalize_mu2::<f32>(&x.to_vec())).collect()
}

// Transforms a sentence
// sentence length cannot be more than 512 words
pub fn transform_sentence(sentence: &String) -> Vec<f32> {
  transform_sentences(&Vec::from([sentence.clone()])).get(0).unwrap().to_vec()
}

// Transforms a Phrases
// Requires a sentence, of any length
pub fn transform_phrases(phrases_of_law: &Vec<language::Phrase>) -> Vec<Option<Vec<f32>>> {
  let mut ret:Vec<Option<Vec<f32>>>  = Vec::new();
  for phrase in phrases_of_law {
    ret.push(transform_phrase(phrase));
  }
  return ret;
}

// Transforms a Phrase
// Requires a sentence, of any length
pub fn transform_phrase(phrase_of_law: &language::Phrase) -> Option<Vec<f32>> {
  let segments = language::segment_phrase(phrase_of_law);
  if segments.is_empty() {return None;}
  let texts: Vec<String> = segments.iter().map(|x| x.text.clone()).collect::<Vec<String>>();
  let encds = transform_sentences(&texts);
  return Some(mathematics::vec2d_axis_average::<f32>(&encds,0));
}

pub fn embeddings_entropy(embeddings: &Vec<Vec<f32>>) -> Vec<f32> {
  return embeddings.iter().map(|v| 
    mathematics::vec1d_normalize_binary_entropy(&v.iter().map(|x| x.abs())
    .collect::<Vec<f32>>())).collect::<Vec<f32>>();
  // let negative_entropy = embeddings.iter().map(|v| 
  //   v.iter().filter(|&&vsplit| vsplit<0.0f32).collect::<Vec<&f32>>()).collect::<Vec<Vec<&f32>>>()
  //   .iter().map(|x| mathematics::vec1d_normalize_binary_entropy::<f32>(&x.iter().map(|x| (-1.0f32)*(**x)).collect::<Vec<f32>>())).collect::<Vec<f32>>();
  // let positive_entropy = embeddings.iter().map(|v| 
  //   v.iter().filter(|&&vsplit| vsplit>=0.0f32).collect::<Vec<&f32>>()).collect::<Vec<Vec<&f32>>>()
  //   .iter().map(|x| mathematics::vec1d_normalize_binary_entropy::<f32>(&x.iter().map(|x| **x).collect::<Vec<f32>>())).collect::<Vec<f32>>();
  // println!("negative_entropy: {:?}",negative_entropy);
  // println!("positive_entropy: {:?}",positive_entropy);
  // (negative_entropy,positive_entropy)
}

pub fn meaning_fabric(phrase_of_law: &language::Phrase, dembedding: &Option<Vec<f32>>, etype: EmbeddingType) -> Meaning {
  Meaning {
    phrase: phrase_of_law.clone(),
    embedding: Embedding {
      etype: etype, 
      vector: if dembedding.is_none() { transform_phrase(&phrase_of_law.clone()) } else { dembedding.clone() }
    }
  }
}