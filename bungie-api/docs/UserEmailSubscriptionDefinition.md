# UserEmailSubscriptionDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The unique identifier for this subscription. | [optional] [default to null]
**localization** | [**::std::collections::HashMap<String, ::models::UserEMailSettingSubscriptionLocalization>**](User.EMailSettingSubscriptionLocalization.md) | A dictionary of localized text for the EMail Opt-in setting, keyed by the locale. | [optional] [default to null]
**value** | **i64** | The bitflag value for this subscription. Should be a unique power of two value. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


