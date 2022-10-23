use lazy_static::lazy_static;
use rocket::serde::{Serialize, Deserialize};
use core::fmt::Debug;
use std::sync::Mutex;

use crate::utils;
use crate::language;
use crate::laws;

#[derive(Debug,Clone,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Phrase {
  pub text: String
}

#[derive(Debug)]
pub enum TextOfLawValidation {
  Short,
  Long,
  Proper
}

#[derive(Debug, Clone)]
pub struct Vocab {
  pub file: String, 
  pub words: Vec<String>
}

lazy_static! {
  static ref VALIDATION_VOCAB: Mutex<Vocab> = Mutex::new(
    Vocab {
      file: String::from(utils::config_vocab_file()),
      words: utils::lines_from_file(utils::config_vocab_file())
        .expect(utils::error_message("E0001").as_str())
    }
  );
}

pub fn phrase_fabric(text: String) -> Phrase {
  return Phrase {
    text: text
  }
}

// Phrases of Law
pub fn validate_phrase(phrase_of_law: &language::Phrase) -> TextOfLawValidation {
  if phrase_of_law.text == "" || phrase_of_law.text.split(" ").collect::<Vec<&str>>().len() < utils::atoi::<usize>(utils::config_minimum_window_size().as_str()).unwrap() {
    TextOfLawValidation::Short
  } else if phrase_of_law.text.split(" ").collect::<Vec<&str>>().len() > utils::atoi::<usize>(utils::config_maximum_window_size().as_str()).unwrap() {
    TextOfLawValidation::Long
  } else {
    TextOfLawValidation::Proper
  }
}
pub fn clean_phrase_of_law(phrase_of_law: &language::Phrase) -> language::Phrase {
  return language::phrase_fabric(phrase_of_law.text.replace("\n"," ").replace("  "," ").trim().to_string());
}
pub fn segment_phrase_with_index(phrase_of_law: &language::Phrase, index: &laws::LawIndex) -> Vec<(laws::LawIndex, language::Phrase)> { 
  let mut ret : Vec<(laws::LawIndex, language::Phrase)> = Vec::new();
  let mut c_index = index.clone();
  match validate_phrase(phrase_of_law) {
    TextOfLawValidation::Short => {}
    TextOfLawValidation::Proper => {
      c_index.parte = None;
      ret.push((c_index.clone(),phrase_of_law.clone()));
    }
    TextOfLawValidation::Long => {
      let mut parte : Option<u16> = Some(0);
      for seg in utils::overlaping_chunks::<&str>(
        &phrase_of_law.text.split(" ").collect::<Vec<&str>>(), 
        utils::atoi::<usize>(utils::config_maximum_window_size().as_str()).unwrap(), 
        utils::atoi::<usize>(utils::config_window_retrocede().as_str()).unwrap())
        .iter().map(|x| x.join(" ")).collect::<Vec<String>>() {
        c_index.parte=parte;
        ret.push((c_index.clone(),language::phrase_fabric(seg)));
        parte=Some(parte.unwrap()+1);
      }
    }
  }
  return ret;
}
pub fn segment_phrase(phrase_of_law: &language::Phrase) -> Vec<language::Phrase> { 
  let mut ret : Vec<language::Phrase> = Vec::new();
  match validate_phrase(phrase_of_law) {
    TextOfLawValidation::Short => {}
    TextOfLawValidation::Proper => {
      ret.push(phrase_of_law.clone());
    }
    TextOfLawValidation::Long => {
      for seg in utils::overlaping_chunks::<&str>(
        &phrase_of_law.text.split(" ").collect::<Vec<&str>>(), 
        utils::atoi::<usize>(utils::config_maximum_window_size().as_str()).unwrap(), 
        utils::atoi::<usize>(utils::config_window_retrocede().as_str()).unwrap())
          .iter().map(|x| x.join(" ")).collect::<Vec<String>>() {
        ret.push(language::phrase_fabric(seg));
      }
    }
  }
  return ret;
}
