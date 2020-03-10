# DestinyDefinitionsDestinyFactionDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md) |  | [optional] [default to null]
**progression_hash** | **i32** | The hash identifier for the DestinyProgressionDefinition that indicates the character&#39;s relationship with this faction in terms of experience and levels. | [optional] [default to null]
**token_values** | **::std::collections::HashMap<String, i32>** | The faction token item hashes, and their respective progression values. | [optional] [default to null]
**reward_item_hash** | **i32** | The faction reward item hash, usually an engram. | [optional] [default to null]
**reward_vendor_hash** | **i32** | The faction reward vendor hash, used for faction engram previews. | [optional] [default to null]
**vendors** | [**Vec<::models::DestinyDefinitionsDestinyFactionVendorDefinition>**](Destiny.Definitions.DestinyFactionVendorDefinition.md) | List of vendors that are associated with this faction. The last vendor that passes the unlock flag checks is the one that should be shown. | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


