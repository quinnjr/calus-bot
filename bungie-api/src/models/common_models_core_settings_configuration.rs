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
pub struct CommonModelsCoreSettingsConfiguration {
  #[serde(rename = "environment")]
  environment: Option<String>,
  #[serde(rename = "systems")]
  systems: Option<::std::collections::HashMap<String, ::models::CommonModelsCoreSystem>>,
  #[serde(rename = "ignoreReasons")]
  ignore_reasons: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "forumCategories")]
  forum_categories: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "groupAvatars")]
  group_avatars: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "destinyMembershipTypes")]
  destiny_membership_types: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "recruitmentPlatformTags")]
  recruitment_platform_tags: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "recruitmentMiscTags")]
  recruitment_misc_tags: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "recruitmentActivities")]
  recruitment_activities: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "userContentLocales")]
  user_content_locales: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "systemContentLocales")]
  system_content_locales: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "clanBannerDecals")]
  clan_banner_decals: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "clanBannerDecalColors")]
  clan_banner_decal_colors: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "clanBannerGonfalons")]
  clan_banner_gonfalons: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "clanBannerGonfalonColors")]
  clan_banner_gonfalon_colors: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "clanBannerGonfalonDetails")]
  clan_banner_gonfalon_details: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "clanBannerGonfalonDetailColors")]
  clan_banner_gonfalon_detail_colors: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "clanBannerStandards")]
  clan_banner_standards: Option<Vec<::models::CommonModelsCoreSetting>>,
  #[serde(rename = "destiny2CoreSettings")]
  destiny2_core_settings: Option<::models::CommonModelsDestiny2CoreSettings>,
  #[serde(rename = "emailSettings")]
  email_settings: Option<::models::UserEmailSettings>
}

impl CommonModelsCoreSettingsConfiguration {
  pub fn new() -> CommonModelsCoreSettingsConfiguration {
    CommonModelsCoreSettingsConfiguration {
      environment: None,
      systems: None,
      ignore_reasons: None,
      forum_categories: None,
      group_avatars: None,
      destiny_membership_types: None,
      recruitment_platform_tags: None,
      recruitment_misc_tags: None,
      recruitment_activities: None,
      user_content_locales: None,
      system_content_locales: None,
      clan_banner_decals: None,
      clan_banner_decal_colors: None,
      clan_banner_gonfalons: None,
      clan_banner_gonfalon_colors: None,
      clan_banner_gonfalon_details: None,
      clan_banner_gonfalon_detail_colors: None,
      clan_banner_standards: None,
      destiny2_core_settings: None,
      email_settings: None
    }
  }

  pub fn set_environment(&mut self, environment: String) {
    self.environment = Some(environment);
  }

  pub fn with_environment(mut self, environment: String) -> CommonModelsCoreSettingsConfiguration {
    self.environment = Some(environment);
    self
  }

  pub fn environment(&self) -> Option<&String> {
    self.environment.as_ref()
  }

  pub fn reset_environment(&mut self) {
    self.environment = None;
  }

  pub fn set_systems(&mut self, systems: ::std::collections::HashMap<String, ::models::CommonModelsCoreSystem>) {
    self.systems = Some(systems);
  }

  pub fn with_systems(mut self, systems: ::std::collections::HashMap<String, ::models::CommonModelsCoreSystem>) -> CommonModelsCoreSettingsConfiguration {
    self.systems = Some(systems);
    self
  }

  pub fn systems(&self) -> Option<&::std::collections::HashMap<String, ::models::CommonModelsCoreSystem>> {
    self.systems.as_ref()
  }

  pub fn reset_systems(&mut self) {
    self.systems = None;
  }

  pub fn set_ignore_reasons(&mut self, ignore_reasons: Vec<::models::CommonModelsCoreSetting>) {
    self.ignore_reasons = Some(ignore_reasons);
  }

  pub fn with_ignore_reasons(mut self, ignore_reasons: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.ignore_reasons = Some(ignore_reasons);
    self
  }

  pub fn ignore_reasons(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.ignore_reasons.as_ref()
  }

  pub fn reset_ignore_reasons(&mut self) {
    self.ignore_reasons = None;
  }

  pub fn set_forum_categories(&mut self, forum_categories: Vec<::models::CommonModelsCoreSetting>) {
    self.forum_categories = Some(forum_categories);
  }

  pub fn with_forum_categories(mut self, forum_categories: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.forum_categories = Some(forum_categories);
    self
  }

  pub fn forum_categories(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.forum_categories.as_ref()
  }

  pub fn reset_forum_categories(&mut self) {
    self.forum_categories = None;
  }

  pub fn set_group_avatars(&mut self, group_avatars: Vec<::models::CommonModelsCoreSetting>) {
    self.group_avatars = Some(group_avatars);
  }

  pub fn with_group_avatars(mut self, group_avatars: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.group_avatars = Some(group_avatars);
    self
  }

  pub fn group_avatars(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.group_avatars.as_ref()
  }

  pub fn reset_group_avatars(&mut self) {
    self.group_avatars = None;
  }

  pub fn set_destiny_membership_types(&mut self, destiny_membership_types: Vec<::models::CommonModelsCoreSetting>) {
    self.destiny_membership_types = Some(destiny_membership_types);
  }

  pub fn with_destiny_membership_types(mut self, destiny_membership_types: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.destiny_membership_types = Some(destiny_membership_types);
    self
  }

  pub fn destiny_membership_types(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.destiny_membership_types.as_ref()
  }

  pub fn reset_destiny_membership_types(&mut self) {
    self.destiny_membership_types = None;
  }

  pub fn set_recruitment_platform_tags(&mut self, recruitment_platform_tags: Vec<::models::CommonModelsCoreSetting>) {
    self.recruitment_platform_tags = Some(recruitment_platform_tags);
  }

  pub fn with_recruitment_platform_tags(mut self, recruitment_platform_tags: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.recruitment_platform_tags = Some(recruitment_platform_tags);
    self
  }

  pub fn recruitment_platform_tags(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.recruitment_platform_tags.as_ref()
  }

  pub fn reset_recruitment_platform_tags(&mut self) {
    self.recruitment_platform_tags = None;
  }

  pub fn set_recruitment_misc_tags(&mut self, recruitment_misc_tags: Vec<::models::CommonModelsCoreSetting>) {
    self.recruitment_misc_tags = Some(recruitment_misc_tags);
  }

  pub fn with_recruitment_misc_tags(mut self, recruitment_misc_tags: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.recruitment_misc_tags = Some(recruitment_misc_tags);
    self
  }

  pub fn recruitment_misc_tags(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.recruitment_misc_tags.as_ref()
  }

  pub fn reset_recruitment_misc_tags(&mut self) {
    self.recruitment_misc_tags = None;
  }

  pub fn set_recruitment_activities(&mut self, recruitment_activities: Vec<::models::CommonModelsCoreSetting>) {
    self.recruitment_activities = Some(recruitment_activities);
  }

  pub fn with_recruitment_activities(mut self, recruitment_activities: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.recruitment_activities = Some(recruitment_activities);
    self
  }

  pub fn recruitment_activities(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.recruitment_activities.as_ref()
  }

  pub fn reset_recruitment_activities(&mut self) {
    self.recruitment_activities = None;
  }

  pub fn set_user_content_locales(&mut self, user_content_locales: Vec<::models::CommonModelsCoreSetting>) {
    self.user_content_locales = Some(user_content_locales);
  }

  pub fn with_user_content_locales(mut self, user_content_locales: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.user_content_locales = Some(user_content_locales);
    self
  }

  pub fn user_content_locales(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.user_content_locales.as_ref()
  }

  pub fn reset_user_content_locales(&mut self) {
    self.user_content_locales = None;
  }

  pub fn set_system_content_locales(&mut self, system_content_locales: Vec<::models::CommonModelsCoreSetting>) {
    self.system_content_locales = Some(system_content_locales);
  }

  pub fn with_system_content_locales(mut self, system_content_locales: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.system_content_locales = Some(system_content_locales);
    self
  }

  pub fn system_content_locales(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.system_content_locales.as_ref()
  }

  pub fn reset_system_content_locales(&mut self) {
    self.system_content_locales = None;
  }

  pub fn set_clan_banner_decals(&mut self, clan_banner_decals: Vec<::models::CommonModelsCoreSetting>) {
    self.clan_banner_decals = Some(clan_banner_decals);
  }

  pub fn with_clan_banner_decals(mut self, clan_banner_decals: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.clan_banner_decals = Some(clan_banner_decals);
    self
  }

  pub fn clan_banner_decals(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.clan_banner_decals.as_ref()
  }

  pub fn reset_clan_banner_decals(&mut self) {
    self.clan_banner_decals = None;
  }

  pub fn set_clan_banner_decal_colors(&mut self, clan_banner_decal_colors: Vec<::models::CommonModelsCoreSetting>) {
    self.clan_banner_decal_colors = Some(clan_banner_decal_colors);
  }

  pub fn with_clan_banner_decal_colors(mut self, clan_banner_decal_colors: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.clan_banner_decal_colors = Some(clan_banner_decal_colors);
    self
  }

  pub fn clan_banner_decal_colors(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.clan_banner_decal_colors.as_ref()
  }

  pub fn reset_clan_banner_decal_colors(&mut self) {
    self.clan_banner_decal_colors = None;
  }

  pub fn set_clan_banner_gonfalons(&mut self, clan_banner_gonfalons: Vec<::models::CommonModelsCoreSetting>) {
    self.clan_banner_gonfalons = Some(clan_banner_gonfalons);
  }

  pub fn with_clan_banner_gonfalons(mut self, clan_banner_gonfalons: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.clan_banner_gonfalons = Some(clan_banner_gonfalons);
    self
  }

  pub fn clan_banner_gonfalons(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.clan_banner_gonfalons.as_ref()
  }

  pub fn reset_clan_banner_gonfalons(&mut self) {
    self.clan_banner_gonfalons = None;
  }

  pub fn set_clan_banner_gonfalon_colors(&mut self, clan_banner_gonfalon_colors: Vec<::models::CommonModelsCoreSetting>) {
    self.clan_banner_gonfalon_colors = Some(clan_banner_gonfalon_colors);
  }

  pub fn with_clan_banner_gonfalon_colors(mut self, clan_banner_gonfalon_colors: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.clan_banner_gonfalon_colors = Some(clan_banner_gonfalon_colors);
    self
  }

  pub fn clan_banner_gonfalon_colors(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.clan_banner_gonfalon_colors.as_ref()
  }

  pub fn reset_clan_banner_gonfalon_colors(&mut self) {
    self.clan_banner_gonfalon_colors = None;
  }

  pub fn set_clan_banner_gonfalon_details(&mut self, clan_banner_gonfalon_details: Vec<::models::CommonModelsCoreSetting>) {
    self.clan_banner_gonfalon_details = Some(clan_banner_gonfalon_details);
  }

  pub fn with_clan_banner_gonfalon_details(mut self, clan_banner_gonfalon_details: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.clan_banner_gonfalon_details = Some(clan_banner_gonfalon_details);
    self
  }

  pub fn clan_banner_gonfalon_details(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.clan_banner_gonfalon_details.as_ref()
  }

  pub fn reset_clan_banner_gonfalon_details(&mut self) {
    self.clan_banner_gonfalon_details = None;
  }

  pub fn set_clan_banner_gonfalon_detail_colors(&mut self, clan_banner_gonfalon_detail_colors: Vec<::models::CommonModelsCoreSetting>) {
    self.clan_banner_gonfalon_detail_colors = Some(clan_banner_gonfalon_detail_colors);
  }

  pub fn with_clan_banner_gonfalon_detail_colors(mut self, clan_banner_gonfalon_detail_colors: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.clan_banner_gonfalon_detail_colors = Some(clan_banner_gonfalon_detail_colors);
    self
  }

  pub fn clan_banner_gonfalon_detail_colors(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.clan_banner_gonfalon_detail_colors.as_ref()
  }

  pub fn reset_clan_banner_gonfalon_detail_colors(&mut self) {
    self.clan_banner_gonfalon_detail_colors = None;
  }

  pub fn set_clan_banner_standards(&mut self, clan_banner_standards: Vec<::models::CommonModelsCoreSetting>) {
    self.clan_banner_standards = Some(clan_banner_standards);
  }

  pub fn with_clan_banner_standards(mut self, clan_banner_standards: Vec<::models::CommonModelsCoreSetting>) -> CommonModelsCoreSettingsConfiguration {
    self.clan_banner_standards = Some(clan_banner_standards);
    self
  }

  pub fn clan_banner_standards(&self) -> Option<&Vec<::models::CommonModelsCoreSetting>> {
    self.clan_banner_standards.as_ref()
  }

  pub fn reset_clan_banner_standards(&mut self) {
    self.clan_banner_standards = None;
  }

  pub fn set_destiny2_core_settings(&mut self, destiny2_core_settings: ::models::CommonModelsDestiny2CoreSettings) {
    self.destiny2_core_settings = Some(destiny2_core_settings);
  }

  pub fn with_destiny2_core_settings(mut self, destiny2_core_settings: ::models::CommonModelsDestiny2CoreSettings) -> CommonModelsCoreSettingsConfiguration {
    self.destiny2_core_settings = Some(destiny2_core_settings);
    self
  }

  pub fn destiny2_core_settings(&self) -> Option<&::models::CommonModelsDestiny2CoreSettings> {
    self.destiny2_core_settings.as_ref()
  }

  pub fn reset_destiny2_core_settings(&mut self) {
    self.destiny2_core_settings = None;
  }

  pub fn set_email_settings(&mut self, email_settings: ::models::UserEmailSettings) {
    self.email_settings = Some(email_settings);
  }

  pub fn with_email_settings(mut self, email_settings: ::models::UserEmailSettings) -> CommonModelsCoreSettingsConfiguration {
    self.email_settings = Some(email_settings);
    self
  }

  pub fn email_settings(&self) -> Option<&::models::UserEmailSettings> {
    self.email_settings.as_ref()
  }

  pub fn reset_email_settings(&mut self) {
    self.email_settings = None;
  }

}



