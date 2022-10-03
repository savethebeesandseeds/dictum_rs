use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs;
use std::fs::create_dir_all;
use rocket::serde::{Serialize, Deserialize};
use std::cmp::Eq;
use regex::Regex;
use std::ops::Range;

use crate::utils;
use crate::catalogue;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Hash,Eq,PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LawBook {
  pub pais: String,
  pub instrumento: String
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Hash,Eq,PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LawIndex {
  pub book: LawBook,
  pub titulo: u16,
  pub capitulo: u16,
  pub articulo: u16,
  pub parte: Option<u16>
}
#[derive(Debug)]
pub enum LawMark {
  Capitulo,
  Titulo,
  Articulo
}
#[derive(Debug)]
pub enum TextOfLawValidation {
  Short,
  Long,
  Proper
}

lazy_static! {
  static ref LAWS_MEMORY: Mutex<HashMap<LawIndex,String>> = Mutex::new(
    HashMap::new()
  );
}

pub fn consult_laws_memory(reference: &LawIndex) -> String {
  if !(LAWS_MEMORY.lock().unwrap().contains_key(reference)) {
    return utils::error_message("E0010");
  }
  return LAWS_MEMORY.lock().unwrap().get(reference).unwrap().clone();
}

pub fn regex_interpret_law(regex_expresion: &str, text: &String) -> Vec<(u16,Range<usize>)> {
  Regex::new(regex_expresion).unwrap().find_iter(text.as_str())
    .filter_map(|x| Some((utils::atoi::<u16>(x.as_str()).unwrap(), x.range())))
    .collect::<Vec<(u16,Range<usize>)>>()
}
pub fn mark_interrupt_articulo(book: &LawBook, text: &String) -> Vec<(u16,Range<usize>)> {
  regex_interpret_law(utils::config_law(book).get("regex_articulo").unwrap(), text)
}
pub fn mark_interrupt_titulo(book: &LawBook, text: &String) -> Vec<(u16,Range<usize>)> {
  regex_interpret_law(utils::config_law(book).get("regex_titulo").unwrap(), text)
}
pub fn mark_interrupt_capitulo(book: &LawBook, text: &String) -> Vec<(u16,Range<usize>)> {
  regex_interpret_law(utils::config_law(book).get("regex_capitulo").unwrap(), text)
}
pub fn advance_mark(law_index: &mut LawIndex, mark: &(LawMark,u16,Range<usize>)) {
  match mark.0 {
    LawMark::Titulo   => {law_index.titulo   = mark.1;}
    LawMark::Capitulo => {law_index.capitulo = mark.1;}
    LawMark::Articulo => {law_index.articulo = mark.1;}
  }
}
pub fn validate_phrase_of_law(phrase_of_law: &String, book: &LawBook) -> TextOfLawValidation {
  if phrase_of_law.split(" ").collect::<Vec<&str>>().len() < utils::atoi::<usize>(utils::config_law(book).get("minimum_window_size").unwrap()).unwrap() {
    TextOfLawValidation::Short
  } else if phrase_of_law.split(" ").collect::<Vec<&str>>().len() > utils::atoi::<usize>(utils::config_law(book).get("maximum_window_size").unwrap()).unwrap() {
    TextOfLawValidation::Long
  } else {
    TextOfLawValidation::Proper
  }
}
pub fn clean_phrase_of_law(phrase_of_law: &String) -> String {
  return phrase_of_law.replace("\n"," ").replace("  "," ").trim().to_string();
}
pub fn segment_phrase_of_law(phrase_of_law: &String, index: &LawIndex) -> Vec<(LawIndex, String)> { 
  let mut ret : Vec<(LawIndex, String)> = Vec::new();
  let mut c_index = index.clone();
  
  match validate_phrase_of_law(phrase_of_law, &index.book) {
    TextOfLawValidation::Short => {}
    TextOfLawValidation::Proper => {
      c_index.parte = None;
      ret.push((c_index.clone(),phrase_of_law.to_string()));
    }
    TextOfLawValidation::Long => {
      let mut parte : Option<u16> = Some(0);
      for seg in utils::overlaping_chunks::<&str>(
        &phrase_of_law.split(" ").collect::<Vec<&str>>(), 
        utils::atoi::<usize>(utils::config_law(&index.book).get("maximum_window_size").unwrap()).unwrap(), 
        utils::atoi::<usize>(utils::config_law(&index.book).get("window_retrocede").unwrap()).unwrap())
        .iter().map(|x| x.join(" ")).collect::<Vec<String>>() {
        c_index.parte=parte;
        ret.push((c_index.clone(),seg));
        parte=Some(parte.unwrap()+1);
      }
    }
  }
  
  return ret;
}
// Read law file
pub fn read_law_book(book: &LawBook) -> String {
  let path_of_law = 
    &format!("{}{}.{}{}",
      utils::config_laws_folder(),
      book.pais,
      book.instrumento,
      utils::config_law_extension());
  fs::read_to_string(path_of_law)
    .expect(format!("{} : {}",utils::error_message("E0011").as_str(),path_of_law).as_str())
}
// Efective read of laws, returns markings of all aparitions of [Articulo, Titulo, Capitulo]
pub fn mark_text_of_law(text_of_law: &String, book: &LawBook) -> Vec<(LawMark, u16, Range<usize>)> {
  let mut marks = mark_interrupt_articulo(book, text_of_law)
    .iter().map(|x| (LawMark::Articulo,x.0,x.1.clone())).collect::<Vec<(LawMark,u16,Range<usize>)>>();
  marks.append(mark_interrupt_titulo(book, text_of_law)
    .iter().map(|x| (LawMark::Titulo,x.0,x.1.clone())).collect::<Vec<(LawMark,u16,Range<usize>)>>().as_mut());
  marks.append(mark_interrupt_capitulo(book, text_of_law)
    .iter().map(|x| (LawMark::Capitulo,x.0,x.1.clone())).collect::<Vec<(LawMark,u16,Range<usize>)>>().as_mut());
  marks.sort_by(|a,b| a.2.end.partial_cmp(&b.2.end).unwrap());
  return marks;
}
pub fn law_mech(phrase_of_law: &String, law_index: &LawIndex) {
  let segments = segment_phrase_of_law(phrase_of_law, law_index);
  if !segments.is_empty() {
    // Save catalgues
    for seg in segments {
      catalogue::save_catalogue(&catalogue::catalogue_fabric(
        seg.0.book.pais.clone().to_lowercase(), 
        seg.0.book.instrumento.clone().to_lowercase(), 
        seg.0.titulo.clone(),
        seg.0.capitulo.clone(), 
        seg.0.articulo.clone(),
        seg.0.parte.clone(), 
        seg.1.clone(), 
        Vec::new()));
    }
    // Save document of law
    save_file_of_law(&phrase_of_law.clone(), law_index);
  }
}
// Given a catalogue of Law this function reads, interprests and dumps a TsahduCatalogue
pub fn interpret_law(book: &LawBook) {
  let current_law_index =  &mut LawIndex {
    book:book.clone(),
    titulo:0,
    capitulo:0,
    articulo:0,
    parte:None
  };
  let text_of_law = &read_law_book(book);
  let marks = mark_text_of_law(text_of_law, book);
  for mark in marks.windows(2) {
    advance_mark(current_law_index, mark.get(0).unwrap());
    let phrase_of_law = &clean_phrase_of_law(
      &utils::substring(text_of_law, 
        mark.get(0).unwrap().2.end, 
        mark.get(1).unwrap().2.start));
    law_mech(phrase_of_law, &current_law_index);
  }
  advance_mark(current_law_index, marks.last().unwrap());
  let phrase_of_law = &clean_phrase_of_law(&
    utils::substring(text_of_law, 
      marks.last().unwrap().2.end, 
      text_of_law.len()));
  law_mech(phrase_of_law, &current_law_index);
}

pub fn law_index_filename(index: &LawIndex) -> String {
  format!(
r#"{}.{}.titulo-{}.capitulo-{}.articulo-{}{}"#,
    index.book.pais,
    index.book.instrumento,
    index.titulo,
    index.capitulo,
    index.articulo,
    if index.parte.is_none() {"".to_string()}  else {format!(".parte-{}",index.parte.unwrap()).to_string()})
}

pub fn save_file_of_law(phrase_of_law: &String, index: &LawIndex) {
  let folder_name = format!("{}{}.{}/",
    utils::config_laws_folder(),
    index.book.pais,
    index.book.instrumento);
  let file_name = format!("{}{}{}",
    folder_name,
    law_index_filename(index),
    utils::config_law_extension()
  );
  create_dir_all(folder_name).unwrap();
  fs::write(file_name.clone(), phrase_of_law.clone())
    .expect(format!("{}{}",utils::error_message("E0012").as_str(),file_name).as_str());
}
