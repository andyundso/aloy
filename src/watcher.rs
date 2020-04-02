use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch(paths: Vec<String>) -> notify::Result<()> {
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
        Ok(event) => println!("{:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    }
  }
}
