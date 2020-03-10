# DestinyConfigDestinyManifest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **String** |  | [optional] [default to null]
**mobile_asset_content_path** | **String** |  | [optional] [default to null]
**mobile_gear_asset_data_bases** | [**Vec<::models::DestinyConfigGearAssetDataBaseDefinition>**](Destiny.Config.GearAssetDataBaseDefinition.md) |  | [optional] [default to null]
**mobile_world_content_paths** | **::std::collections::HashMap<String, String>** |  | [optional] [default to null]
**json_world_content_paths** | **::std::collections::HashMap<String, String>** | This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a path to the aggregated world definitions (warning: large file!) | [optional] [default to null]
**json_world_component_content_paths** | [**::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>**](map.md) | This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a dictionary, where the key is a definition type by name, and the value is the path to the file for that definition. WARNING: This is unsafe and subject to change - do not depend on data in these files staying around long-term. | [optional] [default to null]
**mobile_clan_banner_database_path** | **String** |  | [optional] [default to null]
**mobile_gear_cdn** | **::std::collections::HashMap<String, String>** |  | [optional] [default to null]
**icon_image_pyramid_info** | [**Vec<::models::DestinyConfigImagePyramidEntry>**](Destiny.Config.ImagePyramidEntry.md) | Information about the \&quot;Image Pyramid\&quot; for Destiny icons. Where possible, we create smaller versions of Destiny icons. These are found as subfolders under the location of the \&quot;original/full size\&quot; Destiny images, with the same file name and extension as the original image itself. (this lets us avoid sending largely redundant path info with every entity, at the expense of the smaller versions of the image being less discoverable) | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


