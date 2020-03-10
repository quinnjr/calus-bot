# DestinyDefinitionsArtifactsDestinyArtifactTierDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tier_hash** | **i32** | An identifier, unique within the Artifact, for this specific tier. | [optional] [default to null]
**display_title** | **String** | The human readable title of this tier, if any. | [optional] [default to null]
**progress_requirement_message** | **String** | A string representing the localized minimum requirement text for this Tier, if any. | [optional] [default to null]
**items** | [**Vec<::models::DestinyDefinitionsArtifactsDestinyArtifactTierItemDefinition>**](Destiny.Definitions.Artifacts.DestinyArtifactTierItemDefinition.md) | The items that can be earned within this tier. | [optional] [default to null]
**minimum_unlock_points_used_requirement** | **i32** | The minimum number of \&quot;unlock points\&quot; that you must have used before you can unlock items from this tier. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


