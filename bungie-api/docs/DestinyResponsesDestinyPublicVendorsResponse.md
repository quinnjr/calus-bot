# DestinyResponsesDestinyPublicVendorsResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_groups** | [***Value**](Value.md) | For Vendors being returned, this will give you the information you need to group them and order them in the same way that the Bungie Companion app performs grouping. It will automatically be returned if you request the Vendors component.  COMPONENT TYPE: Vendors | [optional] [default to null]
**vendors** | [***Value**](Value.md) | The base properties of the vendor. These are keyed by the Vendor Hash, so you will get one Vendor Component per vendor returned.  COMPONENT TYPE: Vendors | [optional] [default to null]
**categories** | [***Value**](Value.md) | Categories that the vendor has available, and references to the sales therein. These are keyed by the Vendor Hash, so you will get one Categories Component per vendor returned.  COMPONENT TYPE: VendorCategories | [optional] [default to null]
**sales** | [***Value**](Value.md) | Sales, keyed by the vendorItemIndex of the item being sold. These are keyed by the Vendor Hash, so you will get one Sale Item Set Component per vendor returned.  Note that within the Sale Item Set component, the sales are themselves keyed by the vendorSaleIndex, so you can relate it to the corrent sale item definition within the Vendor&#39;s definition.  COMPONENT TYPE: VendorSales | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


