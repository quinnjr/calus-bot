# UserEmailViewDefinitionSetting

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The identifier for this UI Setting, which can be used to relate it to custom strings or other data as desired. | [optional] [default to null]
**localization** | [**::std::collections::HashMap<String, ::models::UserEMailSettingLocalization>**](User.EMailSettingLocalization.md) | A dictionary of localized text for the EMail setting, keyed by the locale. | [optional] [default to null]
**set_by_default** | **bool** | If true, this setting should be set by default if the user hasn&#39;t chosen whether it&#39;s set or cleared yet. | [optional] [default to null]
**opt_in_aggregate_value** | **i64** | The OptInFlags value to set or clear if this setting is set or cleared in the UI. It is the aggregate of all underlying opt-in flags related to this setting. | [optional] [default to null]
**subscriptions** | [**Vec<::models::UserEmailSubscriptionDefinition>**](User.EmailSubscriptionDefinition.md) | The subscriptions to show as children of this setting, if any. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


