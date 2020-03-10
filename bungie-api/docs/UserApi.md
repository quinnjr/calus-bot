# \UserApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_get_available_themes**](UserApi.md#user_get_available_themes) | **Get** /User/GetAvailableThemes/ | 
[**user_get_bungie_net_user_by_id**](UserApi.md#user_get_bungie_net_user_by_id) | **Get** /User/GetBungieNetUserById/{id}/ | 
[**user_get_membership_data_by_id**](UserApi.md#user_get_membership_data_by_id) | **Get** /User/GetMembershipsById/{membershipId}/{membershipType}/ | 
[**user_get_membership_data_for_current_user**](UserApi.md#user_get_membership_data_for_current_user) | **Get** /User/GetMembershipsForCurrentUser/ | 
[**user_get_membership_from_hard_linked_credential**](UserApi.md#user_get_membership_from_hard_linked_credential) | **Get** /User/GetMembershipFromHardLinkedCredential/{crType}/{credential}/ | 
[**user_get_partnerships**](UserApi.md#user_get_partnerships) | **Get** /User/{membershipId}/Partnerships/ | 
[**user_search_users**](UserApi.md#user_search_users) | **Get** /User/SearchUsers/ | 


# **user_get_available_themes**
> ::models::InlineResponse2004 user_get_available_themes()


Returns a list of all available user themes.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_get_bungie_net_user_by_id**
> ::models::InlineResponse2002 user_get_bungie_net_user_by_id(id)


Loads a bungienet user by membership id.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **i64**| The requested Bungie.net membership id. | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_get_membership_data_by_id**
> ::models::InlineResponse2005 user_get_membership_data_by_id(membership_id, membership_type)


Returns a list of accounts associated with the supplied membership ID and membership type. This will include all linked accounts (even when hidden) if supplied credentials permit it.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **membership_id** | **i64**| The membership ID of the target user. | 
  **membership_type** | **i32**| Type of the supplied membership ID. | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_get_membership_data_for_current_user**
> ::models::InlineResponse2005 user_get_membership_data_for_current_user(ctx, )


Returns a list of accounts associated with signed in user. This is useful for OAuth implementations that do not give you access to the token response.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_get_membership_from_hard_linked_credential**
> ::models::InlineResponse2007 user_get_membership_from_hard_linked_credential(credential, cr_type)


Gets any hard linked membership given a credential. Only works for credentials that are public (just SteamID64 right now). Cross Save aware.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **credential** | **String**| The credential to look up. Must be a valid SteamID64. | 
  **cr_type** | **i32**| The credential type. &#39;SteamId&#39; is the only valid value at present. | 

### Return type

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_get_partnerships**
> ::models::InlineResponse2006 user_get_partnerships(membership_id)


Returns a user's linked Partnerships.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **membership_id** | **i64**| The ID of the member for whom partnerships should be returned. | 

### Return type

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_search_users**
> ::models::InlineResponse2003 user_search_users(optional)


Returns a list of possible users based on the search string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **q** | **String**| The search string. | 

### Return type

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

