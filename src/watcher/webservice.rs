use super::file_identifier::FileInformation;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::json;

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

pub fn send_json(file_information: FileInformation, event_type: String) -> Result<()> {
  let request_body = WebserviceRequestBody {
    id: file_information.id,
    event_type: event_type,
    body: DetailedBody {
      category: file_information.category,
      path: file_information.path,
    },
  };

  // Post body to a webservice
  ureq::post(&super::super::configuration::CONFIGURATION.webservice_url)
        .send_json(json!(request_body));

  return Ok(());
}
