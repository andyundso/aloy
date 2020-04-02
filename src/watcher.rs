use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

mod file_identifier;
mod webservice;

pub fn watch(paths: Vec<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
  let (tx, rx) = channel();

  // Automatically select the best implementation for your platform.
  let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.

  for path in &paths {
    watcher.watch(path, RecursiveMode::Recursive)?;
  }

  println!("Aloy is configured, now waiting for events!");

  loop {
    match rx.recv() {
      Ok(event) => {
        match event {
          DebouncedEvent::NoticeWrite(path)
          | DebouncedEvent::Create(path)
          | DebouncedEvent::Write(path) => {
            send_event_to_service(path)?;
          }
          DebouncedEvent::Rename(_old_path, new_path) => {
            send_event_to_service(new_path)?;
          }
          _ => {}
        };
      }
      Err(e) => println!("watch error: {:?}", e),
    }
  }
}

fn send_event_to_service(path: PathBuf) -> std::result::Result<(), Box<dyn std::error::Error>> {
  let file_information = file_identifier::file_information(path)?;
  webservice::build_json(file_information)?;

  Ok(())
}
