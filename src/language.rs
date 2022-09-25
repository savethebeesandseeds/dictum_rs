use lazy_static::lazy_static;
use rocket::serde::{Serialize, Deserialize};
use core::fmt::Debug;
use std::sync::Mutex;

use crate::utils;

#[derive(Debug,Clone,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Phrase {
  pub text: String
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