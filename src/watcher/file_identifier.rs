use std::fs;
use std::io;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::MetadataExt;

pub struct FileInformation {
  pub id: String,
  pub path: String,
  pub category: String,
}

#[cfg(target_os = "windows")]
pub fn file_information(path: PathBuf) -> io::Result<(FileInformation)> {
    let meta = fs::metadata(path.clone())?;
    let file_index = meta.file_index().unwrap_or(0);
  
    let file_information = FileInformation {
      id: file_index.to_string().to_owned(),
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

#[cfg(not(target_os = "windows"))]
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
