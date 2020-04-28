#![cfg_attr(all(rustc_is_nightly, windows), feature(windows_by_handle))]

mod watcher;

#[macro_use]
extern crate lazy_static;
extern crate ureq;

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
