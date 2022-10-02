// --- --- --- --- --- --- --- --- --- --- 
// These are open source tools, 
// TSAHDU provides free legal assistance. 
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
mod laws;
mod catalogue;

#[launch]
fn tsahdu() -> _ {
  catalogue::load_catalogues_memory(true);
  rocket::build().attach(server::stage())
}