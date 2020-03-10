# UserCrossSaveUserMembership

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cross_save_override** | **i32** | If there is a cross save override in effect, this value will tell you the type that is overridding this one. | [optional] [default to null]
**applicable_membership_types** | **Vec<i32>** | The list of Membership Types indicating the platforms on which this Membership can be used.   Not in Cross Save &#x3D; its original membership type. Cross Save Primary &#x3D; Any membership types it is overridding, and its original membership type Cross Save Overridden &#x3D; Empty list | [optional] [default to null]
**is_public** | **bool** | If True, this is a public user membership. | [optional] [default to null]
**membership_type** | **i32** | Type of the membership. Not necessarily the native type. | [optional] [default to null]
**membership_id** | **i64** | Membership ID as they user is known in the Accounts service | [optional] [default to null]
**display_name** | **String** | Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


