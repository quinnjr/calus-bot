# \DefaultApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**_get_available_locales**](DefaultApi.md#_get_available_locales) | **Get** /GetAvailableLocales/ | 
[**_get_common_settings**](DefaultApi.md#_get_common_settings) | **Get** /Settings/ | 
[**_get_global_alerts**](DefaultApi.md#_get_global_alerts) | **Get** /GlobalAlerts/ | 


# **_get_available_locales**
> ::models::InlineResponse20016 _get_available_locales()


List of available localization cultures

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **_get_common_settings**
> ::models::InlineResponse20070 _get_common_settings()


Get the common settings used by the Bungie.Net environment.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20070**](inline_response_200_70.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **_get_global_alerts**
> ::models::InlineResponse20071 _get_global_alerts(optional)


Gets any active global alert for display in the forum banners, help pages, etc. Usually used for DOC alerts.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **includestreaming** | **bool**| Determines whether Streaming Alerts are included in results | 

### Return type

[**::models::InlineResponse20071**](inline_response_200_71.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

