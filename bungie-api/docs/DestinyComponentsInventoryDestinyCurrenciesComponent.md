# DestinyComponentsInventoryDestinyCurrenciesComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_quantities** | **::std::collections::HashMap<String, i32>** | A dictionary - keyed by the item&#39;s hash identifier (DestinyInventoryItemDefinition), and whose value is the amount of that item you have across all available inventory buckets for purchasing.  This allows you to see whether the requesting character can afford any given purchase/action without having to re-create this list itself. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


