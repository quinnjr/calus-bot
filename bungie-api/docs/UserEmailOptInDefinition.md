# UserEmailOptInDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The unique identifier for this opt-in category. | [optional] [default to null]
**value** | **i64** | The flag value for this opt-in category. For historical reasons, this is defined as a flags enum. | [optional] [default to null]
**set_by_default** | **bool** | If true, this opt-in setting should be set by default in situations where accounts are created without explicit choices about what they&#39;re opting into. | [optional] [default to null]
**dependent_subscriptions** | [**Vec<::models::UserEmailSubscriptionDefinition>**](User.EmailSubscriptionDefinition.md) | Information about the dependent subscriptions for this opt-in. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


