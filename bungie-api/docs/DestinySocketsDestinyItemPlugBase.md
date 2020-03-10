# DestinySocketsDestinyItemPlugBase

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plug_item_hash** | **i32** | The hash identifier of the DestinyInventoryItemDefinition that represents this plug. | [optional] [default to null]
**can_insert** | **bool** | If true, this plug has met all of its insertion requirements. Big if true. | [optional] [default to null]
**enabled** | **bool** | If true, this plug will provide its benefits while inserted. | [optional] [default to null]
**insert_fail_indexes** | **Vec<i32>** | If the plug cannot be inserted for some reason, this will have the indexes into the plug item definition&#39;s plug.insertionRules property, so you can show the reasons why it can&#39;t be inserted.  This list will be empty if the plug can be inserted. | [optional] [default to null]
**enable_fail_indexes** | **Vec<i32>** | If a plug is not enabled, this will be populated with indexes into the plug item definition&#39;s plug.enabledRules property, so that you can show the reasons why it is not enabled.  This list will be empty if the plug is enabled. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


