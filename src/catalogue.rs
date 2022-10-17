use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs::create_dir_all;
use rocket::serde::{Serialize, Deserialize};
use walkdir::WalkDir;

use crate::transformer::EmbeddingType;
use crate::utils;
use crate::language;
use crate::transformer;
use crate::mathematics;
use crate::files;
use crate::laws;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Catalogue {
  pub dindex: laws::LawIndex,
  pub dmeaning: transformer::Meaning
}

lazy_static! {
  static ref CATALOGUES_MEMORY: Mutex<HashMap<laws::LawIndex,Catalogue>> = Mutex::new(
    HashMap::new()
  );
}

pub fn consult_catalogues_memory(dindex: &laws::LawIndex) -> Catalogue {
  if !(CATALOGUES_MEMORY.lock().unwrap().contains_key(dindex)) {
    load_catalogues_memory(false);
  }
  CATALOGUES_MEMORY.lock().unwrap().get(dindex)
    .expect(utils::error_message("E0009").as_str()).to_owned()
}

pub fn compare_embedding_against_law_book(embedding: &transformer::Embedding, book: &laws::LawBook) -> Vec<(laws::LawIndex,f32)>{
  let mut aux = CATALOGUES_MEMORY.lock().unwrap()
    .iter().filter(|(dindex,dcatalogue)| 
      dindex.book==*book)
      .collect::<HashMap<&laws::LawIndex,&Catalogue>>()
    .iter().map(|(&dindex,dcatalogue)| 
      (dindex.clone(),transformer::embeddings_vectors_distance(&embedding.vector.clone().unwrap(), &dcatalogue.dmeaning.embedding.vector.clone().unwrap())))
      .collect::<Vec<(laws::LawIndex,f32)>>();
  aux.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
  return aux[0..utils::config_return_count()].to_vec();
}

pub fn load_catalogues_memory_item(law_index: &laws::LawIndex, etype: transformer::EmbeddingType) {
  let phrase_of_law = files::read_phrase_of_law(law_index);
  let embedding = &Some(utils::lines_from_file(&files::embeddings_filename(&law_index))
    .expect(format!("{} : {}",utils::error_message("E0003").as_str(),&files::embeddings_filename(&law_index)).as_str())
    .iter().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>());
  CATALOGUES_MEMORY.lock().unwrap().insert(
    law_index.clone(),
    catalogue_fabric(
      law_index.book.pais.clone(),
      law_index.book.instrumento.clone(),
      law_index.titulo,
      law_index.capitulo,
      law_index.articulo,
      law_index.parte,
      &language::phrase_fabric(phrase_of_law),
      etype,
      embedding
    )
  );
}
pub fn load_catalogues_memory(force_load: bool) {
  // for dpath in fs::read_dir(utils::config_reference_folder()).expect("Catalogues folder not found") {
  for dpath in WalkDir::new(utils::config_reference_folder()).into_iter().filter_map(|e| e.ok()) {
    let filename = utils::name_from_dir_entry(&dpath);
    if !(filename.ends_with(&utils::config_reference_extension())) {
      continue;
    }
    let (law_index, etype) = files::read_reference_file(&dpath);
    if !(force_load || !(CATALOGUES_MEMORY.lock().unwrap().contains_key(&law_index))) {
      continue;
    }
    println!("Loading file to CATALOGUES_MEMORY: [{}]",filename);
    load_catalogues_memory_item(&law_index, etype);
  }
}

pub fn save_catalogue(doc: &Catalogue) {
  create_dir_all(files::reference_foldername(&doc.dindex)).unwrap();
  create_dir_all(files::embeddings_foldername(&doc.dindex)).unwrap();
  files::write_reference_file(doc);
  files::write_embeddings_file(doc);
}

pub fn catalogue_fabric(
  pais: String, instrumento: String, titulo: Option<u16>,
  capitulo: Option<u16>, articulo: Option<u16>,
  parte: Option<u16>, phrase_of_law: &language::Phrase, 
  etype: transformer::EmbeddingType, embedding: &Option<Vec<f32>>) -> Catalogue {
  return Catalogue {
    dindex: laws::LawIndex {
      book:laws::LawBook {
        pais:pais.to_lowercase(),
        instrumento:instrumento.to_lowercase(),
      },
      titulo:titulo,
      capitulo:capitulo,
      articulo:articulo,
      parte:parte
    },
    dmeaning: transformer::meaning_fabric(
      phrase_of_law,
      embedding,
      etype
    )
  }
}

pub fn embedd_sentence(phrase_of_law: &language::Phrase, law_index: &laws::LawIndex) -> (Option<Vec<f32>>, EmbeddingType) {
  let mut embedding: Option<Vec<f32>>= None;
  let segments = language::segment_phrase_with_index(phrase_of_law, law_index);
  let mut etype = transformer::EmbeddingType::Total;
  if !segments.is_empty() {
    let texts: Vec<String> = segments.iter().map(|x| x.1.text.clone()).collect::<Vec<String>>();
    let encds = transformer::transform_sentences(&texts);
    let encds_arr = mathematics::vec2d_axis_average::<f32>(&encds,0);
    embedding = Some(encds_arr);
    if segments.len() != 1 {
      etype = transformer::EmbeddingType::Average;
    }
  } else {
    if !phrase_of_law.text.is_empty() {
      println!("[Warning]: catalogue_mech, phrase_of_law is found too short : <{}>",phrase_of_law.text);
    }
  }
  return (embedding,etype);
}
// Generate catalogue for phrase of law
pub fn catalogue_mech(phrase_of_law: &language::Phrase, law_index: &laws::LawIndex) {
  let (embd, etype) = embedd_sentence(phrase_of_law, law_index);
  if embd.is_some() {
    // Save catalgue
    save_catalogue(&catalogue_fabric(
      law_index.book.pais.clone().to_lowercase(), 
      law_index.book.instrumento.clone().to_lowercase(), 
      law_index.titulo.clone(),
      law_index.capitulo.clone(), 
      law_index.articulo.clone(),
      None,
      &phrase_of_law.clone(), 
      etype.clone(),
      &embd));
    // Save document of law
    files::write_file_of_law(&phrase_of_law.clone(), law_index);
    // Load catalogue
    load_catalogues_memory_item(law_index, etype.clone());
  }
}