// use rocket::http::{Status, ContentType};
// use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value, json};
use std::time::Instant;

use crate::utils;
use crate::laws;
use crate::language;
use crate::catalogue;
use crate::transformer;
use crate::mathematics;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CompareRequest {
  phrase1: language::Phrase,
  phrase2: language::Phrase
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct InformRequest {
  phrase: language::Phrase,
  pais: String,
  instrumento: String,
  titulo: u16,
  capitulo: u16,
  articulo: u16,
  parte: Option<u16>
}

#[get("/ping")]
fn ping() -> String {
  let aux = &laws::LawBook {
    pais:String::from("colombia"),
    instrumento:String::from("constitucion"),
  };
  laws::interpret_law(aux);
  return String::from("pong");
}
#[get("/norm/<phrase>")]
fn phrase_norm_get(phrase: String) -> String {
  // Sentences
  let sentences = Vec::from([phrase.clone()]);
  // Generate Embeddings
  let embeddings = transformer::transform_sentences(&sentences);
  // Return
  format!("Phrase: {:?}, norm: {:?}", 
    phrase, mathematics::vector_norm(&embeddings[0]))
}

#[post("/norm", format="json", data = "<payload>")]
fn phrase_norm_post(payload: Json<language::Phrase>) -> String {
  // Sentences
  let sentences = Vec::from([payload.text.clone()]);
  // Generate Embeddings
  let embeddings = transformer::transform_sentences(&sentences);
  // Return
  format!("Phrase: {:?}, norm: {:?}", 
    payload, mathematics::vector_norm(&embeddings[0]))
}

#[post("/compare", format="json", data = "<payload>")]
fn phrases_distance_post(payload: Json<CompareRequest>) -> String {
  // Sentences
  let sentences = Vec::from([payload.phrase1.text.clone(),payload.phrase2.text.clone()]);
  // Generate Embeddings
  let embeddings = transformer::transform_sentences(&sentences);
  format!("Phrase1: {:?}, Phrase2: {:?}, Distance: {:?}", 
    payload.phrase1, payload.phrase2, mathematics::vector_diff(&embeddings[0],&embeddings[1]))
}

#[get("/compare/<phrase1>/<phrase2>")]
fn phrases_distance_get(phrase1: String, phrase2: String) -> String {
  // Sentences
  let sentences = Vec::from([phrase1.clone(),phrase2.clone()]);
  // Generate Embeddings
  let embeddings = transformer::transform_sentences(&sentences);
  // Return
  format!("Phrase1: {:?}, Phrase2: {:?}, Distance: {:?}", 
    phrase1, phrase2, mathematics::vector_diff(&embeddings[0],&embeddings[1]))
}

#[post("/inform", format="json", data = "<payload>")]
fn inform_post(payload: Json<InformRequest>) -> String {
  let now = Instant::now();
  // Catalogue fabric
  let catalogue = catalogue::catalogue_fabric(
    payload.pais.clone().to_lowercase(), 
    payload.instrumento.clone().to_lowercase(), 
    payload.titulo.clone(),
    payload.capitulo.clone(), 
    payload.articulo.clone(),
    payload.parte.clone(), 
    payload.phrase.clone().text.clone(), 
    Vec::new()); // empty Vec to request calculation
  // Save catalogue
  catalogue::save_catalogue(&catalogue);
  println!("Enlapsed time informing catalogue : {:?}",now.elapsed().as_millis());
  // Return hash reference
  return catalogue.reference.dref;
}


#[catch(404)]
fn not_found() -> Value {
  json!({
    "status": "error",
    "reason": utils::error_message("EHTTP0404").as_str()
  })
}

pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("TSAHDU_server", |rocket| async {
    rocket.mount("/", routes![
      ping,
      phrase_norm_get,
      phrases_distance_get,
      phrase_norm_post,
      phrases_distance_post,
      inform_post])
    .register("/", catchers![not_found])
    // .manage()
  })
}