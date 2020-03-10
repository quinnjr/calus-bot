# \ForumApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**forum_get_core_topics_paged**](ForumApi.md#forum_get_core_topics_paged) | **Get** /Forum/GetCoreTopicsPaged/{page}/{sort}/{quickDate}/{categoryFilter}/ | 
[**forum_get_forum_tag_suggestions**](ForumApi.md#forum_get_forum_tag_suggestions) | **Get** /Forum/GetForumTagSuggestions/ | 
[**forum_get_poll**](ForumApi.md#forum_get_poll) | **Get** /Forum/Poll/{topicId}/ | 
[**forum_get_post_and_parent**](ForumApi.md#forum_get_post_and_parent) | **Get** /Forum/GetPostAndParent/{childPostId}/ | 
[**forum_get_post_and_parent_awaiting_approval**](ForumApi.md#forum_get_post_and_parent_awaiting_approval) | **Get** /Forum/GetPostAndParentAwaitingApproval/{childPostId}/ | 
[**forum_get_posts_threaded_paged**](ForumApi.md#forum_get_posts_threaded_paged) | **Get** /Forum/GetPostsThreadedPaged/{parentPostId}/{page}/{pageSize}/{replySize}/{getParentPost}/{rootThreadMode}/{sortMode}/ | 
[**forum_get_posts_threaded_paged_from_child**](ForumApi.md#forum_get_posts_threaded_paged_from_child) | **Get** /Forum/GetPostsThreadedPagedFromChild/{childPostId}/{page}/{pageSize}/{replySize}/{rootThreadMode}/{sortMode}/ | 
[**forum_get_recruitment_thread_summaries**](ForumApi.md#forum_get_recruitment_thread_summaries) | **Post** /Forum/Recruit/Summaries/ | 
[**forum_get_topic_for_content**](ForumApi.md#forum_get_topic_for_content) | **Get** /Forum/GetTopicForContent/{contentId}/ | 
[**forum_get_topics_paged**](ForumApi.md#forum_get_topics_paged) | **Get** /Forum/GetTopicsPaged/{page}/{pageSize}/{group}/{sort}/{quickDate}/{categoryFilter}/ | 


# **forum_get_core_topics_paged**
> ::models::InlineResponse20012 forum_get_core_topics_paged(category_filter, page, quick_date, sort, optional)


Gets a listing of all topics marked as part of the core group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **category_filter** | **i32**| The category filter. | 
  **page** | **i32**| Zero base page | 
  **quick_date** | **i32**| The date filter. | 
  **sort** | **i32**| The sort mode. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **category_filter** | **i32**| The category filter. | 
 **page** | **i32**| Zero base page | 
 **quick_date** | **i32**| The date filter. | 
 **sort** | **i32**| The sort mode. | 
 **locales** | **String**| Comma seperated list of locales posts must match to return in the result list. Default &#39;en&#39; | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_forum_tag_suggestions**
> ::models::InlineResponse20014 forum_get_forum_tag_suggestions(optional)


Gets tag suggestions based on partial text entry, matching them with other tags previously used in the forums.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **partialtag** | **String**| The partial tag input to generate suggestions from. | 

### Return type

[**::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_poll**
> ::models::InlineResponse20012 forum_get_poll(topic_id)


Gets the specified forum poll.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **topic_id** | **i64**| The post id of the topic that has the poll. | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_post_and_parent**
> ::models::InlineResponse20012 forum_get_post_and_parent(child_post_id, optional)


Returns the post specified and its immediate parent.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **child_post_id** | **i64**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **child_post_id** | **i64**|  | 
 **showbanned** | **String**| If this value is not null or empty, banned posts are requested to be returned | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_post_and_parent_awaiting_approval**
> ::models::InlineResponse20012 forum_get_post_and_parent_awaiting_approval(child_post_id, optional)


Returns the post specified and its immediate parent of posts that are awaiting approval.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **child_post_id** | **i64**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **child_post_id** | **i64**|  | 
 **showbanned** | **String**| If this value is not null or empty, banned posts are requested to be returned | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_posts_threaded_paged**
> ::models::InlineResponse20012 forum_get_posts_threaded_paged(get_parent_post, page, page_size, parent_post_id, reply_size, root_thread_mode, sort_mode, optional)


Returns a thread of posts at the given parent, optionally returning replies to those posts as well as the original parent.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **get_parent_post** | **bool**|  | 
  **page** | **i32**|  | 
  **page_size** | **i32**|  | 
  **parent_post_id** | **i64**|  | 
  **reply_size** | **i32**|  | 
  **root_thread_mode** | **bool**|  | 
  **sort_mode** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **get_parent_post** | **bool**|  | 
 **page** | **i32**|  | 
 **page_size** | **i32**|  | 
 **parent_post_id** | **i64**|  | 
 **reply_size** | **i32**|  | 
 **root_thread_mode** | **bool**|  | 
 **sort_mode** | **i32**|  | 
 **showbanned** | **String**| If this value is not null or empty, banned posts are requested to be returned | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_posts_threaded_paged_from_child**
> ::models::InlineResponse20012 forum_get_posts_threaded_paged_from_child(child_post_id, page, page_size, reply_size, root_thread_mode, sort_mode, optional)


Returns a thread of posts starting at the topicId of the input childPostId, optionally returning replies to those posts as well as the original parent.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **child_post_id** | **i64**|  | 
  **page** | **i32**|  | 
  **page_size** | **i32**|  | 
  **reply_size** | **i32**|  | 
  **root_thread_mode** | **bool**|  | 
  **sort_mode** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **child_post_id** | **i64**|  | 
 **page** | **i32**|  | 
 **page_size** | **i32**|  | 
 **reply_size** | **i32**|  | 
 **root_thread_mode** | **bool**|  | 
 **sort_mode** | **i32**|  | 
 **showbanned** | **String**| If this value is not null or empty, banned posts are requested to be returned | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_recruitment_thread_summaries**
> ::models::InlineResponse20015 forum_get_recruitment_thread_summaries()


Allows the caller to get a list of to 25 recruitment thread summary information objects.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_topic_for_content**
> ::models::InlineResponse20013 forum_get_topic_for_content(content_id)


Gets the post Id for the given content item's comments, if it exists.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **content_id** | **i64**|  | 

### Return type

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **forum_get_topics_paged**
> ::models::InlineResponse20012 forum_get_topics_paged(category_filter, group, page, page_size, quick_date, sort, optional)


Get topics from any forum.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **category_filter** | **i32**| A category filter | 
  **group** | **i64**| The group, if any. | 
  **page** | **i32**| Zero paged page number | 
  **page_size** | **i32**| Unused | 
  **quick_date** | **i32**| A date filter. | 
  **sort** | **i32**| The sort mode. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **category_filter** | **i32**| A category filter | 
 **group** | **i64**| The group, if any. | 
 **page** | **i32**| Zero paged page number | 
 **page_size** | **i32**| Unused | 
 **quick_date** | **i32**| A date filter. | 
 **sort** | **i32**| The sort mode. | 
 **locales** | **String**| Comma seperated list of locales posts must match to return in the result list. Default &#39;en&#39; | 
 **tagstring** | **String**| The tags to search, if any. | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

