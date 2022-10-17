use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use rocket::serde::{Serialize, Deserialize};
use std::cmp::Eq;
use regex::Regex;
use std::ops::Range;

use crate::utils;
use crate::catalogue;
use crate::language;
use crate::files;

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
  pub titulo: Option<u16>,
  pub capitulo: Option<u16>,
  pub articulo: Option<u16>,
  pub parte: Option<u16>
}
#[derive(Debug)]
pub enum LawMark {
  Capitulo,
  Titulo,
  Articulo
}

lazy_static! {
  static ref LAWS_MEMORY: Mutex<HashMap<LawIndex,language::Phrase>> = Mutex::new(
    HashMap::new()
  );
}

pub fn consult_laws_memory(reference: &LawIndex) -> language::Phrase {
  if !LAWS_MEMORY.lock().unwrap().contains_key(reference) {
    load_law_memory();
  }
  return LAWS_MEMORY.lock().unwrap().get(reference)
    .expect(format!("{}",utils::error_message("E0010")).as_str()).clone();
}
pub fn load_law_memory() {
}

// Marks
pub fn advance_mark(law_index: &mut LawIndex, mark: &(LawMark,u16,Range<usize>)) {
  match mark.0 {
    LawMark::Titulo   => {law_index.titulo   = Some(mark.1);}
    LawMark::Capitulo => {law_index.capitulo = Some(mark.1);}
    LawMark::Articulo => {law_index.articulo = Some(mark.1);}
  }
}
// Given a catalogue of Law this function reads, interprests and dumps a TsahduCatalogue
pub fn interpret_law(book: &LawBook) {
  let current_law_index =  &mut LawIndex {
    book:book.clone(),
    titulo:None,
    capitulo:None,
    articulo:None,
    parte:None
  };
  let text_of_law = &language::phrase_fabric(files::read_law_book(book));
  let marks = mark_text_of_law(text_of_law, book);
  for mark in marks.windows(2) {
    advance_mark(current_law_index, mark.get(0).unwrap());
    let phrase_of_law = &language::clean_phrase_of_law(
      &language::phrase_fabric(utils::substring(&text_of_law.text, 
        mark.get(0).unwrap().2.end, 
        mark.get(1).unwrap().2.start)));
    catalogue::catalogue_mech(phrase_of_law, &current_law_index);
  }
  advance_mark(current_law_index, marks.last().unwrap());
  let phrase_of_law = &language::clean_phrase_of_law(&
    &language::phrase_fabric(utils::substring(&text_of_law.text, 
      marks.last().unwrap().2.end, 
      text_of_law.text.len())));
  catalogue::catalogue_mech(phrase_of_law, &current_law_index);
}
// Efective read of laws, returns markings of all aparitions of [Articulo, Titulo, Capitulo]
pub fn mark_text_of_law(text_of_law: &language::Phrase, book: &LawBook) -> Vec<(LawMark, u16, Range<usize>)> {
  let mut marks = mark_interrupt_articulo(book, &text_of_law.text)
    .iter().map(|x| (LawMark::Articulo,x.0,x.1.clone())).collect::<Vec<(LawMark,u16,Range<usize>)>>();
  marks.append(mark_interrupt_titulo(book, &text_of_law.text)
    .iter().map(|x| (LawMark::Titulo,x.0,x.1.clone())).collect::<Vec<(LawMark,u16,Range<usize>)>>().as_mut());
  marks.append(mark_interrupt_capitulo(book, &text_of_law.text)
    .iter().map(|x| (LawMark::Capitulo,x.0,x.1.clone())).collect::<Vec<(LawMark,u16,Range<usize>)>>().as_mut());
  marks.sort_by(|a,b| a.2.end.partial_cmp(&b.2.end).unwrap());
  return marks;
}

// Regex
pub fn regex_interpret_law(regex_expresion: &str, text: &String) -> Vec<(u16,Range<usize>)> {
  dbg!(regex_expresion);
  Regex::new(regex_expresion).expect(format!("[{}] is not a regex expression",regex_expresion).as_str()).find_iter(text.as_str())
    .filter_map(|x| Some((utils::atoi::<u16>(x.as_str()).expect(format!("[{}] cannot be casted to atoi",x.as_str()).as_str()), x.range())))
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