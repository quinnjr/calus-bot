# DestinyMilestonesDestinyPublicMilestoneActivity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | **i32** | The hash identifier of the activity that&#39;s been chosen to be considered the canonical \&quot;conceptual\&quot; activity definition. This may have many variants, defined herein. | [optional] [default to null]
**modifier_hashes** | **Vec<i32>** | The activity may have 0-to-many modifiers: if it does, this will contain the hashes to the DestinyActivityModifierDefinition that defines the modifier being applied. | [optional] [default to null]
**variants** | [**Vec<::models::DestinyMilestonesDestinyPublicMilestoneActivityVariant>**](Destiny.Milestones.DestinyPublicMilestoneActivityVariant.md) | Every relevant variation of this conceptual activity, including the conceptual activity itself, have variants defined here. | [optional] [default to null]
**activity_mode_hash** | **i32** | The hash identifier of the most specific Activity Mode under which this activity is played. This is useful for situations where the activity in question is - for instance - a PVP map, but it&#39;s not clear what mode the PVP map is being played under. If it&#39;s a playlist, this will be less specific: but hopefully useful in some way. | [optional] [default to null]
**activity_mode_type** | **i32** | The enumeration equivalent of the most specific Activity Mode under which this activity is played. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


