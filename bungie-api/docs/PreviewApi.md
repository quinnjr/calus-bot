# \PreviewApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**destiny2_get_clan_aggregate_stats**](PreviewApi.md#destiny2_get_clan_aggregate_stats) | **Get** /Destiny2/Stats/AggregateClanStats/{groupId}/ | 
[**destiny2_get_clan_leaderboards**](PreviewApi.md#destiny2_get_clan_leaderboards) | **Get** /Destiny2/Stats/Leaderboards/Clans/{groupId}/ | 
[**destiny2_get_leaderboards**](PreviewApi.md#destiny2_get_leaderboards) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/Leaderboards/ | 
[**destiny2_get_leaderboards_for_character**](PreviewApi.md#destiny2_get_leaderboards_for_character) | **Get** /Destiny2/Stats/Leaderboards/{membershipType}/{destinyMembershipId}/{characterId}/ | 
[**destiny2_get_public_vendors**](PreviewApi.md#destiny2_get_public_vendors) | **Get** /Destiny2//Vendors/ | 
[**destiny2_insert_socket_plug**](PreviewApi.md#destiny2_insert_socket_plug) | **Post** /Destiny2/Actions/Items/InsertSocketPlug/ | 


# **destiny2_get_clan_aggregate_stats**
> ::models::InlineResponse20051 destiny2_get_clan_aggregate_stats(group_id, optional)


Gets aggregated stats for a clan using the same categories as the clan leaderboards. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 

### Return type

[**::models::InlineResponse20051**](inline_response_200_51.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_clan_leaderboards**
> ::models::InlineResponse20050 destiny2_get_clan_leaderboards(group_id, optional)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **maxtop** | **i32**| Maximum number of top players to return. Use a large number to get entire leaderboard. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **statid** | **String**| ID of stat to return rather than returning all Leaderboard stats. | 

### Return type

[**::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_leaderboards**
> ::models::InlineResponse20050 destiny2_get_leaderboards(destiny_membership_id, membership_type, optional)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint has not yet been implemented. It is being returned for a preview of future functionality, and for public comment/suggestion/preparation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **maxtop** | **i32**| Maximum number of top players to return. Use a large number to get entire leaderboard. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **statid** | **String**| ID of stat to return rather than returning all Leaderboard stats. | 

### Return type

[**::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_leaderboards_for_character**
> ::models::InlineResponse20050 destiny2_get_leaderboards_for_character(character_id, destiny_membership_id, membership_type, optional)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The specific character to build the leaderboard around for the provided Destiny Membership. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The specific character to build the leaderboard around for the provided Destiny Membership. | 
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **maxtop** | **i32**| Maximum number of top players to return. Use a large number to get entire leaderboard. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **statid** | **String**| ID of stat to return rather than returning all Leaderboard stats. | 

### Return type

[**::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_public_vendors**
> ::models::InlineResponse20044 destiny2_get_public_vendors(optional)


Get items available from vendors where the vendors have items for sale that are common for everyone. If any portion of the Vendor's available inventory is character or account specific, we will be unable to return their data from this endpoint due to the way that available inventory is computed. As I am often guilty of saying: 'It's a long story...'

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **components** | [**Vec&lt;i32&gt;**](i32.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20044**](inline_response_200_44.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_insert_socket_plug**
> ::models::InlineResponse20047 destiny2_insert_socket_plug(ctx, )


Insert a plug into a socketed item. I know how it sounds, but I assure you it's much more G-rated than you might be guessing. We haven't decided yet whether this will be able to insert plugs that have side effects, but if we do it will require special scope permission for an application attempting to do so. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline. Request must include proof of permission for 'InsertPlugs' from the account owner.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20047**](inline_response_200_47.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

