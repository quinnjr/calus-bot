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
pub struct TrendingTrendingEntryCommunityCreation {
  #[serde(rename = "media")]
  media: Option<String>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "author")]
  author: Option<String>,
  #[serde(rename = "authorMembershipId")]
  author_membership_id: Option<i64>,
  #[serde(rename = "postId")]
  post_id: Option<i64>,
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "upvotes")]
  upvotes: Option<i32>
}

impl TrendingTrendingEntryCommunityCreation {
  pub fn new() -> TrendingTrendingEntryCommunityCreation {
    TrendingTrendingEntryCommunityCreation {
      media: None,
      title: None,
      author: None,
      author_membership_id: None,
      post_id: None,
      body: None,
      upvotes: None
    }
  }

  pub fn set_media(&mut self, media: String) {
    self.media = Some(media);
  }

  pub fn with_media(mut self, media: String) -> TrendingTrendingEntryCommunityCreation {
    self.media = Some(media);
    self
  }

  pub fn media(&self) -> Option<&String> {
    self.media.as_ref()
  }

  pub fn reset_media(&mut self) {
    self.media = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> TrendingTrendingEntryCommunityCreation {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_author(&mut self, author: String) {
    self.author = Some(author);
  }

  pub fn with_author(mut self, author: String) -> TrendingTrendingEntryCommunityCreation {
    self.author = Some(author);
    self
  }

  pub fn author(&self) -> Option<&String> {
    self.author.as_ref()
  }

  pub fn reset_author(&mut self) {
    self.author = None;
  }

  pub fn set_author_membership_id(&mut self, author_membership_id: i64) {
    self.author_membership_id = Some(author_membership_id);
  }

  pub fn with_author_membership_id(mut self, author_membership_id: i64) -> TrendingTrendingEntryCommunityCreation {
    self.author_membership_id = Some(author_membership_id);
    self
  }

  pub fn author_membership_id(&self) -> Option<&i64> {
    self.author_membership_id.as_ref()
  }

  pub fn reset_author_membership_id(&mut self) {
    self.author_membership_id = None;
  }

  pub fn set_post_id(&mut self, post_id: i64) {
    self.post_id = Some(post_id);
  }

  pub fn with_post_id(mut self, post_id: i64) -> TrendingTrendingEntryCommunityCreation {
    self.post_id = Some(post_id);
    self
  }

  pub fn post_id(&self) -> Option<&i64> {
    self.post_id.as_ref()
  }

  pub fn reset_post_id(&mut self) {
    self.post_id = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> TrendingTrendingEntryCommunityCreation {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_upvotes(&mut self, upvotes: i32) {
    self.upvotes = Some(upvotes);
  }

  pub fn with_upvotes(mut self, upvotes: i32) -> TrendingTrendingEntryCommunityCreation {
    self.upvotes = Some(upvotes);
    self
  }

  pub fn upvotes(&self) -> Option<&i32> {
    self.upvotes.as_ref()
  }

  pub fn reset_upvotes(&mut self) {
    self.upvotes = None;
  }

}



