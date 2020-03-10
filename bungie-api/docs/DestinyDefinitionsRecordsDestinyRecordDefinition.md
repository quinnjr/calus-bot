# DestinyDefinitionsRecordsDestinyRecordDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md) |  | [optional] [default to null]
**scope** | **i32** | Indicates whether this Record&#39;s state is determined on a per-character or on an account-wide basis. | [optional] [default to null]
**presentation_info** | [***::models::DestinyDefinitionsPresentationDestinyPresentationChildBlock**](Destiny.Definitions.Presentation.DestinyPresentationChildBlock.md) |  | [optional] [default to null]
**lore_hash** | **i32** |  | [optional] [default to null]
**objective_hashes** | **Vec<i32>** |  | [optional] [default to null]
**record_value_style** | **i32** |  | [optional] [default to null]
**title_info** | [***::models::DestinyDefinitionsRecordsDestinyRecordTitleBlock**](Destiny.Definitions.Records.DestinyRecordTitleBlock.md) |  | [optional] [default to null]
**completion_info** | [***::models::DestinyDefinitionsRecordsDestinyRecordCompletionBlock**](Destiny.Definitions.Records.DestinyRecordCompletionBlock.md) |  | [optional] [default to null]
**state_info** | [***::models::DestinyDefinitionsRecordsSchemaRecordStateBlock**](Destiny.Definitions.Records.SchemaRecordStateBlock.md) |  | [optional] [default to null]
**requirements** | [***::models::DestinyDefinitionsPresentationDestinyPresentationNodeRequirementsBlock**](Destiny.Definitions.Presentation.DestinyPresentationNodeRequirementsBlock.md) |  | [optional] [default to null]
**expiration_info** | [***::models::DestinyDefinitionsRecordsDestinyRecordExpirationBlock**](Destiny.Definitions.Records.DestinyRecordExpirationBlock.md) |  | [optional] [default to null]
**interval_info** | [***Value**](Value.md) | Some records have multiple &#39;interval&#39; objectives, and the record may be claimed at each completed interval | [optional] [default to null]
**reward_items** | [**Vec<::models::DestinyDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md) | If there is any publicly available information about rewards earned for achieving this record, this is the list of those items.   However, note that some records intentionally have \&quot;hidden\&quot; rewards. These will not be returned in this list. | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


