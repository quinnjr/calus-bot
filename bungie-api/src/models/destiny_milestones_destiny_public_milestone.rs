/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyMilestonesDestinyPublicMilestone : Information about milestones, presented in a character state-agnostic manner. Combine this data with DestinyMilestoneDefinition to get a full picture of the milestone, which is basically a checklist of things to do in the game. Think of this as GetPublicAdvisors 3.0, for those who used the Destiny 1 API.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyMilestonesDestinyPublicMilestone {
  /// The hash identifier for the milestone. Use it to look up the DestinyMilestoneDefinition for static data about the Milestone.
  #[serde(rename = "milestoneHash")]
  milestone_hash: Option<i32>,
  /// A milestone not need have even a single quest, but if there are active quests they will be returned here.
  #[serde(rename = "availableQuests")]
  available_quests: Option<Vec<::models::DestinyMilestonesDestinyPublicMilestoneQuest>>,
  #[serde(rename = "activities")]
  activities: Option<Vec<::models::DestinyMilestonesDestinyPublicMilestoneChallengeActivity>>,
  /// Sometimes milestones - or activities active in milestones - will have relevant vendors. These are the vendors that are currently relevant.  Deprecated, already, for the sake of the new \"vendors\" property that has more data. What was I thinking.
  #[serde(rename = "vendorHashes")]
  vendor_hashes: Option<Vec<i32>>,
  /// This is why we can't have nice things. This is the ordered list of vendors to be shown that relate to this milestone, potentially along with other interesting data.
  #[serde(rename = "vendors")]
  vendors: Option<Vec<::models::DestinyMilestonesDestinyPublicMilestoneVendor>>,
  /// If known, this is the date when the Milestone started/became active.
  #[serde(rename = "startDate")]
  start_date: Option<String>,
  /// If known, this is the date when the Milestone will expire/recycle/end.
  #[serde(rename = "endDate")]
  end_date: Option<String>,
  /// Used for ordering milestones in a display to match how we order them in BNet. May pull from static data, or possibly in the future from dynamic information.
  #[serde(rename = "order")]
  order: Option<i32>
}

impl DestinyMilestonesDestinyPublicMilestone {
  /// Information about milestones, presented in a character state-agnostic manner. Combine this data with DestinyMilestoneDefinition to get a full picture of the milestone, which is basically a checklist of things to do in the game. Think of this as GetPublicAdvisors 3.0, for those who used the Destiny 1 API.
  pub fn new() -> DestinyMilestonesDestinyPublicMilestone {
    DestinyMilestonesDestinyPublicMilestone {
      milestone_hash: None,
      available_quests: None,
      activities: None,
      vendor_hashes: None,
      vendors: None,
      start_date: None,
      end_date: None,
      order: None
    }
  }

  pub fn set_milestone_hash(&mut self, milestone_hash: i32) {
    self.milestone_hash = Some(milestone_hash);
  }

  pub fn with_milestone_hash(mut self, milestone_hash: i32) -> DestinyMilestonesDestinyPublicMilestone {
    self.milestone_hash = Some(milestone_hash);
    self
  }

  pub fn milestone_hash(&self) -> Option<&i32> {
    self.milestone_hash.as_ref()
  }

  pub fn reset_milestone_hash(&mut self) {
    self.milestone_hash = None;
  }

  pub fn set_available_quests(&mut self, available_quests: Vec<::models::DestinyMilestonesDestinyPublicMilestoneQuest>) {
    self.available_quests = Some(available_quests);
  }

  pub fn with_available_quests(mut self, available_quests: Vec<::models::DestinyMilestonesDestinyPublicMilestoneQuest>) -> DestinyMilestonesDestinyPublicMilestone {
    self.available_quests = Some(available_quests);
    self
  }

  pub fn available_quests(&self) -> Option<&Vec<::models::DestinyMilestonesDestinyPublicMilestoneQuest>> {
    self.available_quests.as_ref()
  }

  pub fn reset_available_quests(&mut self) {
    self.available_quests = None;
  }

  pub fn set_activities(&mut self, activities: Vec<::models::DestinyMilestonesDestinyPublicMilestoneChallengeActivity>) {
    self.activities = Some(activities);
  }

  pub fn with_activities(mut self, activities: Vec<::models::DestinyMilestonesDestinyPublicMilestoneChallengeActivity>) -> DestinyMilestonesDestinyPublicMilestone {
    self.activities = Some(activities);
    self
  }

  pub fn activities(&self) -> Option<&Vec<::models::DestinyMilestonesDestinyPublicMilestoneChallengeActivity>> {
    self.activities.as_ref()
  }

  pub fn reset_activities(&mut self) {
    self.activities = None;
  }

  pub fn set_vendor_hashes(&mut self, vendor_hashes: Vec<i32>) {
    self.vendor_hashes = Some(vendor_hashes);
  }

  pub fn with_vendor_hashes(mut self, vendor_hashes: Vec<i32>) -> DestinyMilestonesDestinyPublicMilestone {
    self.vendor_hashes = Some(vendor_hashes);
    self
  }

  pub fn vendor_hashes(&self) -> Option<&Vec<i32>> {
    self.vendor_hashes.as_ref()
  }

  pub fn reset_vendor_hashes(&mut self) {
    self.vendor_hashes = None;
  }

  pub fn set_vendors(&mut self, vendors: Vec<::models::DestinyMilestonesDestinyPublicMilestoneVendor>) {
    self.vendors = Some(vendors);
  }

  pub fn with_vendors(mut self, vendors: Vec<::models::DestinyMilestonesDestinyPublicMilestoneVendor>) -> DestinyMilestonesDestinyPublicMilestone {
    self.vendors = Some(vendors);
    self
  }

  pub fn vendors(&self) -> Option<&Vec<::models::DestinyMilestonesDestinyPublicMilestoneVendor>> {
    self.vendors.as_ref()
  }

  pub fn reset_vendors(&mut self) {
    self.vendors = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> DestinyMilestonesDestinyPublicMilestone {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_end_date(&mut self, end_date: String) {
    self.end_date = Some(end_date);
  }

  pub fn with_end_date(mut self, end_date: String) -> DestinyMilestonesDestinyPublicMilestone {
    self.end_date = Some(end_date);
    self
  }

  pub fn end_date(&self) -> Option<&String> {
    self.end_date.as_ref()
  }

  pub fn reset_end_date(&mut self) {
    self.end_date = None;
  }

  pub fn set_order(&mut self, order: i32) {
    self.order = Some(order);
  }

  pub fn with_order(mut self, order: i32) -> DestinyMilestonesDestinyPublicMilestone {
    self.order = Some(order);
    self
  }

  pub fn order(&self) -> Option<&i32> {
    self.order.as_ref()
  }

  pub fn reset_order(&mut self) {
    self.order = None;
  }

}



