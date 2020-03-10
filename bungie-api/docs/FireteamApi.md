# \FireteamApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fireteam_get_active_private_clan_fireteam_count**](FireteamApi.md#fireteam_get_active_private_clan_fireteam_count) | **Get** /Fireteam/Clan/{groupId}/ActiveCount/ | 
[**fireteam_get_available_clan_fireteams**](FireteamApi.md#fireteam_get_available_clan_fireteams) | **Get** /Fireteam/Clan/{groupId}/Available/{platform}/{activityType}/{dateRange}/{slotFilter}/{publicOnly}/{page}/ | 
[**fireteam_get_clan_fireteam**](FireteamApi.md#fireteam_get_clan_fireteam) | **Get** /Fireteam/Clan/{groupId}/Summary/{fireteamId}/ | 
[**fireteam_get_my_clan_fireteams**](FireteamApi.md#fireteam_get_my_clan_fireteams) | **Get** /Fireteam/Clan/{groupId}/My/{platform}/{includeClosed}/{page}/ | 
[**fireteam_search_public_available_clan_fireteams**](FireteamApi.md#fireteam_search_public_available_clan_fireteams) | **Get** /Fireteam/Search/Available/{platform}/{activityType}/{dateRange}/{slotFilter}/{page}/ | 


# **fireteam_get_active_private_clan_fireteam_count**
> ::models::InlineResponse20023 fireteam_get_active_private_clan_fireteam_count(ctx, group_id)


Gets a count of all active non-public fireteams for the specified clan. Maximum value returned is 25.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| The group id of the clan. | 

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fireteam_get_available_clan_fireteams**
> ::models::InlineResponse20067 fireteam_get_available_clan_fireteams(ctx, activity_type, date_range, group_id, page, platform, public_only, slot_filter, optional)


Gets a listing of all of this clan's fireteams that are have available slots. Caller is not checked for join criteria so caching is maximized.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **activity_type** | **i32**| The activity type to filter by. | 
  **date_range** | **i32**| The date range to grab available fireteams. | 
  **group_id** | **i64**| The group id of the clan. | 
  **page** | **i32**| Zero based page | 
  **platform** | **i32**| The platform filter. | 
  **public_only** | **i32**| Determines public/private filtering. | 
  **slot_filter** | **i32**| Filters based on available slots | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **activity_type** | **i32**| The activity type to filter by. | 
 **date_range** | **i32**| The date range to grab available fireteams. | 
 **group_id** | **i64**| The group id of the clan. | 
 **page** | **i32**| Zero based page | 
 **platform** | **i32**| The platform filter. | 
 **public_only** | **i32**| Determines public/private filtering. | 
 **slot_filter** | **i32**| Filters based on available slots | 
 **lang_filter** | **String**| An optional language filter. | 

### Return type

[**::models::InlineResponse20067**](inline_response_200_67.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fireteam_get_clan_fireteam**
> ::models::InlineResponse20069 fireteam_get_clan_fireteam(ctx, fireteam_id, group_id)


Gets a specific clan fireteam.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **fireteam_id** | **i64**| The unique id of the fireteam. | 
  **group_id** | **i64**| The group id of the clan. | 

### Return type

[**::models::InlineResponse20069**](inline_response_200_69.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fireteam_get_my_clan_fireteams**
> ::models::InlineResponse20068 fireteam_get_my_clan_fireteams(ctx, group_id, include_closed, page, platform, optional)


Gets a listing of all clan fireteams that caller is an applicant, a member, or an alternate of.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| The group id of the clan. (This parameter is ignored unless the optional query parameter groupFilter is true). | 
  **include_closed** | **bool**| If true, return fireteams that have been closed. | 
  **page** | **i32**| Deprecated parameter, ignored. | 
  **platform** | **i32**| The platform filter. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_id** | **i64**| The group id of the clan. (This parameter is ignored unless the optional query parameter groupFilter is true). | 
 **include_closed** | **bool**| If true, return fireteams that have been closed. | 
 **page** | **i32**| Deprecated parameter, ignored. | 
 **platform** | **i32**| The platform filter. | 
 **group_filter** | **bool**| If true, filter by clan. Otherwise, ignore the clan and show all of the user&#39;s fireteams. | 
 **lang_filter** | **String**| An optional language filter. | 

### Return type

[**::models::InlineResponse20068**](inline_response_200_68.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fireteam_search_public_available_clan_fireteams**
> ::models::InlineResponse20067 fireteam_search_public_available_clan_fireteams(ctx, activity_type, date_range, page, platform, slot_filter, optional)


Gets a listing of all public fireteams starting now with open slots. Caller is not checked for join criteria so caching is maximized.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **activity_type** | **i32**| The activity type to filter by. | 
  **date_range** | **i32**| The date range to grab available fireteams. | 
  **page** | **i32**| Zero based page | 
  **platform** | **i32**| The platform filter. | 
  **slot_filter** | **i32**| Filters based on available slots | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **activity_type** | **i32**| The activity type to filter by. | 
 **date_range** | **i32**| The date range to grab available fireteams. | 
 **page** | **i32**| Zero based page | 
 **platform** | **i32**| The platform filter. | 
 **slot_filter** | **i32**| Filters based on available slots | 
 **lang_filter** | **String**| An optional language filter. | 

### Return type

[**::models::InlineResponse20067**](inline_response_200_67.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

