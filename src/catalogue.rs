use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs;
use std::fs::DirEntry;
use rocket::serde::{Serialize, Deserialize};
use std::cmp::Eq;

use crate::utils;
use crate::language;
use crate::transformer;
use crate::cryptography;
use crate::laws;

#[derive(Clone)]
#[derive(Hash,Eq,PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TsahduCatalogueReference {
  pub dindex: laws::LawIndex,
  pub dref: String
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TsahduCatalogue {
  pub reference: TsahduCatalogueReference,
  pub phrase: language::Phrase,
  pub encoding: Vec<f32>
}

lazy_static! {
  static ref CATALOGUES_MEMORY: Mutex<HashMap<String,TsahduCatalogue>> = Mutex::new(
    HashMap::new()
  );
}

pub fn name_from_dir_entry(filepath: &DirEntry) -> String {
  return filepath.file_name().to_str().unwrap().to_string();
}

pub fn read_reference_file(filepath: &DirEntry) -> (String, String, String, u16, u16, u16, Option<u16>) {
  let filecontent = utils::read_config_file(filepath.path().as_os_str().to_str().unwrap());
  let dref = filecontent["dref"].clone();
  let pais = filecontent["pais"].clone();
  let instrumento = filecontent["instrumento"].clone();
  let titulo = filecontent["titulo"].parse::<u16>().unwrap();
  let capitulo = filecontent["capitulo"].parse::<u16>().unwrap();
  let articulo = filecontent["articulo"].parse::<u16>().unwrap();
  let parte = filecontent["parte"].parse::<i16>().unwrap();
  let parte: Option<u16> = if parte<0 { None } else { Some(parte.try_into().unwrap()) };
  return (dref,pais,instrumento, titulo,capitulo, articulo,parte);
}

pub fn catalogue_reference_filename(reference: &TsahduCatalogueReference) -> String {
  format!(
r#"{}.{}.titulo-{}.capitulo-{}.articulo-{}{}"#,
    reference.dindex.book.pais,
    reference.dindex.book.instrumento,
    reference.dindex.titulo,
    reference.dindex.capitulo,
    reference.dindex.articulo,
    if reference.dindex.parte.is_none() {"".to_string()}  else {format!(".parte-{}",reference.dindex.parte.unwrap()).to_string()})
}

pub fn catalogue_reference_to_string(reference: &TsahduCatalogueReference) -> String {
  format!(
r#"pais = "{}"
instrumento = "{}"
titulo = "{}"
capitulo = "{}"
articulo = "{}"
parte = "{}"
dref = "{}""#,
  reference.dindex.book.pais,
  reference.dindex.book.instrumento,
  reference.dindex.titulo,
  reference.dindex.capitulo,
  reference.dindex.articulo,
  if reference.dindex.parte.is_none() {"-1".to_string()}  else {format!("{}",reference.dindex.parte.unwrap()).to_string()},
  reference.dref)
}

pub fn consult_catalogues_memory(dref: &String) -> String {
  if !(CATALOGUES_MEMORY.lock().unwrap().contains_key(dref)) {
    return utils::error_message("E0009");
  }
  return CATALOGUES_MEMORY.lock().unwrap().get(dref).unwrap().phrase.text.clone();
}

pub fn load_catalogues_memory(force_load: bool) {
  for dpath in fs::read_dir(utils::config_reference_folder()).expect("Catalogues folder not found") {
    let filename = name_from_dir_entry(&dpath.as_ref().unwrap());
    println!("Loading file to CATALOGUES_MEMORY: [{}]",filename);
    if !(filename.ends_with(&utils::config_reference_extension())) {
      continue;
    }
    let (dref,pais,instrumento,titulo,capitulo,articulo,parte) = 
      read_reference_file(&dpath.as_ref().unwrap());
    if !(force_load || !(CATALOGUES_MEMORY.lock().unwrap().contains_key(&dref))) {
      continue;
    }
    let catalogue_dpath = 
      format!("{}{}{}",utils::config_catalogues_folder(),dref.as_str(),utils::config_catalogues_extension());
    let encoding_dpath = 
      format!("{}{}{}",utils::config_encodings_folder(),dref.as_str(),utils::config_encodings_extension());
    let text = fs::read_to_string(&catalogue_dpath)
      .expect(format!("{} : {}",utils::error_message("E0002").as_str(),&catalogue_dpath).as_str());
    let encoding = utils::lines_from_file(&encoding_dpath)
      .expect(format!("{} : {}",utils::error_message("E0003").as_str(),&encoding_dpath).as_str())
      .iter().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    CATALOGUES_MEMORY.lock().unwrap().insert(
      dref,
      catalogue_fabric(
        pais,
        instrumento,
        titulo,
        capitulo,
        articulo,
        parte,
        text,
        encoding)
    );
  }
}

pub fn save_catalogue(doc: &TsahduCatalogue) {
  fs::write(format!("{}{}{}",utils::config_reference_folder(),&catalogue_reference_filename(&doc.reference),utils::config_reference_extension()),catalogue_reference_to_string(&doc.reference))
    .expect(format!("{}{}{}",utils::config_reference_folder(),utils::error_message("E0008"),&doc.reference.dref).as_str());
  fs::write(format!("{}{}{}",utils::config_catalogues_folder(),&doc.reference.dref,utils::config_catalogues_extension()),&doc.phrase.text)
    .expect(format!("{}{}{}",utils::config_catalogues_folder(),utils::error_message("E0004"),&doc.reference.dref).as_str());
  fs::write(format!("{}{}{}",utils::config_encodings_folder(),&doc.reference.dref,utils::config_encodings_extension()),&doc.encoding
    .iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"))
    .expect(format!("{}{}{}",utils::config_encodings_folder(),utils::error_message("E0005"),&doc.reference.dref).as_str());
}

pub fn reference_fabric(
  pais:String, instrumento: String, 
  titulo: u16, capitulo: u16, articulo: u16,
  parte: Option<u16>) -> TsahduCatalogueReference {
  return TsahduCatalogueReference {
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
    dref: cryptography::sha256_digest(format!("{}.{}.{}.{}.{}{}",
      pais.to_lowercase(),
      instrumento.to_lowercase(),
      titulo,
      capitulo,
      articulo,
      if parte.is_none() {"".to_string()}  else {format!(".{}",parte.unwrap()).to_string()})),
  }
}

pub fn catalogue_fabric(
  pais: String, instrumento: String, titulo: u16,
  capitulo: u16, articulo: u16,
  parte: Option<u16>, text: String, encoding: Vec<f32>) -> TsahduCatalogue {
  return TsahduCatalogue {
    reference: reference_fabric(
      pais.clone(),
      instrumento.clone(), 
      titulo.clone(),
      capitulo.clone(), 
      articulo.clone(),
      parte.clone()), 
    phrase: language::phrase_fabric(text.clone()), 
    encoding: if encoding.is_empty() {transformer::transform_sentence(text.clone())} else {encoding.clone()}
  }
}