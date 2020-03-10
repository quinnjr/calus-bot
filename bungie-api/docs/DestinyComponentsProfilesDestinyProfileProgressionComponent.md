# DestinyComponentsProfilesDestinyProfileProgressionComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checklists** | [**::std::collections::HashMap<String, ::std::collections::HashMap<String, bool>>**](map.md) | The set of checklists that can be examined on a profile-wide basis, keyed by the hash identifier of the Checklist (DestinyChecklistDefinition)  For each checklist returned, its value is itself a Dictionary keyed by the checklist&#39;s hash identifier with the value being a boolean indicating if it&#39;s been discovered yet. | [optional] [default to null]
**seasonal_artifact** | [***Value**](Value.md) | Data related to your progress on the current season&#39;s artifact that is the same across characters. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


