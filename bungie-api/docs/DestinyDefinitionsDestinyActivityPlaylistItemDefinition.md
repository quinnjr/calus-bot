# DestinyDefinitionsDestinyActivityPlaylistItemDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | **i32** | The hash identifier of the Activity that can be played. Use it to look up the DestinyActivityDefinition. | [optional] [default to null]
**direct_activity_mode_hash** | **i32** | If this playlist entry had an activity mode directly defined on it, this will be the hash of that mode. | [optional] [default to null]
**direct_activity_mode_type** | **i32** | If the playlist entry had an activity mode directly defined on it, this will be the enum value of that mode. | [optional] [default to null]
**activity_mode_hashes** | **Vec<i32>** | The hash identifiers for Activity Modes relevant to this entry. | [optional] [default to null]
**activity_mode_types** | **Vec<i32>** | The activity modes - if any - in enum form. Because we can&#39;t seem to escape the enums. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


