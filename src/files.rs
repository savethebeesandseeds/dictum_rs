use std::fs;
use std::fs::create_dir_all;
use walkdir::DirEntry;

use crate::language;
use crate::transformer;
use crate::catalogue;
use crate::utils;
use crate::laws;

// Folders paths
pub fn reference_foldername(dindex: &laws::LawIndex) -> String {
  format!("{}{}.{}/",utils::config_reference_folder(),dindex.book.pais,dindex.book.instrumento)
}
pub fn embeddings_foldername(dindex: &laws::LawIndex) -> String {
  format!("{}{}.{}/",utils::config_reference_folder(),dindex.book.pais,dindex.book.instrumento)
}
pub fn file_of_law_foldername(dindex: &laws::LawIndex) -> String {
  format!("{}{}.{}/",utils::config_reference_folder(),dindex.book.pais,dindex.book.instrumento)
}
pub fn book_of_law_foldername() -> String {
  format!("{}",utils::config_laws_folder())
}

// File names
pub fn law_index_to_filename(dindex: &laws::LawIndex) -> String {
  format!("{}.{}{}{}{}{}",
    dindex.book.pais,dindex.book.instrumento,
    if dindex.titulo.is_none() {"".to_string()}  else {format!(".titulo-{}",dindex.titulo.unwrap()).to_string()},
    if dindex.capitulo.is_none() {"".to_string()}  else {format!(".capitulo-{}",dindex.capitulo.unwrap()).to_string()},
    if dindex.articulo.is_none() {"".to_string()}  else {format!(".articulo-{}",dindex.articulo.unwrap()).to_string()},
    if dindex.parte.is_none() {"".to_string()}  else {format!(".parte-{}",dindex.parte.unwrap()).to_string()})
}
pub fn reference_filename(dindex: &laws::LawIndex) -> String {
  format!("{}{}{}",reference_foldername(dindex),&law_index_to_filename(dindex),utils::config_reference_extension())
}
pub fn embeddings_filename(dindex: &laws::LawIndex) -> String {
  format!("{}{}{}",embeddings_foldername(dindex),&law_index_to_filename(dindex),utils::config_embeddings_extension())
}
pub fn file_of_law_filename(dindex: &laws::LawIndex) -> String {
  format!("{}{}{}",file_of_law_foldername(dindex),law_index_to_filename(dindex),utils::config_law_extension())
}
pub fn book_of_law_filename(book: &laws::LawBook) -> String {
  format!("{}{}.{}{}",
    book_of_law_foldername(),
    book.pais,
    book.instrumento,
    utils::config_law_extension())
}


// Files Readings
pub fn read_reference_file(filepath: &DirEntry) -> (laws::LawIndex, transformer::EmbeddingType) {
  let filecontent = utils::read_config_file(filepath.path().as_os_str().to_str().unwrap());
  let titulo = filecontent["titulo"].parse::<i16>().unwrap();
  let capitulo = filecontent["capitulo"].parse::<i16>().unwrap();
  let articulo = filecontent["articulo"].parse::<i16>().unwrap();
  let parte = filecontent["parte"].parse::<i16>().unwrap();
  let law_index = laws::LawIndex { 
    book: laws::LawBook {
    pais:filecontent["pais"].clone(),
    instrumento:filecontent["instrumento"].clone()
    }, 
    titulo: if titulo<0 { None } else { Some(titulo.try_into().unwrap()) }, 
    capitulo: if capitulo<0 { None } else { Some(capitulo.try_into().unwrap()) }, 
    articulo: if articulo<0 { None } else { Some(articulo.try_into().unwrap()) }, 
    parte: if parte<0 { None } else { Some(parte.try_into().unwrap()) }
  };
  assert!(filecontent["etype"].as_str() == "Total" || filecontent["etype"].as_str() == "Average");
  let etype = if filecontent["etype"].as_str() == "Total" 
    {transformer::EmbeddingType::Total} else {transformer::EmbeddingType::Average};
  return (law_index, etype);
}
pub fn read_law_book(book: &laws::LawBook) -> String {
  fs::read_to_string(book_of_law_filename(book))
    .expect(format!("{} : {}",utils::error_message("E0011").as_str(),book_of_law_filename(book)).as_str())
}
pub fn read_phrase_of_law(dindex: &laws::LawIndex) -> String {
  fs::read_to_string(&file_of_law_filename(dindex))
    .expect(format!("{} : {}",utils::error_message("E0002").as_str(),&file_of_law_filename(dindex)).as_str())
}
// Files Writing
pub fn write_file_of_law(phrase_of_law: &language::Phrase, dindex: &laws::LawIndex) {
  create_dir_all(file_of_law_foldername(dindex)).unwrap();
  fs::write(file_of_law_filename(dindex), phrase_of_law.text.clone())
    .expect(format!("{}{}",utils::error_message("E0012").as_str(),file_of_law_filename(dindex)).as_str());
}
pub fn write_reference_file(doc: &catalogue::Catalogue) {
  fs::write(reference_filename(&doc.dindex),format!(
r#"pais = "{}"
instrumento = "{}"
titulo = "{}"
capitulo = "{}"
articulo = "{}"
parte = "{}"
etype = "{:?}""#,
  doc.dindex.book.pais,
  doc.dindex.book.instrumento,
  if doc.dindex.titulo.is_none() {"-1".to_string()}  else {format!("{}",doc.dindex.titulo.unwrap()).to_string()},
  if doc.dindex.capitulo.is_none() {"-1".to_string()}  else {format!("{}",doc.dindex.capitulo.unwrap()).to_string()},
  if doc.dindex.articulo.is_none() {"-1".to_string()}  else {format!("{}",doc.dindex.articulo.unwrap()).to_string()},
  if doc.dindex.parte.is_none() {"-1".to_string()}  else {format!("{}",doc.dindex.parte.unwrap()).to_string()},
  doc.dmeaning.embedding.etype))
  .expect(format!("{}: {}",utils::error_message("E0008"),reference_filename(&doc.dindex)).as_str());
}
pub fn write_embeddings_file(doc: &catalogue::Catalogue) {
  fs::write(embeddings_filename(&doc.dindex),&doc.dmeaning.embedding.vector.clone().unwrap()
    .iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("\n"))
    .expect(format!("{}: {}",utils::error_message("E0005"),embeddings_filename(&doc.dindex)).as_str());
}