mod watcher;

#[macro_use]
extern crate lazy_static;

pub mod configuration {
  mod configuration_loader;

  lazy_static! {
      pub static ref CONFIGURATION: configuration_loader::Configuration = configuration_loader::load("config.json").unwrap();
  }
}


fn main() {
  // Initialize watcher loop
  if let Err(e) = watcher::watch() {
        println!("error: {:?}", e)
    }
}
