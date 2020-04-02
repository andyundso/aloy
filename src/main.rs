mod configuration_loader;
mod watcher;

fn main() {
  
  let configuration = configuration_loader::load("config.json").unwrap();

  // Initialize watcher loop
  if let Err(e) = watcher::watch(configuration.paths) {
        println!("error: {:?}", e)
    }
}
