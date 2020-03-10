/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsMilestonesDestinyMilestoneDefinition : Milestones are an in-game concept where they're attempting to tell you what you can do next in-game.  If that sounds a lot like Advisors in Destiny 1, it is! So we threw out Advisors in the Destiny 2 API and tacked all of the data we would have put on Advisors onto Milestones instead.  Each Milestone represents something going on in the game right now:  - A \"ritual activity\" you can perform, like nightfall  - A \"special event\" that may have activities related to it, like Taco Tuesday (there's no Taco Tuesday in Destiny 2)  - A checklist you can fulfill, like helping your Clan complete all of its weekly objectives  - A tutorial quest you can play through, like the introduction to the Crucible.  Most of these milestones appear in game as well. Some of them are BNet only, because we're so extra. You're welcome.  There are some important caveats to understand about how we currently render Milestones and their deficiencies. The game currently doesn't have any content that actually tells you oughtright *what* the Milestone is: that is to say, what you'll be doing. The best we get is either a description of the overall Milestone, or of the Quest that the Milestone is having you partake in: which is usually something that assumes you already know what it's talking about, like \"Complete 5 Challenges\". 5 Challenges for what? What's a challenge? These are not questions that the Milestone data will answer for you unfortunately.  This isn't great, and in the future I'd like to add some custom text to give you more contextual information to pass on to your users. But for now, you can do what we do to render what little display info we do have:  Start by looking at the currently active quest (ideally, you've fetched DestinyMilestone or DestinyPublicMilestone data from the API, so you know the currently active quest for the Milestone in question). Look up the Quests property in the Milestone Definition, and check if it has display properties. If it does, show that as the description of the Milestone. If it doesn't, fall back on the Milestone's description.  This approach will let you avoid, whenever possible, the even less useful (and sometimes nonexistant) milestone-level names and descriptions.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
  #[serde(rename = "displayProperties")]
  display_properties: Option<::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition>,
  /// A hint to the UI to indicate what to show as the display properties for this Milestone when showing \"Live\" milestone data. Feel free to show more than this if desired: this hint is meant to simplify our own UI, but it may prove useful to you as well.
  #[serde(rename = "displayPreference")]
  display_preference: Option<i32>,
  /// A custom image someone made just for the milestone. Isn't that special?
  #[serde(rename = "image")]
  image: Option<String>,
  /// An enumeration listing one of the possible types of milestones. Check out the DestinyMilestoneType enum for more info!
  #[serde(rename = "milestoneType")]
  milestone_type: Option<i32>,
  /// If True, then the Milestone has been integrated with BNet's recruiting feature.
  #[serde(rename = "recruitable")]
  recruitable: Option<bool>,
  /// If the milestone has a friendly identifier for association with other features - such as Recruiting - that identifier can be found here. This is \"friendly\" in that it looks better in a URL than whatever the identifier for the Milestone actually is.
  #[serde(rename = "friendlyName")]
  friendly_name: Option<String>,
  /// If TRUE, this entry should be returned in the list of milestones for the \"Explore Destiny\" (i.e. new BNet homepage) features of Bungie.net (as long as the underlying event is active) Note that this is a property specifically used by BNet and the companion app for the \"Live Events\" feature of the front page/welcome view: it's not a reflection of what you see in-game.
  #[serde(rename = "showInExplorer")]
  show_in_explorer: Option<bool>,
  /// Determines whether we'll show this Milestone in the user's personal Milestones list.
  #[serde(rename = "showInMilestones")]
  show_in_milestones: Option<bool>,
  /// If TRUE, \"Explore Destiny\" (the front page of BNet and the companion app) prioritize using the activity image over any overriding Quest or Milestone image provided. This unfortunate hack is brought to you by Trials of The Nine.
  #[serde(rename = "explorePrioritizesActivityImage")]
  explore_prioritizes_activity_image: Option<bool>,
  /// A shortcut for clients - and the server - to understand whether we can predict the start and end dates for this event. In practice, there are multiple ways that an event could have predictable date ranges, but not all events will be able to be predicted via any mechanism (for instance, events that are manually triggered on and off)
  #[serde(rename = "hasPredictableDates")]
  has_predictable_dates: Option<bool>,
  /// The full set of possible Quests that give the overview of the Milestone event/activity in question. Only one of these can be active at a time for a given Conceptual Milestone, but many of them may be \"available\" for the user to choose from. (for instance, with Milestones you can choose from the three available Quests, but only one can be active at a time) Keyed by the quest item.  As of Forsaken (~September 2018), Quest-style Milestones are being removed for many types of activities. There will likely be further revisions to the Milestone concept in the future.
  #[serde(rename = "quests")]
  quests: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneQuestDefinition>>,
  /// If this milestone can provide rewards, this will define the categories into which the individual reward entries are placed.  This is keyed by the Category's hash, which is only guaranteed to be unique within a given Milestone.
  #[serde(rename = "rewards")]
  rewards: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition>>,
  /// If you're going to show Vendors for the Milestone, you can use this as a localized \"header\" for the section where you show that vendor data. It'll provide a more context-relevant clue about what the vendor's role is in the Milestone.
  #[serde(rename = "vendorsDisplayTitle")]
  vendors_display_title: Option<String>,
  /// Sometimes, milestones will have rewards provided by Vendors. This definition gives the information needed to understand which vendors are relevant, the order in which they should be returned if order matters, and the conditions under which the Vendor is relevant to the user.
  #[serde(rename = "vendors")]
  vendors: Option<Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneVendorDefinition>>,
  /// Sometimes, milestones will have arbitrary values associated with them that are of interest to us or to third party developers. This is the collection of those values' definitions, keyed by the identifier of the value and providing useful definition information such as localizable names and descriptions for the value.
  #[serde(rename = "values")]
  values: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneValueDefinition>>,
  /// Some milestones are explicit objectives that you can see and interact with in the game. Some milestones are more conceptual, built by BNet to help advise you on activities and events that happen in-game but that aren't explicitly shown in game as Milestones. If this is TRUE, you can see this as a milestone in the game. If this is FALSE, it's an event or activity you can participate in, but you won't see it as a Milestone in the game's UI.
  #[serde(rename = "isInGameMilestone")]
  is_in_game_milestone: Option<bool>,
  /// A Milestone can now be represented by one or more activities directly (without a backing Quest), and that activity can have many challenges, modifiers, and related to it.
  #[serde(rename = "activities")]
  activities: Option<Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition>>,
  #[serde(rename = "defaultOrder")]
  default_order: Option<i32>,
  /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// The index of the entity as it was found in the investment tables.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
  #[serde(rename = "redacted")]
  redacted: Option<bool>
}

impl DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
  /// Milestones are an in-game concept where they're attempting to tell you what you can do next in-game.  If that sounds a lot like Advisors in Destiny 1, it is! So we threw out Advisors in the Destiny 2 API and tacked all of the data we would have put on Advisors onto Milestones instead.  Each Milestone represents something going on in the game right now:  - A \"ritual activity\" you can perform, like nightfall  - A \"special event\" that may have activities related to it, like Taco Tuesday (there's no Taco Tuesday in Destiny 2)  - A checklist you can fulfill, like helping your Clan complete all of its weekly objectives  - A tutorial quest you can play through, like the introduction to the Crucible.  Most of these milestones appear in game as well. Some of them are BNet only, because we're so extra. You're welcome.  There are some important caveats to understand about how we currently render Milestones and their deficiencies. The game currently doesn't have any content that actually tells you oughtright *what* the Milestone is: that is to say, what you'll be doing. The best we get is either a description of the overall Milestone, or of the Quest that the Milestone is having you partake in: which is usually something that assumes you already know what it's talking about, like \"Complete 5 Challenges\". 5 Challenges for what? What's a challenge? These are not questions that the Milestone data will answer for you unfortunately.  This isn't great, and in the future I'd like to add some custom text to give you more contextual information to pass on to your users. But for now, you can do what we do to render what little display info we do have:  Start by looking at the currently active quest (ideally, you've fetched DestinyMilestone or DestinyPublicMilestone data from the API, so you know the currently active quest for the Milestone in question). Look up the Quests property in the Milestone Definition, and check if it has display properties. If it does, show that as the description of the Milestone. If it doesn't, fall back on the Milestone's description.  This approach will let you avoid, whenever possible, the even less useful (and sometimes nonexistant) milestone-level names and descriptions.
  pub fn new() -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
      display_properties: None,
      display_preference: None,
      image: None,
      milestone_type: None,
      recruitable: None,
      friendly_name: None,
      show_in_explorer: None,
      show_in_milestones: None,
      explore_prioritizes_activity_image: None,
      has_predictable_dates: None,
      quests: None,
      rewards: None,
      vendors_display_title: None,
      vendors: None,
      values: None,
      is_in_game_milestone: None,
      activities: None,
      default_order: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_display_preference(&mut self, display_preference: i32) {
    self.display_preference = Some(display_preference);
  }

  pub fn with_display_preference(mut self, display_preference: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.display_preference = Some(display_preference);
    self
  }

  pub fn display_preference(&self) -> Option<&i32> {
    self.display_preference.as_ref()
  }

  pub fn reset_display_preference(&mut self) {
    self.display_preference = None;
  }

  pub fn set_image(&mut self, image: String) {
    self.image = Some(image);
  }

  pub fn with_image(mut self, image: String) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.image = Some(image);
    self
  }

  pub fn image(&self) -> Option<&String> {
    self.image.as_ref()
  }

  pub fn reset_image(&mut self) {
    self.image = None;
  }

  pub fn set_milestone_type(&mut self, milestone_type: i32) {
    self.milestone_type = Some(milestone_type);
  }

  pub fn with_milestone_type(mut self, milestone_type: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.milestone_type = Some(milestone_type);
    self
  }

  pub fn milestone_type(&self) -> Option<&i32> {
    self.milestone_type.as_ref()
  }

  pub fn reset_milestone_type(&mut self) {
    self.milestone_type = None;
  }

  pub fn set_recruitable(&mut self, recruitable: bool) {
    self.recruitable = Some(recruitable);
  }

  pub fn with_recruitable(mut self, recruitable: bool) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.recruitable = Some(recruitable);
    self
  }

  pub fn recruitable(&self) -> Option<&bool> {
    self.recruitable.as_ref()
  }

  pub fn reset_recruitable(&mut self) {
    self.recruitable = None;
  }

  pub fn set_friendly_name(&mut self, friendly_name: String) {
    self.friendly_name = Some(friendly_name);
  }

  pub fn with_friendly_name(mut self, friendly_name: String) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.friendly_name = Some(friendly_name);
    self
  }

  pub fn friendly_name(&self) -> Option<&String> {
    self.friendly_name.as_ref()
  }

  pub fn reset_friendly_name(&mut self) {
    self.friendly_name = None;
  }

  pub fn set_show_in_explorer(&mut self, show_in_explorer: bool) {
    self.show_in_explorer = Some(show_in_explorer);
  }

  pub fn with_show_in_explorer(mut self, show_in_explorer: bool) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.show_in_explorer = Some(show_in_explorer);
    self
  }

  pub fn show_in_explorer(&self) -> Option<&bool> {
    self.show_in_explorer.as_ref()
  }

  pub fn reset_show_in_explorer(&mut self) {
    self.show_in_explorer = None;
  }

  pub fn set_show_in_milestones(&mut self, show_in_milestones: bool) {
    self.show_in_milestones = Some(show_in_milestones);
  }

  pub fn with_show_in_milestones(mut self, show_in_milestones: bool) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.show_in_milestones = Some(show_in_milestones);
    self
  }

  pub fn show_in_milestones(&self) -> Option<&bool> {
    self.show_in_milestones.as_ref()
  }

  pub fn reset_show_in_milestones(&mut self) {
    self.show_in_milestones = None;
  }

  pub fn set_explore_prioritizes_activity_image(&mut self, explore_prioritizes_activity_image: bool) {
    self.explore_prioritizes_activity_image = Some(explore_prioritizes_activity_image);
  }

  pub fn with_explore_prioritizes_activity_image(mut self, explore_prioritizes_activity_image: bool) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.explore_prioritizes_activity_image = Some(explore_prioritizes_activity_image);
    self
  }

  pub fn explore_prioritizes_activity_image(&self) -> Option<&bool> {
    self.explore_prioritizes_activity_image.as_ref()
  }

  pub fn reset_explore_prioritizes_activity_image(&mut self) {
    self.explore_prioritizes_activity_image = None;
  }

  pub fn set_has_predictable_dates(&mut self, has_predictable_dates: bool) {
    self.has_predictable_dates = Some(has_predictable_dates);
  }

  pub fn with_has_predictable_dates(mut self, has_predictable_dates: bool) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.has_predictable_dates = Some(has_predictable_dates);
    self
  }

  pub fn has_predictable_dates(&self) -> Option<&bool> {
    self.has_predictable_dates.as_ref()
  }

  pub fn reset_has_predictable_dates(&mut self) {
    self.has_predictable_dates = None;
  }

  pub fn set_quests(&mut self, quests: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneQuestDefinition>) {
    self.quests = Some(quests);
  }

  pub fn with_quests(mut self, quests: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneQuestDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.quests = Some(quests);
    self
  }

  pub fn quests(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneQuestDefinition>> {
    self.quests.as_ref()
  }

  pub fn reset_quests(&mut self) {
    self.quests = None;
  }

  pub fn set_rewards(&mut self, rewards: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition>) {
    self.rewards = Some(rewards);
  }

  pub fn with_rewards(mut self, rewards: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.rewards = Some(rewards);
    self
  }

  pub fn rewards(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition>> {
    self.rewards.as_ref()
  }

  pub fn reset_rewards(&mut self) {
    self.rewards = None;
  }

  pub fn set_vendors_display_title(&mut self, vendors_display_title: String) {
    self.vendors_display_title = Some(vendors_display_title);
  }

  pub fn with_vendors_display_title(mut self, vendors_display_title: String) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.vendors_display_title = Some(vendors_display_title);
    self
  }

  pub fn vendors_display_title(&self) -> Option<&String> {
    self.vendors_display_title.as_ref()
  }

  pub fn reset_vendors_display_title(&mut self) {
    self.vendors_display_title = None;
  }

  pub fn set_vendors(&mut self, vendors: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneVendorDefinition>) {
    self.vendors = Some(vendors);
  }

  pub fn with_vendors(mut self, vendors: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneVendorDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.vendors = Some(vendors);
    self
  }

  pub fn vendors(&self) -> Option<&Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneVendorDefinition>> {
    self.vendors.as_ref()
  }

  pub fn reset_vendors(&mut self) {
    self.vendors = None;
  }

  pub fn set_values(&mut self, values: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneValueDefinition>) {
    self.values = Some(values);
  }

  pub fn with_values(mut self, values: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneValueDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.values = Some(values);
    self
  }

  pub fn values(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneValueDefinition>> {
    self.values.as_ref()
  }

  pub fn reset_values(&mut self) {
    self.values = None;
  }

  pub fn set_is_in_game_milestone(&mut self, is_in_game_milestone: bool) {
    self.is_in_game_milestone = Some(is_in_game_milestone);
  }

  pub fn with_is_in_game_milestone(mut self, is_in_game_milestone: bool) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.is_in_game_milestone = Some(is_in_game_milestone);
    self
  }

  pub fn is_in_game_milestone(&self) -> Option<&bool> {
    self.is_in_game_milestone.as_ref()
  }

  pub fn reset_is_in_game_milestone(&mut self) {
    self.is_in_game_milestone = None;
  }

  pub fn set_activities(&mut self, activities: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition>) {
    self.activities = Some(activities);
  }

  pub fn with_activities(mut self, activities: Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.activities = Some(activities);
    self
  }

  pub fn activities(&self) -> Option<&Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition>> {
    self.activities.as_ref()
  }

  pub fn reset_activities(&mut self) {
    self.activities = None;
  }

  pub fn set_default_order(&mut self, default_order: i32) {
    self.default_order = Some(default_order);
  }

  pub fn with_default_order(mut self, default_order: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.default_order = Some(default_order);
    self
  }

  pub fn default_order(&self) -> Option<&i32> {
    self.default_order.as_ref()
  }

  pub fn reset_default_order(&mut self) {
    self.default_order = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_redacted(&mut self, redacted: bool) {
    self.redacted = Some(redacted);
  }

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsMilestonesDestinyMilestoneDefinition {
    self.redacted = Some(redacted);
    self
  }

  pub fn redacted(&self) -> Option<&bool> {
    self.redacted.as_ref()
  }

  pub fn reset_redacted(&mut self) {
    self.redacted = None;
  }

}


