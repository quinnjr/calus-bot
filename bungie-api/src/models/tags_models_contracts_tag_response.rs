/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsModelsContractsTagResponse {
  #[serde(rename = "tagText")]
  tag_text: Option<String>,
  #[serde(rename = "ignoreStatus")]
  ignore_status: Option<::models::IgnoresIgnoreResponse>
}

impl TagsModelsContractsTagResponse {
  pub fn new() -> TagsModelsContractsTagResponse {
    TagsModelsContractsTagResponse {
      tag_text: None,
      ignore_status: None
    }
  }

  pub fn set_tag_text(&mut self, tag_text: String) {
    self.tag_text = Some(tag_text);
  }

  pub fn with_tag_text(mut self, tag_text: String) -> TagsModelsContractsTagResponse {
    self.tag_text = Some(tag_text);
    self
  }

  pub fn tag_text(&self) -> Option<&String> {
    self.tag_text.as_ref()
  }

  pub fn reset_tag_text(&mut self) {
    self.tag_text = None;
  }

  pub fn set_ignore_status(&mut self, ignore_status: ::models::IgnoresIgnoreResponse) {
    self.ignore_status = Some(ignore_status);
  }

  pub fn with_ignore_status(mut self, ignore_status: ::models::IgnoresIgnoreResponse) -> TagsModelsContractsTagResponse {
    self.ignore_status = Some(ignore_status);
    self
  }

  pub fn ignore_status(&self) -> Option<&::models::IgnoresIgnoreResponse> {
    self.ignore_status.as_ref()
  }

  pub fn reset_ignore_status(&mut self) {
    self.ignore_status = None;
  }

}



