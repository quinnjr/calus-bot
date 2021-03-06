/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.7.1
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyConfigDestinyManifest : DestinyManifest is the external-facing contract for just the properties needed by those calling the Destiny Platform.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyConfigDestinyManifest {
  #[serde(rename = "version")]
  version: Option<String>,
  #[serde(rename = "mobileAssetContentPath")]
  mobile_asset_content_path: Option<String>,
  #[serde(rename = "mobileGearAssetDataBases")]
  mobile_gear_asset_data_bases: Option<Vec<::models::DestinyConfigGearAssetDataBaseDefinition>>,
  #[serde(rename = "mobileWorldContentPaths")]
  mobile_world_content_paths: Option<::std::collections::HashMap<String, String>>,
  /// This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a path to the aggregated world definitions (warning: large file!)
  #[serde(rename = "jsonWorldContentPaths")]
  json_world_content_paths: Option<::std::collections::HashMap<String, String>>,
  /// This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a dictionary, where the key is a definition type by name, and the value is the path to the file for that definition. WARNING: This is unsafe and subject to change - do not depend on data in these files staying around long-term.
  #[serde(rename = "jsonWorldComponentContentPaths")]
  json_world_component_content_paths: Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
  #[serde(rename = "mobileClanBannerDatabasePath")]
  mobile_clan_banner_database_path: Option<String>,
  #[serde(rename = "mobileGearCDN")]
  mobile_gear_cdn: Option<::std::collections::HashMap<String, String>>,
  /// Information about the \"Image Pyramid\" for Destiny icons. Where possible, we create smaller versions of Destiny icons. These are found as subfolders under the location of the \"original/full size\" Destiny images, with the same file name and extension as the original image itself. (this lets us avoid sending largely redundant path info with every entity, at the expense of the smaller versions of the image being less discoverable)
  #[serde(rename = "iconImagePyramidInfo")]
  icon_image_pyramid_info: Option<Vec<::models::DestinyConfigImagePyramidEntry>>
}

impl DestinyConfigDestinyManifest {
  /// DestinyManifest is the external-facing contract for just the properties needed by those calling the Destiny Platform.
  pub fn new() -> DestinyConfigDestinyManifest {
    DestinyConfigDestinyManifest {
      version: None,
      mobile_asset_content_path: None,
      mobile_gear_asset_data_bases: None,
      mobile_world_content_paths: None,
      json_world_content_paths: None,
      json_world_component_content_paths: None,
      mobile_clan_banner_database_path: None,
      mobile_gear_cdn: None,
      icon_image_pyramid_info: None
    }
  }

  pub fn set_version(&mut self, version: String) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: String) -> DestinyConfigDestinyManifest {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&String> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

  pub fn set_mobile_asset_content_path(&mut self, mobile_asset_content_path: String) {
    self.mobile_asset_content_path = Some(mobile_asset_content_path);
  }

  pub fn with_mobile_asset_content_path(mut self, mobile_asset_content_path: String) -> DestinyConfigDestinyManifest {
    self.mobile_asset_content_path = Some(mobile_asset_content_path);
    self
  }

  pub fn mobile_asset_content_path(&self) -> Option<&String> {
    self.mobile_asset_content_path.as_ref()
  }

  pub fn reset_mobile_asset_content_path(&mut self) {
    self.mobile_asset_content_path = None;
  }

  pub fn set_mobile_gear_asset_data_bases(&mut self, mobile_gear_asset_data_bases: Vec<::models::DestinyConfigGearAssetDataBaseDefinition>) {
    self.mobile_gear_asset_data_bases = Some(mobile_gear_asset_data_bases);
  }

  pub fn with_mobile_gear_asset_data_bases(mut self, mobile_gear_asset_data_bases: Vec<::models::DestinyConfigGearAssetDataBaseDefinition>) -> DestinyConfigDestinyManifest {
    self.mobile_gear_asset_data_bases = Some(mobile_gear_asset_data_bases);
    self
  }

  pub fn mobile_gear_asset_data_bases(&self) -> Option<&Vec<::models::DestinyConfigGearAssetDataBaseDefinition>> {
    self.mobile_gear_asset_data_bases.as_ref()
  }

  pub fn reset_mobile_gear_asset_data_bases(&mut self) {
    self.mobile_gear_asset_data_bases = None;
  }

  pub fn set_mobile_world_content_paths(&mut self, mobile_world_content_paths: ::std::collections::HashMap<String, String>) {
    self.mobile_world_content_paths = Some(mobile_world_content_paths);
  }

  pub fn with_mobile_world_content_paths(mut self, mobile_world_content_paths: ::std::collections::HashMap<String, String>) -> DestinyConfigDestinyManifest {
    self.mobile_world_content_paths = Some(mobile_world_content_paths);
    self
  }

  pub fn mobile_world_content_paths(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.mobile_world_content_paths.as_ref()
  }

  pub fn reset_mobile_world_content_paths(&mut self) {
    self.mobile_world_content_paths = None;
  }

  pub fn set_json_world_content_paths(&mut self, json_world_content_paths: ::std::collections::HashMap<String, String>) {
    self.json_world_content_paths = Some(json_world_content_paths);
  }

  pub fn with_json_world_content_paths(mut self, json_world_content_paths: ::std::collections::HashMap<String, String>) -> DestinyConfigDestinyManifest {
    self.json_world_content_paths = Some(json_world_content_paths);
    self
  }

  pub fn json_world_content_paths(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.json_world_content_paths.as_ref()
  }

  pub fn reset_json_world_content_paths(&mut self) {
    self.json_world_content_paths = None;
  }

  pub fn set_json_world_component_content_paths(&mut self, json_world_component_content_paths: ::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>) {
    self.json_world_component_content_paths = Some(json_world_component_content_paths);
  }

  pub fn with_json_world_component_content_paths(mut self, json_world_component_content_paths: ::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>) -> DestinyConfigDestinyManifest {
    self.json_world_component_content_paths = Some(json_world_component_content_paths);
    self
  }

  pub fn json_world_component_content_paths(&self) -> Option<&::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>> {
    self.json_world_component_content_paths.as_ref()
  }

  pub fn reset_json_world_component_content_paths(&mut self) {
    self.json_world_component_content_paths = None;
  }

  pub fn set_mobile_clan_banner_database_path(&mut self, mobile_clan_banner_database_path: String) {
    self.mobile_clan_banner_database_path = Some(mobile_clan_banner_database_path);
  }

  pub fn with_mobile_clan_banner_database_path(mut self, mobile_clan_banner_database_path: String) -> DestinyConfigDestinyManifest {
    self.mobile_clan_banner_database_path = Some(mobile_clan_banner_database_path);
    self
  }

  pub fn mobile_clan_banner_database_path(&self) -> Option<&String> {
    self.mobile_clan_banner_database_path.as_ref()
  }

  pub fn reset_mobile_clan_banner_database_path(&mut self) {
    self.mobile_clan_banner_database_path = None;
  }

  pub fn set_mobile_gear_cdn(&mut self, mobile_gear_cdn: ::std::collections::HashMap<String, String>) {
    self.mobile_gear_cdn = Some(mobile_gear_cdn);
  }

  pub fn with_mobile_gear_cdn(mut self, mobile_gear_cdn: ::std::collections::HashMap<String, String>) -> DestinyConfigDestinyManifest {
    self.mobile_gear_cdn = Some(mobile_gear_cdn);
    self
  }

  pub fn mobile_gear_cdn(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.mobile_gear_cdn.as_ref()
  }

  pub fn reset_mobile_gear_cdn(&mut self) {
    self.mobile_gear_cdn = None;
  }

  pub fn set_icon_image_pyramid_info(&mut self, icon_image_pyramid_info: Vec<::models::DestinyConfigImagePyramidEntry>) {
    self.icon_image_pyramid_info = Some(icon_image_pyramid_info);
  }

  pub fn with_icon_image_pyramid_info(mut self, icon_image_pyramid_info: Vec<::models::DestinyConfigImagePyramidEntry>) -> DestinyConfigDestinyManifest {
    self.icon_image_pyramid_info = Some(icon_image_pyramid_info);
    self
  }

  pub fn icon_image_pyramid_info(&self) -> Option<&Vec<::models::DestinyConfigImagePyramidEntry>> {
    self.icon_image_pyramid_info.as_ref()
  }

  pub fn reset_icon_image_pyramid_info(&mut self) {
    self.icon_image_pyramid_info = None;
  }

}



