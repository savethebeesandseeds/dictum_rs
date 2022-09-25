use std::{
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};
use config::Config;
use std::collections::HashMap;

// use std::time::Instant;
// let now = Instant::now();
// function
// println!("Enlapsed time reading config file : {:?}",now.elapsed().as_micros());

// // Prints the Type of a Variable
// pub fn print_type_of<T>(_: &T) {
//   println!("{}", std::any::type_name::<T>())
// }

// Reads a File, returns a Vec of all Lines
pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
  BufReader::new(File::open(filename)?).lines().collect()
}

// Reads a configuration file
pub fn read_config_file(filepath: &str) -> HashMap<String, String> {
  Config::builder()
    .add_source(config::File::with_name(filepath))
    .build()
    .unwrap()
    .try_deserialize::<HashMap<String, String>>()
    .unwrap()
}

// Defined Errors configuration
pub fn dictum_errors() -> HashMap<String, String> {
  return read_config_file("Errors");
}
// Defined Errors messages
pub fn error_message(code : &str) -> String {
  return dictum_errors()[code].clone();
}
// Defined File of configuration
pub fn dictum_config() -> HashMap<String, String> {
  return read_config_file("Config");
}
// Get the configured Language 
pub fn config_language() -> String {
  return dictum_config()["language"].clone();
}
// Get the configured path for models 
pub fn config_models_path() -> String {
  return dictum_config()["models_path"].clone();
}
// Get the configured path for models 
pub fn config_vocab_filename() -> String {
  return dictum_config()["vocab_filename"].clone();
}
// Get the configured Transformer Model
pub fn config_model() -> String {
  if config_language() == "es" {
    return dictum_config()["es_model"].clone();
  } else if config_language() == "en" {
    return dictum_config()["en_model"].clone();
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

// Get the documents folder
pub fn config_documents_folder() -> String {
  return dictum_config()["documents_folder"].clone();
}
// Get the encodings folder
pub fn config_encodings_folder() -> String {
  return dictum_config()["encodings_folder"].clone();
}
// Get the reference folder
pub fn config_reference_folder() -> String {
  return dictum_config()["reference_folder"].clone();
}
// Get the documents extension
pub fn config_documents_extension() -> String {
  return dictum_config()["documents_extension"].clone();
}
// Get the encodings extension
pub fn config_encodings_extension() -> String {
  return dictum_config()["encodings_extension"].clone();
}
// Get the reference extension
pub fn config_reference_extension() -> String {
  return dictum_config()["reference_extension"].clone();
}