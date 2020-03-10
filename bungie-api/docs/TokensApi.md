# \TokensApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_apply_missing_partner_offers_without_claim**](TokensApi.md#tokens_apply_missing_partner_offers_without_claim) | **Post** /Tokens/Partner/ApplyMissingOffers/{partnerApplicationId}/{targetBnetMembershipId}/ | 
[**tokens_claim_partner_offer**](TokensApi.md#tokens_claim_partner_offer) | **Post** /Tokens/Partner/ClaimOffer/ | 
[**tokens_get_partner_offer_sku_history**](TokensApi.md#tokens_get_partner_offer_sku_history) | **Get** /Tokens/Partner/History/{partnerApplicationId}/{targetBnetMembershipId}/ | 


# **tokens_apply_missing_partner_offers_without_claim**
> ::models::InlineResponse20018 tokens_apply_missing_partner_offers_without_claim(ctx, partner_application_id, target_bnet_membership_id)


Apply a partner offer to the targeted user. This endpoint does not claim a new offer, but any already claimed offers will be applied to the game if not already.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **partner_application_id** | **i32**| The partner application identifier. | 
  **target_bnet_membership_id** | **i64**| The bungie.net user to apply missing offers to. If not self, elevated permissions are required. | 

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **tokens_claim_partner_offer**
> ::models::InlineResponse20018 tokens_claim_partner_offer(ctx, )


Claim a partner offer as the authenticated user.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **tokens_get_partner_offer_sku_history**
> ::models::InlineResponse20033 tokens_get_partner_offer_sku_history(ctx, partner_application_id, target_bnet_membership_id)


Returns the partner sku and offer history of the targeted user. Elevated permissions are required to see users that are not yourself.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **partner_application_id** | **i32**| The partner application identifier. | 
  **target_bnet_membership_id** | **i64**| The bungie.net user to apply missing offers to. If not self, elevated permissions are required. | 

### Return type

[**::models::InlineResponse20033**](inline_response_200_33.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

