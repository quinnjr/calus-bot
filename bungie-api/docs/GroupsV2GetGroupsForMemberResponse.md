# GroupsV2GetGroupsForMemberResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**are_all_memberships_inactive** | **::std::collections::HashMap<String, bool>** | A convenience property that indicates if every membership this user has that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save.   The key is the Group ID for the group being checked, and the value is true if the users&#39; memberships for that group are all inactive. | [optional] [default to null]
**results** | [**Vec<::models::GroupsV2GroupMembership>**](GroupsV2.GroupMembership.md) |  | [optional] [default to null]
**total_results** | **i32** |  | [optional] [default to null]
**has_more** | **bool** |  | [optional] [default to null]
**query** | [***::models::QueriesPagedQuery**](Queries.PagedQuery.md) |  | [optional] [default to null]
**replacement_continuation_token** | **String** |  | [optional] [default to null]
**use_total_results** | **bool** | If useTotalResults is true, then totalResults represents an accurate count.  If False, it does not, and may be estimated/only the size of the current page.  Either way, you should probably always only trust hasMore.  This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


