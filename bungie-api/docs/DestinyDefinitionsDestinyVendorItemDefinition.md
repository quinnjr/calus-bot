# DestinyDefinitionsDestinyVendorItemDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_item_index** | **i32** | The index into the DestinyVendorDefinition.saleList. This is what we use to refer to items being sold throughout live and definition data. | [optional] [default to null]
**item_hash** | **i32** | The hash identifier of the item being sold (DestinyInventoryItemDefinition).  Note that a vendor can sell the same item in multiple ways, so don&#39;t assume that itemHash is a unique identifier for this entity. | [optional] [default to null]
**quantity** | **i32** | The amount you will recieve of the item described in itemHash if you make the purchase. | [optional] [default to null]
**failure_indexes** | **Vec<i32>** | An list of indexes into the DestinyVendorDefinition.failureStrings array, indicating the possible failure strings that can be relevant for this item. | [optional] [default to null]
**currencies** | [**Vec<::models::DestinyDefinitionsDestinyVendorItemQuantity>**](Destiny.Definitions.DestinyVendorItemQuantity.md) | This is a pre-compiled aggregation of item value and priceOverrideList, so that we have one place to check for what the purchaser must pay for the item. Use this instead of trying to piece together the price separately.  The somewhat crappy part about this is that, now that item quantity overrides have dynamic modifiers, this will not necessarily be statically true. If you were using this instead of live data, switch to using live data. | [optional] [default to null]
**refund_policy** | **i32** | If this item can be refunded, this is the policy for what will be refundd, how, and in what time period. | [optional] [default to null]
**refund_time_limit** | **i32** | The amount of time before refundability of the newly purchased item will expire. | [optional] [default to null]
**creation_levels** | [**Vec<::models::DestinyDefinitionsDestinyItemCreationEntryLevelDefinition>**](Destiny.Definitions.DestinyItemCreationEntryLevelDefinition.md) | The Default level at which the item will spawn. Almost always driven by an adjusto these days. Ideally should be singular. It&#39;s a long story how this ended up as a list, but there is always either going to be 0:1 of these entities. | [optional] [default to null]
**display_category_index** | **i32** | This is an index specifically into the display category, as opposed to the server-side Categories (which do not need to match or pair with each other in any way: server side categories are really just structures for common validation. Display Category will let us more easily categorize items visually) | [optional] [default to null]
**category_index** | **i32** | The index into the DestinyVendorDefinition.categories array, so you can find the category associated with this item. | [optional] [default to null]
**original_category_index** | **i32** | Same as above, but for the original category indexes. | [optional] [default to null]
**minimum_level** | **i32** | The minimum character level at which this item is available for sale. | [optional] [default to null]
**maximum_level** | **i32** | The maximum character level at which this item is available for sale. | [optional] [default to null]
**action** | [***Value**](Value.md) | The action to be performed when purchasing the item, if it&#39;s not just \&quot;buy\&quot;. | [optional] [default to null]
**display_category** | **String** | The string identifier for the category selling this item. | [optional] [default to null]
**inventory_bucket_hash** | **i32** | The inventory bucket into which this item will be placed upon purchase. | [optional] [default to null]
**visibility_scope** | **i32** | The most restrictive scope that determines whether the item is available in the Vendor&#39;s inventory. See DestinyGatingScope&#39;s documentation for more information.  This can be determined by Unlock gating, or by whether or not the item has purchase level requirements (minimumLevel and maximumLevel properties). | [optional] [default to null]
**purchasable_scope** | **i32** | Similar to visibilityScope, it represents the most restrictive scope that determines whether the item can be purchased. It will at least be as restrictive as visibilityScope, but could be more restrictive if the item has additional purchase requirements beyond whether it is merely visible or not.  See DestinyGatingScope&#39;s documentation for more information. | [optional] [default to null]
**exclusivity** | **i32** | If this item can only be purchased by a given platform, this indicates the platform to which it is restricted. | [optional] [default to null]
**is_offer** | **bool** | If this sale can only be performed as the result of an offer check, this is true. | [optional] [default to null]
**is_crm** | **bool** | If this sale can only be performed as the result of receiving a CRM offer, this is true. | [optional] [default to null]
**sort_value** | **i32** | *if* the category this item is in supports non-default sorting, this value should represent the sorting value to use, pre-processed and ready to go. | [optional] [default to null]
**expiration_tooltip** | **String** | If this item can expire, this is the tooltip message to show with its expiration info. | [optional] [default to null]
**redirect_to_sale_indexes** | **Vec<i32>** | If this is populated, the purchase of this item should redirect to purchasing these other items instead. | [optional] [default to null]
**socket_overrides** | [**Vec<::models::DestinyDefinitionsDestinyVendorItemSocketOverride>**](Destiny.Definitions.DestinyVendorItemSocketOverride.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


