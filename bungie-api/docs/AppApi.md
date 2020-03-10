# \AppApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_get_application_api_usage**](AppApi.md#app_get_application_api_usage) | **Get** /App/ApiUsage/{applicationId}/ | 
[**app_get_bungie_applications**](AppApi.md#app_get_bungie_applications) | **Get** /App/FirstParty/ | 


# **app_get_application_api_usage**
> ::models::InlineResponse200 app_get_application_api_usage(ctx, application_id, optional)


Get API usage by application for time frame specified. You can go as far back as 30 days ago, and can ask for up to a 48 hour window of time in a single request. You must be authenticated with at least the ReadUserData permission to access this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **application_id** | **i32**| ID of the application to get usage statistics. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **application_id** | **i32**| ID of the application to get usage statistics. | 
 **end** | **String**| End time for query. Goes to now if not specified. | 
 **start** | **String**| Start time for query. Goes to 24 hours ago if not specified. | 

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **app_get_bungie_applications**
> ::models::InlineResponse2001 app_get_bungie_applications()


Get list of applications created by Bungie.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

