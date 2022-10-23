use std::collections::HashMap;
use std::collections::HashSet;
use rocket::serde::{Serialize, Deserialize};
use std::cmp::Eq;
use regex::Regex;
use std::ops::Range;

use crate::utils;
use crate::mathematics;
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

// To advance a mark is to advance to the next article, to the next chapter or to the next title
pub fn advance_mark(law_index: &mut LawIndex, mark: &(LawMark,u16,Range<usize>)) {
  match mark.0 {
    LawMark::Titulo   => {law_index.titulo   = Some(mark.1);}
    LawMark::Capitulo => {law_index.capitulo = Some(mark.1);}
    LawMark::Articulo => {law_index.articulo = Some(mark.1);}
  }
}
// Calculate the vector average of an entire Title
pub fn title_average(book: &LawBook, titulo: u16) -> Vec<f32> {
  mathematics::vec2d_axis_average::<f32>(&catalogue::CATALOGUES_MEMORY.lock().unwrap()
    .iter().filter(|(dindex,_)| 
      dindex.book==*book && dindex.titulo==Some(titulo))
      .collect::<HashMap<&LawIndex,&catalogue::Catalogue>>()
    .iter().map(|(_,dcatalogue)|
      dcatalogue.dmeaning.embedding.vector.clone().unwrap())
      .collect::<Vec<Vec<f32>>>(),0)
}
// Calculate the vector average of an entire Chapter
pub fn chapter_average(book: &LawBook, titulo: u16, capitulo: u16) -> Vec<f32> {
  mathematics::vec2d_axis_average::<f32>(&catalogue::CATALOGUES_MEMORY.lock().unwrap()
    .iter().filter(|(dindex,_)| 
      dindex.book==*book && dindex.titulo==Some(titulo) && dindex.capitulo==Some(capitulo))
      .collect::<HashMap<&LawIndex,&catalogue::Catalogue>>()
    .iter().map(|(_,dcatalogue)|
      dcatalogue.dmeaning.embedding.vector.clone().unwrap())
      .collect::<Vec<Vec<f32>>>(),0)
}
// Return all Titles in a Book
pub fn all_titles(book: &LawBook) -> HashSet<Option<u16>> {
  catalogue::CATALOGUES_MEMORY.lock().unwrap()
  .iter().filter(|(dindex,_)| 
    dindex.book == *book && 
    dindex.titulo.is_some())
  .map(|(pindex,_)| pindex.titulo).collect::<HashSet<Option<u16>>>()
}
// Return all Chapters in a Book's Title
pub fn all_chapters_in_title(book: &LawBook, title: u16) -> HashSet<Option<u16>> {
  catalogue::CATALOGUES_MEMORY.lock().unwrap()
  .iter().filter(|(dindex, _)| 
    dindex.book==*book && 
    dindex.titulo.is_some() && 
    dindex.capitulo.is_some() && 
    dindex.titulo==Some(title))
  .map(|(pindex,_)| pindex.capitulo).collect::<HashSet<Option<u16>>>()
  // all_titles(book).iter().map(|dtitle| 
  //   .collect::<HashSet<Option<u16>>>()
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
  // Fabric Catalogue for all Average Titles 
  all_titles(book).iter().for_each(|dtitle| 
    make the catalogue for average items
    (title_average(book, dtitle.unwrap()))
  );
  // Fabric Catalogue for all Average Chapters 
  all_titles(book).iter().for_each(|dtitle| 
    (*dtitle,all_chapters_in_title(book, dtitle.unwrap()))).collect::<Vec<(Option<u16>,HashSet<Option<u16>>)>>()
    .iter().map(|(dtitle,dchapters)|
      dchapters.iter().map(|pchapter|
        chapter_average(book, dtitle.unwrap(),pchapter.unwrap())
      ));
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