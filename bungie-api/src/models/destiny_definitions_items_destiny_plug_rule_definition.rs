/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsItemsDestinyPlugRuleDefinition : Dictates a rule around whether the plug is enabled or insertable.  In practice, the live Destiny data will refer to these entries by index. You can then look up that index in the appropriate property (enabledRules or insertionRules) to get the localized string for the failure message if it failed.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsItemsDestinyPlugRuleDefinition {
  /// The localized string to show if this rule fails.
  #[serde(rename = "failureMessage")]
  failure_message: Option<String>
}

impl DestinyDefinitionsItemsDestinyPlugRuleDefinition {
  /// Dictates a rule around whether the plug is enabled or insertable.  In practice, the live Destiny data will refer to these entries by index. You can then look up that index in the appropriate property (enabledRules or insertionRules) to get the localized string for the failure message if it failed.
  pub fn new() -> DestinyDefinitionsItemsDestinyPlugRuleDefinition {
    DestinyDefinitionsItemsDestinyPlugRuleDefinition {
      failure_message: None
    }
  }

  pub fn set_failure_message(&mut self, failure_message: String) {
    self.failure_message = Some(failure_message);
  }

  pub fn with_failure_message(mut self, failure_message: String) -> DestinyDefinitionsItemsDestinyPlugRuleDefinition {
    self.failure_message = Some(failure_message);
    self
  }

  pub fn failure_message(&self) -> Option<&String> {
    self.failure_message.as_ref()
  }

  pub fn reset_failure_message(&mut self) {
    self.failure_message = None;
  }

}



