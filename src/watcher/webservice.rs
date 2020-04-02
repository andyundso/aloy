use super::file_identifier::FileInformation;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct WebserviceRequestBody {
  pub id: String,
  pub event_type: String,
  pub body: DetailedBody,
}

#[derive(Serialize, Deserialize)]
pub struct DetailedBody {
  pub path: String,
  pub category: String,
}

pub fn build_json(file_information: FileInformation, event_type: String) -> Result<()> {
  let request_body = WebserviceRequestBody {
    id: file_information.id,
    event_type: event_type,
    body: DetailedBody {
      category: file_information.category,
      path: file_information.path,
    },
  };

  // Serialize it to a JSON string.
  let j = serde_json::to_string(&request_body)?;

  // Print, write to a file, or send to an HTTP server.
  println!("{}", j);

  Ok(())
}
