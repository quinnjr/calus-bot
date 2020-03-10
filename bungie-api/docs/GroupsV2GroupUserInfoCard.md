# GroupsV2GroupUserInfoCard

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_seen_display_name** | **String** | This will be the display name the clan server last saw the user as. If the account is an active cross save override, this will be the display name to use. Otherwise, this will match the displayName property. | [optional] [default to null]
**last_seen_display_name_type** | **i32** | The platform of the LastSeenDisplayName | [optional] [default to null]
**supplemental_display_name** | **String** | A platform specific additional display name - ex: psn Real Name, bnet Unique Name, etc. | [optional] [default to null]
**icon_path** | **String** | URL the Icon if available. | [optional] [default to null]
**cross_save_override** | **i32** | If there is a cross save override in effect, this value will tell you the type that is overridding this one. | [optional] [default to null]
**applicable_membership_types** | **Vec<i32>** | The list of Membership Types indicating the platforms on which this Membership can be used.   Not in Cross Save &#x3D; its original membership type. Cross Save Primary &#x3D; Any membership types it is overridding, and its original membership type Cross Save Overridden &#x3D; Empty list | [optional] [default to null]
**is_public** | **bool** | If True, this is a public user membership. | [optional] [default to null]
**membership_type** | **i32** | Type of the membership. Not necessarily the native type. | [optional] [default to null]
**membership_id** | **i64** | Membership ID as they user is known in the Accounts service | [optional] [default to null]
**display_name** | **String** | Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


