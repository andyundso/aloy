pub struct FileInformation {
  pub id: String,
  pub path: String,
  pub category: String,
}

#[cfg(windows)]
pub fn file_information(path: PathBuf) -> io::Result<(u64)> {
  Ok(1234)
}

#[cfg(not(windows))]
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

pub fn file_information(path: PathBuf) -> io::Result<FileInformation> {
  let meta = fs::metadata(path.clone())?;
  let inode = meta.ino();

  let file_information = FileInformation {
    id: inode.to_string().to_owned(),
    path: path
      .clone()
      .into_os_string()
      .into_string()
      .unwrap()
      .to_owned(),
    category: if path.is_dir() {
      "directory"
    } else if path.is_file() {
      "file"
    } else {
      "unknown"
    }
    .to_owned(),
  };

  Ok(file_information)
}
