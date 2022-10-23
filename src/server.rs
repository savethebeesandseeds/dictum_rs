// use rocket::http::{Status, ContentType};
// use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value, json};

use crate::utils;
use crate::laws;
use crate::language;
use crate::transformer;
use crate::catalogue;
use crate::mathematics;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CompareRequest {
  phrase1: language::Phrase,
  phrase2: language::Phrase
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SearchRequest {
  phrase: language::Phrase,
  pais: String,
  instrumento: String
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
  // // Interpet Law
  // let aux = laws::LawBook {
  //   pais:String::from("colombia"),
  //   instrumento:String::from("constitucion"),
  // };
  // laws::interpret_law(&aux);
  
  // // Consult
  // let adux = laws::LawIndex {
  //   book:aux,
  //   titulo:Some(1),
  //   capitulo:Some(0),
  //   articulo:Some(1),
  //   parte:None
  // };
  // let ret = catalogue::consult_catalogues_memory(&adux);
  
  // // Generate Embeddings
  // let phrase = language::phrase_fabric("Colombia es un Estado social de derecho, organizado en forma de República unitaria, descentralizada, con autonomía de sus entidades territoriales, democrática, participativa y pluralista, fundada en el respeto de la dignidad humana, en el trabajo y la solidaridad de las personas que la integran y en la prevalencia del interés general.".to_string());
  // let embedding = transformer::transform_phrase(&phrase);
  // // Compare against LawBook
  // let book = &laws::LawBook {
  //   pais: "colombia".to_string(),
  //   instrumento: "constitucion".to_string()
  // };
  // let embd = &transformer::Embedding {
  //   vector:embedding.clone(),
  //   etype: transformer::EmbeddingType::Total
  // };
  // dbg!(catalogue::compare_embedding_against_law_book(embd, book));
  

  // let mut encds: Vec<Vec<f32>> = Vec::new();
  // encds.push(Vec::from([1.0f32,2.0f32,3.0f32]));
  // encds.push(Vec::from([4.0f32,5.0f32,6.0f32]));
  // println!("encds : {:?}",encds);
  // println!("sum: 0 : {:?}",mathematics::vec2d_axis_sum::<f32>(encds.clone(),0));
  // println!("average: 0 : {:?}",mathematics::vec2d_axis_average::<f32>(encds.clone(),0));
  // println!("sum: 1 : {:?}",mathematics::vec2d_axis_sum::<f32>(encds.clone(),1));
  // println!("average: 1 : {:?}",mathematics::vec2d_axis_average::<f32>(encds.clone(),1));
  return String::from("pong");
}

#[post("/search", format="json", data = "<payload>")]
fn phrase_search_post(payload: Json<SearchRequest>) -> String {//Json<Vec<(laws::LawIndex,f32)>> {
  // Generate Embeddings
  let embedding = transformer::transform_phrase(&payload.phrase);
  if embedding.is_some() {
    // Compare against LawBook
    dbg!(catalogue::compare_embedding_against_law_book(
      &transformer::Embedding {
        vector:embedding.clone(),
        etype: transformer::EmbeddingType::Total
      }, 
      &laws::LawBook {
        pais: payload.pais.clone(),
        instrumento: payload.instrumento.clone()
      }));
    "ok".to_string()
  } else {
    "phrase is too short, not undersootd".to_string()
  }
    // Json::try_from().unwrap();
}

#[get("/norm/<phrase>")]
fn phrase_norm_get(phrase: String) -> String {
  // Sentences
  let sentences = Vec::from([phrase.clone()]);
  // Generate Embeddings
  let embeddings = transformer::transform_sentences(&sentences);
  // Return
  format!("Phrase: {:?}, norm: {:?}, entropy: {:?}", 
    phrase, mathematics::euclidean_magnitude(&embeddings[0]), mathematics::embeddings_entropy(&embeddings))
}

#[post("/norm", format="json", data = "<payload>")]
fn phrase_norm_post(payload: Json<language::Phrase>) -> String {
  // Generate Embeddings
  let embeddings = transformer::transform_phrase(&payload);
  // Compare against LawBook
  let book = &laws::LawBook {
    pais: "colombia".to_string(),
    instrumento: "constitucion".to_string()
  };
  let embd = &transformer::Embedding {
    vector:embeddings.clone(),
    etype: transformer::EmbeddingType::Total
  };
  dbg!(catalogue::compare_embedding_against_law_book(embd, book));
  // Return
  format!("Phrase: {:?}, norm: {:?}", 
    payload.text.clone(), mathematics::euclidean_magnitude(&embeddings.unwrap()))
}

#[post("/compare", format="json", data = "<payload>")]
fn phrases_distance_post(payload: Json<CompareRequest>) -> String {
  // Sentences
  let sentences = Vec::from([payload.phrase1.text.clone(),payload.phrase2.text.clone()]);
  // Generate Embeddings
  let embeddings = transformer::transform_sentences(&sentences);
  format!("Phrase1: {:?}, Phrase2: {:?}, Distance: {:?}", 
    payload.phrase1, payload.phrase2, mathematics::vector_euclidean_distance(&embeddings[0],&embeddings[1]))
}

#[get("/compare/<phrase1>/<phrase2>")]
fn phrases_distance_get(phrase1: String, phrase2: String) -> String {
  // Sentences
  let sentences = Vec::from([phrase1.clone(),phrase2.clone()]);
  // Generate Embeddings
  let embeddings = transformer::transform_sentences(&sentences);
  // Return
  format!("Phrase1: {:?}, Phrase2: {:?}, Distance: {:?}", 
    phrase1, phrase2, mathematics::vector_euclidean_distance(&embeddings[0],&embeddings[1]))
}

// #[post("/inform", format="json", data = "<payload>")]
// fn inform_post(payload: Json<InformRequest>) -> String {
//   let now = Instant::now();
//   // Catalogue fabric
//   let catalogue = catalogue::catalogue_fabric(
//     payload.pais.clone().to_lowercase(), 
//     payload.instrumento.clone().to_lowercase(), 
//     payload.titulo.clone(),
//     payload.capitulo.clone(), 
//     payload.articulo.clone(),
//     payload.parte.clone(), 
//     payload.phrase.clone(), 
//     Vec::new()); // empty Vec to request calculation
//   // Save catalogue
//   catalogue::save_catalogue(&catalogue);
//   println!("Enlapsed time informing catalogue : {:?}",now.elapsed().as_millis());
//   // Return hash reference
//   return catalogue.reference.dref;
// }


#[catch(404)]
fn not_found() -> Value {
  json!({
    "status": "error",
    "reason": utils::error_message("EHTTP0404").as_str()
  })
}

pub fn stage() -> rocket::fairing::AdHoc {
  catalogue::load_catalogues_memory(true);
  rocket::fairing::AdHoc::on_ignite("TSAHDU_server", |rocket| async {
    rocket.mount("/", routes![
      ping,
      phrase_norm_get,
      phrases_distance_get,
      phrase_norm_post,
      phrases_distance_post,
      phrase_search_post,
      // inform_post
      ])
    .register("/", catchers![not_found])
    // .manage()
  })
}