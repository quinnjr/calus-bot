# DestinyDefinitionsDestinyProgressionRewardItemQuantity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rewarded_at_progression_level** | **i32** |  | [optional] [default to null]
**acquisition_behavior** | **i32** |  | [optional] [default to null]
**ui_display_style** | **String** |  | [optional] [default to null]
**claim_unlock_display_strings** | **Vec<String>** |  | [optional] [default to null]
**item_hash** | **i32** | The hash identifier for the item in question. Use it to look up the item&#39;s DestinyInventoryItemDefinition. | [optional] [default to null]
**item_instance_id** | **i64** | If this quantity is referring to a specific instance of an item, this will have the item&#39;s instance ID. Normally, this will be null. | [optional] [default to null]
**quantity** | **i32** | The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


