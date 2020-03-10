/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyUnlockExpressionDefinition : Where the sausage gets made. Unlock Expressions are the foundation of the game's gating mechanics and investment-related restrictions. They can test Unlock Flags and Unlock Values for certain states, using a sufficient amount of logical operators such that unlock expressions are effectively Turing complete.  Use UnlockExpressionParser to evaluate expressions using an IUnlockContext parsed from Babel.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyUnlockExpressionDefinition {
  /// A shortcut for determining the most restrictive gating that this expression performs. See the DestinyGatingScope enum's documentation for more details.
  #[serde(rename = "scope")]
  scope: Option<i32>
}

impl DestinyDefinitionsDestinyUnlockExpressionDefinition {
  /// Where the sausage gets made. Unlock Expressions are the foundation of the game's gating mechanics and investment-related restrictions. They can test Unlock Flags and Unlock Values for certain states, using a sufficient amount of logical operators such that unlock expressions are effectively Turing complete.  Use UnlockExpressionParser to evaluate expressions using an IUnlockContext parsed from Babel.
  pub fn new() -> DestinyDefinitionsDestinyUnlockExpressionDefinition {
    DestinyDefinitionsDestinyUnlockExpressionDefinition {
      scope: None
    }
  }

  pub fn set_scope(&mut self, scope: i32) {
    self.scope = Some(scope);
  }

  pub fn with_scope(mut self, scope: i32) -> DestinyDefinitionsDestinyUnlockExpressionDefinition {
    self.scope = Some(scope);
    self
  }

  pub fn scope(&self) -> Option<&i32> {
    self.scope.as_ref()
  }

  pub fn reset_scope(&mut self) {
    self.scope = None;
  }

}



