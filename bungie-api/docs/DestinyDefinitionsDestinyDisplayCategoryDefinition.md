# DestinyDefinitionsDestinyDisplayCategoryDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | **i32** |  | [optional] [default to null]
**identifier** | **String** | A string identifier for the display category. | [optional] [default to null]
**display_category_hash** | **i32** |  | [optional] [default to null]
**display_properties** | [***::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md) |  | [optional] [default to null]
**display_in_banner** | **bool** | If true, this category should be displayed in the \&quot;Banner\&quot; section of the vendor&#39;s UI. | [optional] [default to null]
**progression_hash** | **i32** | If it exists, this is the hash identifier of a DestinyProgressionDefinition that represents the progression to show on this display category.  Specific categories can now have thier own distinct progression, apparently. So that&#39;s cool. | [optional] [default to null]
**sort_order** | **i32** | If this category sorts items in a nonstandard way, this will be the way we sort. | [optional] [default to null]
**display_style_hash** | **i32** | An indicator of how the category will be displayed in the UI. It&#39;s up to you to do something cool or interesting in response to this, or just to treat it as a normal category. | [optional] [default to null]
**display_style_identifier** | **String** | An indicator of how the category will be displayed in the UI. It&#39;s up to you to do something cool or interesting in response to this, or just to treat it as a normal category. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


