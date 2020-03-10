# \ContentApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_get_content_by_id**](ContentApi.md#content_get_content_by_id) | **Get** /Content/GetContentById/{id}/{locale}/ | 
[**content_get_content_by_tag_and_type**](ContentApi.md#content_get_content_by_tag_and_type) | **Get** /Content/GetContentByTagAndType/{tag}/{type}/{locale}/ | 
[**content_get_content_type**](ContentApi.md#content_get_content_type) | **Get** /Content/GetContentType/{type}/ | 
[**content_search_content_by_tag_and_type**](ContentApi.md#content_search_content_by_tag_and_type) | **Get** /Content/SearchContentByTagAndType/{tag}/{type}/{locale}/ | 
[**content_search_content_with_text**](ContentApi.md#content_search_content_with_text) | **Get** /Content/Search/{locale}/ | 
[**content_search_help_articles**](ContentApi.md#content_search_help_articles) | **Get** /Content/SearchHelpArticles/{searchtext}/{size}/ | 


# **content_get_content_by_id**
> ::models::InlineResponse2009 content_get_content_by_id(id, locale, optional)


Returns a content item referenced by id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **i64**|  | 
  **locale** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**|  | 
 **locale** | **String**|  | 
 **head** | **bool**| false | 

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **content_get_content_by_tag_and_type**
> ::models::InlineResponse2009 content_get_content_by_tag_and_type(locale, tag, _type, optional)


Returns the newest item that matches a given tag and Content Type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **locale** | **String**|  | 
  **tag** | **String**|  | 
  **_type** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **locale** | **String**|  | 
 **tag** | **String**|  | 
 **_type** | **String**|  | 
 **head** | **bool**| Not used. | 

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **content_get_content_type**
> ::models::InlineResponse2008 content_get_content_type(_type)


Gets an object describing a particular variant of content.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **_type** | **String**|  | 

### Return type

[**::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **content_search_content_by_tag_and_type**
> ::models::InlineResponse20010 content_search_content_by_tag_and_type(locale, tag, _type, optional)


Searches for Content Items that match the given Tag and Content Type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **locale** | **String**|  | 
  **tag** | **String**|  | 
  **_type** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **locale** | **String**|  | 
 **tag** | **String**|  | 
 **_type** | **String**|  | 
 **currentpage** | **i32**| Page number for the search results starting with page 1. | 
 **head** | **bool**| Not used. | 
 **itemsperpage** | **i32**| Not used. | 

### Return type

[**::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **content_search_content_with_text**
> ::models::InlineResponse20010 content_search_content_with_text(locale, optional)


Gets content based on querystring information passed in. Provides basic search and text search capabilities.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **locale** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **locale** | **String**|  | 
 **ctype** | **String**| Content type tag: Help, News, etc. Supply multiple ctypes separated by space. | 
 **currentpage** | **i32**| Page number for the search results, starting with page 1. | 
 **head** | **bool**| Not used. | 
 **searchtext** | **String**| Word or phrase for the search. | 
 **source** | **String**| For analytics, hint at the part of the app that triggered the search. Optional. | 
 **tag** | **String**| Tag used on the content to be searched. | 

### Return type

[**::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **content_search_help_articles**
> ::models::InlineResponse20011 content_search_help_articles(searchtext, size)


Search for Help Articles.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **searchtext** | **String**|  | 
  **size** | **String**|  | 

### Return type

[**::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

