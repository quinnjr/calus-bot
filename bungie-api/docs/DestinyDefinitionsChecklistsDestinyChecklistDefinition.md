# DestinyDefinitionsChecklistsDestinyChecklistDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md) |  | [optional] [default to null]
**view_action_string** | **String** | A localized string prompting you to view the checklist. | [optional] [default to null]
**scope** | **i32** | Indicates whether you will find this checklist on the Profile or Character components. | [optional] [default to null]
**entries** | [**Vec<::models::DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition>**](Destiny.Definitions.Checklists.DestinyChecklistEntryDefinition.md) | The individual checklist items. Gotta catch &#39;em all. | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


