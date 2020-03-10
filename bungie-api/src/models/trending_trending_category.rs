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
pub struct TrendingTrendingCategory {
  #[serde(rename = "categoryName")]
  category_name: Option<String>,
  #[serde(rename = "entries")]
  entries: Option<::models::SearchResultOfTrendingEntry>,
  #[serde(rename = "categoryId")]
  category_id: Option<String>
}

impl TrendingTrendingCategory {
  pub fn new() -> TrendingTrendingCategory {
    TrendingTrendingCategory {
      category_name: None,
      entries: None,
      category_id: None
    }
  }

  pub fn set_category_name(&mut self, category_name: String) {
    self.category_name = Some(category_name);
  }

  pub fn with_category_name(mut self, category_name: String) -> TrendingTrendingCategory {
    self.category_name = Some(category_name);
    self
  }

  pub fn category_name(&self) -> Option<&String> {
    self.category_name.as_ref()
  }

  pub fn reset_category_name(&mut self) {
    self.category_name = None;
  }

  pub fn set_entries(&mut self, entries: ::models::SearchResultOfTrendingEntry) {
    self.entries = Some(entries);
  }

  pub fn with_entries(mut self, entries: ::models::SearchResultOfTrendingEntry) -> TrendingTrendingCategory {
    self.entries = Some(entries);
    self
  }

  pub fn entries(&self) -> Option<&::models::SearchResultOfTrendingEntry> {
    self.entries.as_ref()
  }

  pub fn reset_entries(&mut self) {
    self.entries = None;
  }

  pub fn set_category_id(&mut self, category_id: String) {
    self.category_id = Some(category_id);
  }

  pub fn with_category_id(mut self, category_id: String) -> TrendingTrendingCategory {
    self.category_id = Some(category_id);
    self
  }

  pub fn category_id(&self) -> Option<&String> {
    self.category_id.as_ref()
  }

  pub fn reset_category_id(&mut self) {
    self.category_id = None;
  }

}


