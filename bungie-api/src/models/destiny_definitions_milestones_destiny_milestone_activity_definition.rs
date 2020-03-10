/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsMilestonesDestinyMilestoneActivityDefinition : Milestones can have associated activities which provide additional information about the context, challenges, modifiers, state etc... related to this Milestone.   Information we need to be able to return that data is defined here, along with Tier data to establish a relationship between a conceptual Activity and its difficulty levels and variants.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsMilestonesDestinyMilestoneActivityDefinition {
  /// The \"Conceptual\" activity hash. Basically, we picked the lowest level activity and are treating it as the canonical definition of the activity for rendering purposes.  If you care about the specific difficulty modes and variations, use the activities under \"Variants\".
  #[serde(rename = "conceptualActivityHash")]
  conceptual_activity_hash: Option<i32>,
  /// A milestone-referenced activity can have many variants, such as Tiers or alternative modes of play.  Even if there is only a single variant, the details for these are represented within as a variant definition.  It is assumed that, if this DestinyMilestoneActivityDefinition is active, then all variants should be active.  If a Milestone could ever split the variants' active status conditionally, they should all have their own DestinyMilestoneActivityDefinition instead! The potential duplication will be worth it for the obviousness of processing and use.
  #[serde(rename = "variants")]
  variants: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneActivityVariantDefinition>>
}

impl DestinyDefinitionsMilestonesDestinyMilestoneActivityDefinition {
  /// Milestones can have associated activities which provide additional information about the context, challenges, modifiers, state etc... related to this Milestone.   Information we need to be able to return that data is defined here, along with Tier data to establish a relationship between a conceptual Activity and its difficulty levels and variants.
  pub fn new() -> DestinyDefinitionsMilestonesDestinyMilestoneActivityDefinition {
    DestinyDefinitionsMilestonesDestinyMilestoneActivityDefinition {
      conceptual_activity_hash: None,
      variants: None
    }
  }

  pub fn set_conceptual_activity_hash(&mut self, conceptual_activity_hash: i32) {
    self.conceptual_activity_hash = Some(conceptual_activity_hash);
  }

  pub fn with_conceptual_activity_hash(mut self, conceptual_activity_hash: i32) -> DestinyDefinitionsMilestonesDestinyMilestoneActivityDefinition {
    self.conceptual_activity_hash = Some(conceptual_activity_hash);
    self
  }

  pub fn conceptual_activity_hash(&self) -> Option<&i32> {
    self.conceptual_activity_hash.as_ref()
  }

  pub fn reset_conceptual_activity_hash(&mut self) {
    self.conceptual_activity_hash = None;
  }

  pub fn set_variants(&mut self, variants: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneActivityVariantDefinition>) {
    self.variants = Some(variants);
  }

  pub fn with_variants(mut self, variants: ::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneActivityVariantDefinition>) -> DestinyDefinitionsMilestonesDestinyMilestoneActivityDefinition {
    self.variants = Some(variants);
    self
  }

  pub fn variants(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneActivityVariantDefinition>> {
    self.variants.as_ref()
  }

  pub fn reset_variants(&mut self) {
    self.variants = None;
  }

}


