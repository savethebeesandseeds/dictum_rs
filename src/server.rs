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
//   laws::mark_interrupt(&aux,&String::from(
//     r#"
// TITULO 1, CAPITULO 0 - Artículo 1.
// Colombia es un Estado social de derecho, organizado en forma de
// República unitaria, descentralizada, con autonomía de sus entidades territoriales,
// democrática, participativa y pluralista, fundada en el respeto de la dignidad humana, en
// el trabajo y la solidaridad de las personas que la integran y en la prevalencia del interés
// general.
// TITULO 11, CAPITULO 1 - Artículo 2.
// Son fines esenciales del Estado: servir a la comunidad, promover la
// prosperidad general y garantizar la efectividad de los principios, derechos y deberes
// consagrados en la Constitución; facilitar la participación de todos en las decisiones que
// los afectan y en la vida económica, política, administrativa y cultural de la Nación;
// defender la independencia nacional, mantener la integridad territorial y asegurar la
// convivencia pacifica y la vigencia de un orden justo.
// Las autoridades de la República están instituidas para proteger a todas las personas
// residentes en Colombia, en su vida, honra, bienes, creencias, y demás derechos y
// libertades, y para asegurar el cumplimiento de los deberes sociales del Estado y de los
// particulares.
// TITULO 10, CAPITULO 2 - Artículo 3.
// La soberanía reside exclusivamente en el pueblo, del cual emana el poder
// público. El pueblo la ejerce en forma directa o por medio de sus representantes, en los
// términos que la Constitución establece."#
//   ));
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