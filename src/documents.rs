use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs;
use std::fs::DirEntry;
use rocket::serde::{Serialize, Deserialize};

use crate::utils;
use crate::language;
use crate::transformer;
use crate::cryptography;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LawIndex {
  pais: String,
  instrumento: String,
  titulo: u16,
  capitulo: u16,
  articulo: u16,
  paragrafo: u16
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DictumDocumentReference {
  // dtype: String
  pub dindex: LawIndex,
  pub dref: String
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DictumDocument {
  pub reference: DictumDocumentReference,
  pub phrase: language::Phrase,
  pub encoding: Vec<f32>
}

lazy_static! {
  static ref DOCUMENTS_MEMORY: Mutex<HashMap<String,DictumDocument>> = Mutex::new(
    HashMap::new()
  );
}

pub fn read_reference_file(filepath: &DirEntry) -> (String, String, String, u16, u16, u16, u16) {
  let filecontents = utils::read_config_file(filepath.path().as_os_str().to_str().unwrap());
  let dref = filecontents["dref"].clone();
  let pais = filecontents["pais"].clone();
  let instrumento = filecontents["instrumento"].clone();
  let titulo = filecontents["titulo"].parse::<u16>().unwrap();
  let capitulo = filecontents["capitulo"].parse::<u16>().unwrap();
  let articulo = filecontents["articulo"].parse::<u16>().unwrap();
  let paragrafo = filecontents["paragrafo"].parse::<u16>().unwrap();
  return (dref,pais,instrumento, titulo,capitulo, articulo,paragrafo);
}

pub fn name_from_dir_entry(filepath: &DirEntry) -> String {
  return filepath.file_name().to_str().unwrap().to_string();
}

pub fn document_reference_filename(reference: &DictumDocumentReference) -> String {
  format!(
r#"{}.{}.titulo-{}.capitulo-{}.articulo-{}.paragrafo-{}"#,
    reference.dindex.pais,
    reference.dindex.instrumento,
    reference.dindex.titulo,
    reference.dindex.capitulo,
    reference.dindex.articulo,
    reference.dindex.paragrafo)
}

pub fn document_reference_to_string(reference: &DictumDocumentReference) -> String {
  format!(
r#"dref = "{}"
pais = "{}"
instrumento = "{}"
titulo = "{}"
capitulo = "{}"
articulo = "{}"
paragrafo = "{}""#,
    reference.dref,
    reference.dindex.pais,
    reference.dindex.instrumento,
    reference.dindex.titulo,
    reference.dindex.capitulo,
    reference.dindex.articulo,
    reference.dindex.paragrafo)
}

pub fn load_documents_memory(force_load: bool) {
  for dpath in fs::read_dir(utils::config_reference_folder()).expect("Documents folder not found") {
    let filename = name_from_dir_entry(&dpath.as_ref().unwrap());
    if filename.ends_with(&utils::config_reference_extension()) {
      let (dref,pais,instrumento, titulo,capitulo, articulo,paragrafo) = read_reference_file(&dpath.as_ref().unwrap());
      if force_load || !(DOCUMENTS_MEMORY.lock().unwrap().contains_key(&dref)) {
        let document_dpath = format!("{}{}{}",utils::config_documents_folder(),dref.as_str(),utils::config_documents_extension());
        let encoding_dpath = format!("{}{}{}",utils::config_encodings_folder(),dref.as_str(),utils::config_encodings_extension());
        let text = fs::read_to_string(&document_dpath)
          .expect(format!("{} : {}",utils::error_message("E0002").as_str(),&document_dpath).as_str());
        let encoding = utils::lines_from_file(&encoding_dpath)
          .expect(format!("{} : {}",utils::error_message("E0003").as_str(),&encoding_dpath).as_str())
          .iter().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
        DOCUMENTS_MEMORY.lock().unwrap().insert(
          dref,
          document_fabric(
            pais,
            instrumento,
            titulo,
            capitulo,
            articulo,
            paragrafo,
            text,
            encoding)
        );
      }
    }
  }
}

pub fn save_document(doc: &DictumDocument) {
  fs::write(format!("{}{}{}",utils::config_reference_folder(),&document_reference_filename(&doc.reference),utils::config_reference_extension()),document_reference_to_string(&doc.reference))
    .expect(format!("{}{}{}",utils::config_reference_folder(),utils::error_message("E0008"),&doc.reference.dref).as_str());
  fs::write(format!("{}{}{}",utils::config_documents_folder(),&doc.reference.dref,utils::config_documents_extension()),&doc.phrase.text)
    .expect(format!("{}{}{}",utils::config_documents_folder(),utils::error_message("E0004"),&doc.reference.dref).as_str());
  fs::write(format!("{}{}{}",utils::config_encodings_folder(),&doc.reference.dref,utils::config_encodings_extension()),&doc.encoding
    .iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"))
    .expect(format!("{}{}{}",utils::config_encodings_folder(),utils::error_message("E0005"),&doc.reference.dref).as_str());
}

pub fn reference_fabric(
  pais:String, instrumento: String, 
  titulo: u16, capitulo: u16, articulo: u16,
  paragrafo: u16, text: String) -> DictumDocumentReference {
  return DictumDocumentReference {
    dindex: LawIndex {
      pais:pais,
      instrumento:instrumento.to_lowercase(),
      titulo:titulo,
      capitulo:capitulo,
      articulo:articulo,
      paragrafo:paragrafo
    },
    dref: cryptography::sha256_digest(format!("{}.{}.{}.{}.{}={}",
      instrumento.to_lowercase(),
      titulo,
      capitulo,
      articulo,
      paragrafo,
      text)),
  }
}

pub fn document_fabric(
  pais: String, instrumento: String, titulo: u16,
  capitulo: u16, articulo: u16,
  paragrafo: u16, text: String, encoding: Vec<f32>) -> DictumDocument {
  return DictumDocument {
    reference: reference_fabric(
      pais.clone(),
      instrumento.clone(), 
      titulo.clone(),
      capitulo.clone(), 
      articulo.clone(),
      paragrafo.clone(), 
      text.clone()), 
    phrase: language::phrase_fabric(text.clone()), 
    encoding: if encoding.is_empty() {transformer::transform_sentence(text.clone())} else {encoding.clone()}
  }
}