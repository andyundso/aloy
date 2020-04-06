use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

mod file_identifier;
mod webservice;

pub fn watch() -> std::result::Result<(), Box<dyn std::error::Error>> {
  let (tx, rx) = channel();

  // Automatically select the best implementation for your platform.
  let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  for path in &super::configuration::CONFIGURATION.paths {
    watcher.watch(path, RecursiveMode::Recursive)?;
  }

  println!("Aloy is configured, now waiting for events!");

  loop {
    match rx.recv() {
      Ok(event) => {
        match event {
          DebouncedEvent::NoticeWrite(path) => {
            send_event_to_service(path, "notice_write".to_owned());
          },
          DebouncedEvent::Create(path) => {
            send_event_to_service(path, "create".to_owned());
          },
          DebouncedEvent::Write(path) => {
            send_event_to_service(path, "write".to_owned());
          }
          DebouncedEvent::Rename(_old_path, new_path) => {
            send_event_to_service(new_path, "rename".to_owned());
          }
          _ => {}
        };
      }
      Err(e) => println!("watch error: {:?}", e),
    }
  }
}

fn send_event_to_service(path: PathBuf, event_type: String) {
  thread::spawn(move || {
    match file_identifier::file_information(path) {
      Ok(file_information) => {
        webservice::build_json(file_information, event_type);
        return Ok(());
      }
        ,
      Err(_) => {
        println!("an error happened");
        return Err(());
      },
    }
  });
}
