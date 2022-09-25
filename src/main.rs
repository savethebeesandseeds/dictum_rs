// --- --- --- --- --- --- --- --- --- --- 
// These are open source tools, 
// DICTUM provides free legal assistance. 
// author: waajacu
// contact: savethebeesandseeds@gmail.com
// --- --- --- --- --- --- --- --- --- --- 
#[macro_use] extern crate rocket;

mod mathematics;
mod cryptography;
mod language;
mod transformer;
mod server;
mod utils;
mod documents;

#[launch]
fn dictum() -> _ {
  documents::load_documents_memory(true);
  rocket::build().attach(server::stage())
}