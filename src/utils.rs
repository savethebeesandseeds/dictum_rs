use std::{
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};
use walkdir::DirEntry;
use config::Config;
use std::collections::HashMap;
use std::str::FromStr;

use crate::laws;

// use std::time::Instant;
// let now = Instant::now();
// function
// println!("Enlapsed time reading config file : {:?}",now.elapsed().as_micros());

// // Prints the Type of a Variable
// pub fn print_type_of<T>(_: &T) {
//   println!("{}", std::any::type_name::<T>())
// }

// Returns the number expresion of a string "789waka123" -> 789123
pub fn atoi<F: FromStr>(input: &str) -> Result<F, <F as FromStr>::Err> {
  input.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<F>()
}

// Returns overlaping chunks f([5,4,3,2,1],4,2) -> [[5,4,3,2],[3,2,1]]
pub fn overlaping_chunks<T: Clone>(input: &Vec<T>, chunk_size: usize, overlap_size: usize) -> Vec<Vec<T>> {
  let mut ret : Vec<Vec<T>> = Vec::new();
  let mut partial : Vec<T> = Vec::new();
  let mut c_idx : usize = 0;
  let mut stop_idx : usize = c_idx + chunk_size;
  while c_idx<input.len() {
    if c_idx >= stop_idx {
      c_idx-=overlap_size;
      stop_idx=c_idx+chunk_size;
      ret.push(partial.clone());
      partial.clear();
    }
    partial.push(input[c_idx].clone());
    c_idx+=1;
  }
  ret.push(partial.clone());
  partial.clear();
  return ret;
}

pub fn substring(text: &String, start: usize, end: usize) -> String {
  text[start..end].to_string()
}
// Reads a File, returns a Vec of all Lines
pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
  BufReader::new(File::open(filename)?).lines().collect()
}
// extracts the name of a file from a entry dir
pub fn name_from_dir_entry(filepath: &DirEntry) -> String {
  return filepath.file_name().to_str().unwrap().to_string();
}
// Reads a configuration file
pub fn read_config_file(filepath: &str) -> HashMap<String, String> {
  Config::builder()
    .add_source(config::File::with_name(filepath))
    .build()
    .unwrap()
    .try_deserialize::<HashMap<String, String>>()
    .expect(format!("Unable to access file: {}",filepath).as_str())
}

// Defined Errors configuration
pub fn tsahdu_errors() -> HashMap<String, String> {
  return read_config_file("Errors");
}
// Defined Errors messages
pub fn error_message(code : &str) -> String {
  return tsahdu_errors()[code].clone();
}
// Defined File of configuration
pub fn tsahdu_config() -> HashMap<String, String> {
  return read_config_file("Config");
}
// Get the configured Language 
pub fn config_language() -> String {
  return tsahdu_config().get("language").expect(format!("Key not found in Config: language").as_str()).clone();
}
// Get the configured path for models 
pub fn config_models_path() -> String {
  return tsahdu_config().get("models_path").expect(format!("Key not found in Config: models_path").as_str()).clone();
}
// Get the configured path for models 
pub fn config_vocab_filename() -> String {
  return tsahdu_config().get("vocab_filename").expect(format!("Key not found in Config: vocab_filename").as_str()).clone();
}
// Get the configured Transformer Model
pub fn config_model() -> String {
  if config_language() == "es" {
    return tsahdu_config().get("es_model").expect(format!("Key not found in Config: es_model").as_str()).clone();
  } else if config_language() == "en" {
    return tsahdu_config().get("en_model").expect(format!("Key not found in Config: en_model").as_str()).clone();
  } else {
    panic!("{}",error_message("E0000"));
  }
}

// Get the Transformer Model Path
pub fn config_model_path() -> String {
  return format!("{}{}/",config_models_path().as_str().to_owned(),config_model().as_str().to_owned());
}

// Get the Transformer Vocab file
pub fn config_vocab_file() -> String {
  return format!("{}{}",config_model_path().as_str().to_owned(),config_vocab_filename().as_str().to_owned());
}
// Get the reference folder
pub fn config_reference_folder() -> String {
  return tsahdu_config().get("reference_folder").expect(format!("Key not found in Config: reference_folder").as_str()).clone();
}
// Get the laws folder
pub fn config_laws_folder() -> String {
  return tsahdu_config().get("laws_folder").expect(format!("Key not found in Config: laws_folder").as_str()).clone();
}
// Get the embeddings extension
pub fn config_embeddings_extension() -> String {
  return tsahdu_config().get("embeddings_extension").expect(format!("Key not found in Config: embeddings_extension").as_str()).clone();
}
// Get the reference extension
pub fn config_reference_extension() -> String {
  return tsahdu_config().get("reference_extension").expect(format!("Key not found in Config: reference_extension").as_str()).clone();
}
// Get the law extension
pub fn config_law_extension() -> String {
  return tsahdu_config().get("laws_extension").expect(format!("Key not found in Config: laws_extension").as_str()).clone();
}
// Get the law configuration extension
pub fn config_law_config_extension() -> String {
  return tsahdu_config().get("laws_config_extension").expect(format!("Key not found in Config: laws_config_extension").as_str()).clone();
}
// Get the law configuration extension
pub fn config_law(book: &laws::LawBook) -> HashMap<String,String> {
  let search_for = 
    format!("{}{}.{}{}",
      config_laws_folder(),
      book.pais,
      book.instrumento,
      config_law_config_extension());
  return read_config_file(search_for.as_str());
}
// Get the minimum_window_size
pub fn config_minimum_window_size() -> String {
  return tsahdu_config().get("minimum_window_size").expect(format!("Key not found in Config: minimum_window_size").as_str()).clone();
}
// Get the maximum_window_size
pub fn config_maximum_window_size() -> String {
  return tsahdu_config().get("maximum_window_size").expect(format!("Key not found in Config: maximum_window_size").as_str()).clone();
}
// Get the window_retrocede
pub fn config_window_retrocede() -> String {
  return tsahdu_config().get("window_retrocede").expect(format!("Key not found in Config: window_retrocede").as_str()).clone();
}
// Get the return_count
pub fn config_return_count() -> usize {
  return atoi::<usize>(tsahdu_config().get("return_count").expect(format!("Key not found in Config: return_count").as_str())).expect("wrong configuration, return_count must be a numeric string");
}