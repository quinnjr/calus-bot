# ApplicationsApiUsage

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**range** | [***Value**](Value.md) | The date range for the data being reported. | [optional] [default to null]
**api_calls** | [**Vec<::models::ApplicationsSeries>**](Applications.Series.md) | Counts for on API calls made for the time range. | [optional] [default to null]
**throttled_requests** | [**Vec<::models::ApplicationsSeries>**](Applications.Series.md) | Instances of blocked requests or requests that crossed the warn threshold during the time range. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


