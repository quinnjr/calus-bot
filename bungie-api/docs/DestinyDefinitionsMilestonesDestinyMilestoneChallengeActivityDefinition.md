# DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | **i32** | The activity for which this challenge is active. | [optional] [default to null]
**challenges** | [**Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeDefinition>**](Destiny.Definitions.Milestones.DestinyMilestoneChallengeDefinition.md) |  | [optional] [default to null]
**activity_graph_nodes** | [**Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityGraphNodeEntry>**](Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityGraphNodeEntry.md) | If the activity and its challenge is visible on any of these nodes, it will be returned. | [optional] [default to null]
**phases** | [**Vec<::models::DestinyDefinitionsMilestonesDestinyMilestoneChallengeActivityPhase>**](Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityPhase.md) | Phases related to this activity, if there are any.  These will be listed in the order in which they will appear in the actual activity. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


