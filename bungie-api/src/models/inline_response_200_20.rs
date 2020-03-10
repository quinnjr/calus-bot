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
pub struct InlineResponse20020 {
  #[serde(rename = "Response")]
  response: Option<::models::GroupsV2GroupSearchResponse>,
  #[serde(rename = "ErrorCode")]
  error_code: Option<i32>,
  #[serde(rename = "ThrottleSeconds")]
  throttle_seconds: Option<i32>,
  #[serde(rename = "ErrorStatus")]
  error_status: Option<String>,
  #[serde(rename = "Message")]
  message: Option<String>,
  #[serde(rename = "MessageData")]
  message_data: Option<::std::collections::HashMap<String, String>>,
  #[serde(rename = "DetailedErrorTrace")]
  detailed_error_trace: Option<String>
}

impl InlineResponse20020 {
  pub fn new() -> InlineResponse20020 {
    InlineResponse20020 {
      response: None,
      error_code: None,
      throttle_seconds: None,
      error_status: None,
      message: None,
      message_data: None,
      detailed_error_trace: None
    }
  }

  pub fn set_response(&mut self, response: ::models::GroupsV2GroupSearchResponse) {
    self.response = Some(response);
  }

  pub fn with_response(mut self, response: ::models::GroupsV2GroupSearchResponse) -> InlineResponse20020 {
    self.response = Some(response);
    self
  }

  pub fn response(&self) -> Option<&::models::GroupsV2GroupSearchResponse> {
    self.response.as_ref()
  }

  pub fn reset_response(&mut self) {
    self.response = None;
  }

  pub fn set_error_code(&mut self, error_code: i32) {
    self.error_code = Some(error_code);
  }

  pub fn with_error_code(mut self, error_code: i32) -> InlineResponse20020 {
    self.error_code = Some(error_code);
    self
  }

  pub fn error_code(&self) -> Option<&i32> {
    self.error_code.as_ref()
  }

  pub fn reset_error_code(&mut self) {
    self.error_code = None;
  }

  pub fn set_throttle_seconds(&mut self, throttle_seconds: i32) {
    self.throttle_seconds = Some(throttle_seconds);
  }

  pub fn with_throttle_seconds(mut self, throttle_seconds: i32) -> InlineResponse20020 {
    self.throttle_seconds = Some(throttle_seconds);
    self
  }

  pub fn throttle_seconds(&self) -> Option<&i32> {
    self.throttle_seconds.as_ref()
  }

  pub fn reset_throttle_seconds(&mut self) {
    self.throttle_seconds = None;
  }

  pub fn set_error_status(&mut self, error_status: String) {
    self.error_status = Some(error_status);
  }

  pub fn with_error_status(mut self, error_status: String) -> InlineResponse20020 {
    self.error_status = Some(error_status);
    self
  }

  pub fn error_status(&self) -> Option<&String> {
    self.error_status.as_ref()
  }

  pub fn reset_error_status(&mut self) {
    self.error_status = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> InlineResponse20020 {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_message_data(&mut self, message_data: ::std::collections::HashMap<String, String>) {
    self.message_data = Some(message_data);
  }

  pub fn with_message_data(mut self, message_data: ::std::collections::HashMap<String, String>) -> InlineResponse20020 {
    self.message_data = Some(message_data);
    self
  }

  pub fn message_data(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.message_data.as_ref()
  }

  pub fn reset_message_data(&mut self) {
    self.message_data = None;
  }

  pub fn set_detailed_error_trace(&mut self, detailed_error_trace: String) {
    self.detailed_error_trace = Some(detailed_error_trace);
  }

  pub fn with_detailed_error_trace(mut self, detailed_error_trace: String) -> InlineResponse20020 {
    self.detailed_error_trace = Some(detailed_error_trace);
    self
  }

  pub fn detailed_error_trace(&self) -> Option<&String> {
    self.detailed_error_trace.as_ref()
  }

  pub fn reset_detailed_error_trace(&mut self) {
    self.detailed_error_trace = None;
  }

}



