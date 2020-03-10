# \Destiny2Api

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**destiny2_awa_get_action_token**](Destiny2Api.md#destiny2_awa_get_action_token) | **Get** /Destiny2/Awa/GetActionToken/{correlationId}/ | 
[**destiny2_awa_initialize_request**](Destiny2Api.md#destiny2_awa_initialize_request) | **Post** /Destiny2/Awa/Initialize/ | 
[**destiny2_awa_provide_authorization_result**](Destiny2Api.md#destiny2_awa_provide_authorization_result) | **Post** /Destiny2/Awa/AwaProvideAuthorizationResult/ | 
[**destiny2_equip_item**](Destiny2Api.md#destiny2_equip_item) | **Post** /Destiny2/Actions/Items/EquipItem/ | 
[**destiny2_equip_items**](Destiny2Api.md#destiny2_equip_items) | **Post** /Destiny2/Actions/Items/EquipItems/ | 
[**destiny2_get_activity_history**](Destiny2Api.md#destiny2_get_activity_history) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/Activities/ | 
[**destiny2_get_character**](Destiny2Api.md#destiny2_get_character) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/ | 
[**destiny2_get_clan_aggregate_stats**](Destiny2Api.md#destiny2_get_clan_aggregate_stats) | **Get** /Destiny2/Stats/AggregateClanStats/{groupId}/ | 
[**destiny2_get_clan_leaderboards**](Destiny2Api.md#destiny2_get_clan_leaderboards) | **Get** /Destiny2/Stats/Leaderboards/Clans/{groupId}/ | 
[**destiny2_get_clan_weekly_reward_state**](Destiny2Api.md#destiny2_get_clan_weekly_reward_state) | **Get** /Destiny2/Clan/{groupId}/WeeklyRewardState/ | 
[**destiny2_get_collectible_node_details**](Destiny2Api.md#destiny2_get_collectible_node_details) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Collectibles/{collectiblePresentationNodeHash}/ | 
[**destiny2_get_destiny_aggregate_activity_stats**](Destiny2Api.md#destiny2_get_destiny_aggregate_activity_stats) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/AggregateActivityStats/ | 
[**destiny2_get_destiny_entity_definition**](Destiny2Api.md#destiny2_get_destiny_entity_definition) | **Get** /Destiny2/Manifest/{entityType}/{hashIdentifier}/ | 
[**destiny2_get_destiny_manifest**](Destiny2Api.md#destiny2_get_destiny_manifest) | **Get** /Destiny2/Manifest/ | 
[**destiny2_get_historical_stats**](Destiny2Api.md#destiny2_get_historical_stats) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/ | 
[**destiny2_get_historical_stats_definition**](Destiny2Api.md#destiny2_get_historical_stats_definition) | **Get** /Destiny2/Stats/Definition/ | 
[**destiny2_get_historical_stats_for_account**](Destiny2Api.md#destiny2_get_historical_stats_for_account) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/ | 
[**destiny2_get_item**](Destiny2Api.md#destiny2_get_item) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Item/{itemInstanceId}/ | 
[**destiny2_get_leaderboards**](Destiny2Api.md#destiny2_get_leaderboards) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/Leaderboards/ | 
[**destiny2_get_leaderboards_for_character**](Destiny2Api.md#destiny2_get_leaderboards_for_character) | **Get** /Destiny2/Stats/Leaderboards/{membershipType}/{destinyMembershipId}/{characterId}/ | 
[**destiny2_get_linked_profiles**](Destiny2Api.md#destiny2_get_linked_profiles) | **Get** /Destiny2/{membershipType}/Profile/{membershipId}/LinkedProfiles/ | 
[**destiny2_get_post_game_carnage_report**](Destiny2Api.md#destiny2_get_post_game_carnage_report) | **Get** /Destiny2/Stats/PostGameCarnageReport/{activityId}/ | 
[**destiny2_get_profile**](Destiny2Api.md#destiny2_get_profile) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/ | 
[**destiny2_get_public_milestone_content**](Destiny2Api.md#destiny2_get_public_milestone_content) | **Get** /Destiny2/Milestones/{milestoneHash}/Content/ | 
[**destiny2_get_public_milestones**](Destiny2Api.md#destiny2_get_public_milestones) | **Get** /Destiny2/Milestones/ | 
[**destiny2_get_public_vendors**](Destiny2Api.md#destiny2_get_public_vendors) | **Get** /Destiny2//Vendors/ | 
[**destiny2_get_unique_weapon_history**](Destiny2Api.md#destiny2_get_unique_weapon_history) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/UniqueWeapons/ | 
[**destiny2_get_vendor**](Destiny2Api.md#destiny2_get_vendor) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/{vendorHash}/ | 
[**destiny2_get_vendors**](Destiny2Api.md#destiny2_get_vendors) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/ | 
[**destiny2_insert_socket_plug**](Destiny2Api.md#destiny2_insert_socket_plug) | **Post** /Destiny2/Actions/Items/InsertSocketPlug/ | 
[**destiny2_pull_from_postmaster**](Destiny2Api.md#destiny2_pull_from_postmaster) | **Post** /Destiny2/Actions/Items/PullFromPostmaster/ | 
[**destiny2_report_offensive_post_game_carnage_report_player**](Destiny2Api.md#destiny2_report_offensive_post_game_carnage_report_player) | **Post** /Destiny2/Stats/PostGameCarnageReport/{activityId}/Report/ | 
[**destiny2_search_destiny_entities**](Destiny2Api.md#destiny2_search_destiny_entities) | **Get** /Destiny2/Armory/Search/{type}/{searchTerm}/ | 
[**destiny2_search_destiny_player**](Destiny2Api.md#destiny2_search_destiny_player) | **Get** /Destiny2/SearchDestinyPlayer/{membershipType}/{displayName}/ | 
[**destiny2_set_item_lock_state**](Destiny2Api.md#destiny2_set_item_lock_state) | **Post** /Destiny2/Actions/Items/SetLockState/ | 
[**destiny2_transfer_item**](Destiny2Api.md#destiny2_transfer_item) | **Post** /Destiny2/Actions/Items/TransferItem/ | 


# **destiny2_awa_get_action_token**
> ::models::InlineResponse20061 destiny2_awa_get_action_token(ctx, correlation_id)


Returns the action token if user approves the request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **correlation_id** | **String**| The identifier for the advanced write action request. | 

### Return type

[**::models::InlineResponse20061**](inline_response_200_61.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_awa_initialize_request**
> ::models::InlineResponse20060 destiny2_awa_initialize_request(ctx, )


Initialize a request to perform an advanced write action.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20060**](inline_response_200_60.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_awa_provide_authorization_result**
> ::models::InlineResponse20023 destiny2_awa_provide_authorization_result()


Provide the result of the user interaction. Called by the Bungie Destiny App to approve or reject a request.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_equip_item**
> ::models::InlineResponse20023 destiny2_equip_item(ctx, )


Equip an item. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_equip_items**
> ::models::InlineResponse20046 destiny2_equip_items(ctx, )


Equip a list of items by itemInstanceIds. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline. Any items not found on your character will be ignored.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20046**](inline_response_200_46.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_activity_history**
> ::models::InlineResponse20055 destiny2_get_activity_history(character_id, destiny_membership_id, membership_type, optional)


Gets activity history stats for indicated character.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The id of the character to retrieve. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The id of the character to retrieve. | 
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **count** | **i32**| Number of rows to return | 
 **mode** | **i32**| A filter for the activity mode to be returned. None returns all activities. See the documentation for DestinyActivityModeType for valid values, and pass in string representation. | 
 **page** | **i32**| Page number to return, starting with 0. | 

### Return type

[**::models::InlineResponse20055**](inline_response_200_55.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_character**
> ::models::InlineResponse20039 destiny2_get_character(character_id, destiny_membership_id, membership_type, optional)


Returns character information for the supplied character.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| ID of the character. | 
  **destiny_membership_id** | **i64**| Destiny membership ID. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| ID of the character. | 
 **destiny_membership_id** | **i64**| Destiny membership ID. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **components** | [**Vec&lt;i32&gt;**](i32.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20039**](inline_response_200_39.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

# **destiny2_get_clan_weekly_reward_state**
> ::models::InlineResponse20040 destiny2_get_clan_weekly_reward_state(group_id)


Returns information on the weekly clan rewards and if the clan has earned them or not. Note that this will always report rewards as not redeemed.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_id** | **i64**| A valid group id of clan. | 

### Return type

[**::models::InlineResponse20040**](inline_response_200_40.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_collectible_node_details**
> ::models::InlineResponse20045 destiny2_get_collectible_node_details(character_id, collectible_presentation_node_hash, destiny_membership_id, membership_type, optional)


Given a Presentation Node that has Collectibles as direct descendants, this will return item details about those descendants in the context of the requesting character.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting collectible detail info. | 
  **collectible_presentation_node_hash** | **i32**| The hash identifier of the Presentation Node for whom we should return collectible details. Details will only be returned for collectibles that are direct descendants of this node. | 
  **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting collectible detail info. | 
 **collectible_presentation_node_hash** | **i32**| The hash identifier of the Presentation Node for whom we should return collectible details. Details will only be returned for collectibles that are direct descendants of this node. | 
 **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **components** | [**Vec&lt;i32&gt;**](i32.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20045**](inline_response_200_45.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_destiny_aggregate_activity_stats**
> ::models::InlineResponse20057 destiny2_get_destiny_aggregate_activity_stats(character_id, destiny_membership_id, membership_type)


Gets all activities the character has participated in together with aggregate statistics for those activities.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The specific character whose activities should be returned. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 

### Return type

[**::models::InlineResponse20057**](inline_response_200_57.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_destiny_entity_definition**
> ::models::InlineResponse20035 destiny2_get_destiny_entity_definition(entity_type, hash_identifier)


Returns the static definition of an entity of the given Type and hash identifier. Examine the API Documentation for the Type Names of entities that have their own definitions. Note that the return type will always *inherit from* DestinyDefinition, but the specific type returned will be the requested entity type if it can be found. Please don't use this as a chatty alternative to the Manifest database if you require large sets of data, but for simple and one-off accesses this should be handy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **entity_type** | **String**| The type of entity for whom you would like results. These correspond to the entity&#39;s definition contract name. For instance, if you are looking for items, this property should be &#39;DestinyInventoryItemDefinition&#39;. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is tentatively in final form, but there may be bugs that prevent desirable operation. | 
  **hash_identifier** | **i32**| The hash identifier for the specific Entity you want returned. | 

### Return type

[**::models::InlineResponse20035**](inline_response_200_35.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_destiny_manifest**
> ::models::InlineResponse20034 destiny2_get_destiny_manifest()


Returns the current version of the manifest as a json object.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20034**](inline_response_200_34.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_historical_stats**
> ::models::InlineResponse20053 destiny2_get_historical_stats(character_id, destiny_membership_id, membership_type, optional)


Gets historical stats for indicated character.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The id of the character to retrieve. You can omit this character ID or set it to 0 to get aggregate stats across all characters. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The id of the character to retrieve. You can omit this character ID or set it to 0 to get aggregate stats across all characters. | 
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **dayend** | **String**| Last day to return when daily stats are requested. Use the format YYYY-MM-DD. Currently, we cannot allow more than 31 days of daily data to be requested in a single request. | 
 **daystart** | **String**| First day to return when daily stats are requested. Use the format YYYY-MM-DD. Currently, we cannot allow more than 31 days of daily data to be requested in a single request. | 
 **groups** | [**Vec&lt;i32&gt;**](i32.md)| Group of stats to include, otherwise only general stats are returned. Comma separated list is allowed. Values: General, Weapons, Medals | 
 **modes** | [**Vec&lt;i32&gt;**](i32.md)| Game modes to return. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **period_type** | **i32**| Indicates a specific period type to return. Optional. May be: Daily, AllTime, or Activity | 

### Return type

[**::models::InlineResponse20053**](inline_response_200_53.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_historical_stats_definition**
> ::models::InlineResponse20049 destiny2_get_historical_stats_definition()


Gets historical stats definitions.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20049**](inline_response_200_49.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_historical_stats_for_account**
> ::models::InlineResponse20054 destiny2_get_historical_stats_for_account(destiny_membership_id, membership_type, optional)


Gets aggregate historical stats organized around each character for a given account.

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
 **groups** | [**Vec&lt;i32&gt;**](i32.md)| Groups of stats to include, otherwise only general stats are returned. Comma separated list is allowed. Values: General, Weapons, Medals. | 

### Return type

[**::models::InlineResponse20054**](inline_response_200_54.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_item**
> ::models::InlineResponse20041 destiny2_get_item(destiny_membership_id, item_instance_id, membership_type, optional)


Retrieve the details of an instanced Destiny Item. An instanced Destiny item is one with an ItemInstanceId. Non-instanced items, such as materials, have no useful instance-specific details and thus are not queryable here.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **destiny_membership_id** | **i64**| The membership ID of the destiny profile. | 
  **item_instance_id** | **i64**| The Instance ID of the destiny item. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **destiny_membership_id** | **i64**| The membership ID of the destiny profile. | 
 **item_instance_id** | **i64**| The Instance ID of the destiny item. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **components** | [**Vec&lt;i32&gt;**](i32.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20041**](inline_response_200_41.md)

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

# **destiny2_get_linked_profiles**
> ::models::InlineResponse20037 destiny2_get_linked_profiles(membership_id, membership_type, optional)


Returns a summary information about all profiles linked to the requesting membership type/membership ID that have valid Destiny information. The passed-in Membership Type/Membership ID may be a Bungie.Net membership or a Destiny membership. It only returns the minimal amount of data to begin making more substantive requests, but will hopefully serve as a useful alternative to UserServices for people who just care about Destiny data. Note that it will only return linked accounts whose linkages you are allowed to view.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **membership_id** | **i64**| The ID of the membership whose linked Destiny accounts you want returned. Make sure your membership ID matches its Membership Type: don&#39;t pass us a PSN membership ID and the XBox membership type, it&#39;s not going to work! | 
  **membership_type** | **i32**| The type for the membership whose linked Destiny accounts you want returned. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **membership_id** | **i64**| The ID of the membership whose linked Destiny accounts you want returned. Make sure your membership ID matches its Membership Type: don&#39;t pass us a PSN membership ID and the XBox membership type, it&#39;s not going to work! | 
 **membership_type** | **i32**| The type for the membership whose linked Destiny accounts you want returned. | 
 **get_all_memberships** | **bool**| (optional) if set to &#39;true&#39;, all memberships regardless of whether they&#39;re obscured by overrides will be returned. Normal privacy restrictions on account linking will still apply no matter what. | 

### Return type

[**::models::InlineResponse20037**](inline_response_200_37.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_post_game_carnage_report**
> ::models::InlineResponse20048 destiny2_get_post_game_carnage_report(activity_id)


Gets the available post game carnage report for the activity ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **activity_id** | **i64**| The ID of the activity whose PGCR is requested. | 

### Return type

[**::models::InlineResponse20048**](inline_response_200_48.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_profile**
> ::models::InlineResponse20038 destiny2_get_profile(destiny_membership_id, membership_type, optional)


Returns Destiny Profile information for the supplied membership.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **destiny_membership_id** | **i64**| Destiny membership ID. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **destiny_membership_id** | **i64**| Destiny membership ID. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **components** | [**Vec&lt;i32&gt;**](i32.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20038**](inline_response_200_38.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_public_milestone_content**
> ::models::InlineResponse20058 destiny2_get_public_milestone_content(milestone_hash)


Gets custom localized content for the milestone of the given hash, if it exists.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **milestone_hash** | **i32**| The identifier for the milestone to be returned. | 

### Return type

[**::models::InlineResponse20058**](inline_response_200_58.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_public_milestones**
> ::models::InlineResponse20059 destiny2_get_public_milestones()


Gets public information about currently available Milestones.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20059**](inline_response_200_59.md)

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

# **destiny2_get_unique_weapon_history**
> ::models::InlineResponse20056 destiny2_get_unique_weapon_history(character_id, destiny_membership_id, membership_type)


Gets details about unique weapon usage, including all exotic weapons.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The id of the character to retrieve. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 

### Return type

[**::models::InlineResponse20056**](inline_response_200_56.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_vendor**
> ::models::InlineResponse20043 destiny2_get_vendor(character_id, destiny_membership_id, membership_type, vendor_hash, optional)


Get the details of a specific Vendor.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
  **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
  **vendor_hash** | **i32**| The Hash identifier of the Vendor to be returned. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
 **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **vendor_hash** | **i32**| The Hash identifier of the Vendor to be returned. | 
 **components** | [**Vec&lt;i32&gt;**](i32.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20043**](inline_response_200_43.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_vendors**
> ::models::InlineResponse20042 destiny2_get_vendors(character_id, destiny_membership_id, membership_type, optional)


Get currently available vendors from the list of vendors that can possibly have rotating inventory. Note that this does not include things like preview vendors and vendors-as-kiosks, neither of whom have rotating/dynamic inventories. Use their definitions as-is for those.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
  **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
 **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **components** | [**Vec&lt;i32&gt;**](i32.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20042**](inline_response_200_42.md)

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

# **destiny2_pull_from_postmaster**
> ::models::InlineResponse20023 destiny2_pull_from_postmaster(ctx, )


Extract an item from the Postmaster, with whatever implications that may entail. You must have a valid Destiny account. You must also pass BOTH a reference AND an instance ID if it's an instanced item.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_report_offensive_post_game_carnage_report_player**
> ::models::InlineResponse20023 destiny2_report_offensive_post_game_carnage_report_player(ctx, activity_id)


Report a player that you met in an activity that was engaging in ToS-violating activities. Both you and the offending player must have played in the activityId passed in. Please use this judiciously and only when you have strong suspicions of violation, pretty please.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **activity_id** | **i64**| The ID of the activity where you ran into the brigand that you&#39;re reporting. | 

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_search_destiny_entities**
> ::models::InlineResponse20052 destiny2_search_destiny_entities(search_term, _type, optional)


Gets a page list of Destiny items.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **search_term** | **String**| The string to use when searching for Destiny entities. | 
  **_type** | **String**| The type of entity for whom you would like results. These correspond to the entity&#39;s definition contract name. For instance, if you are looking for items, this property should be &#39;DestinyInventoryItemDefinition&#39;. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **search_term** | **String**| The string to use when searching for Destiny entities. | 
 **_type** | **String**| The type of entity for whom you would like results. These correspond to the entity&#39;s definition contract name. For instance, if you are looking for items, this property should be &#39;DestinyInventoryItemDefinition&#39;. | 
 **page** | **i32**| Page number to return, starting with 0. | 

### Return type

[**::models::InlineResponse20052**](inline_response_200_52.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_search_destiny_player**
> ::models::InlineResponse20036 destiny2_search_destiny_player(display_name, membership_type, optional)


Returns a list of Destiny memberships given a full Gamertag or PSN ID. Unless you pass returnOriginalProfile=true, this will return membership information for the users' Primary Cross Save Profile if they are engaged in cross save rather than any original Destiny profile that is now being overridden.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **display_name** | **String**| The full gamertag or PSN id of the player. Spaces and case are ignored. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type, or All. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **display_name** | **String**| The full gamertag or PSN id of the player. Spaces and case are ignored. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type, or All. | 
 **return_original_profile** | **bool**| (optional) If passed in and set to true, we will return the original Destiny Profile(s) linked to that gamertag, and not their currently active Destiny Profile. | 

### Return type

[**::models::InlineResponse20036**](inline_response_200_36.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_set_item_lock_state**
> ::models::InlineResponse20023 destiny2_set_item_lock_state(ctx, )


Set the Lock State for an instanced item. You must have a valid Destiny Account.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_transfer_item**
> ::models::InlineResponse20023 destiny2_transfer_item(ctx, )


Transfer an item to/from your vault. You must have a valid Destiny account. You must also pass BOTH a reference AND an instance ID if it's an instanced item. itshappening.gif

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

