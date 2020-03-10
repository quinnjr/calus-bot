# DestinyDefinitionsDestinyItemVendorSourceReference

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_hash** | **i32** | The identifier for the vendor that may sell this item. | [optional] [default to null]
**vendor_item_indexes** | **Vec<i32>** | The Vendor sale item indexes that represent the sale information for this item. The same vendor may sell an item in multiple \&quot;ways\&quot;, hence why this is a list. (for instance, a weapon may be \&quot;sold\&quot; as a reward in a quest, for Glimmer, and for Masterwork Cores: each of those ways would be represented by a different vendor sale item with a different index) | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


