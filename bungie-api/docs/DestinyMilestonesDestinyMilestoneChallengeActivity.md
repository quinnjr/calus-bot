# DestinyMilestonesDestinyMilestoneChallengeActivity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | **i32** |  | [optional] [default to null]
**challenges** | [**Vec<::models::DestinyChallengesDestinyChallengeStatus>**](Destiny.Challenges.DestinyChallengeStatus.md) |  | [optional] [default to null]
**modifier_hashes** | **Vec<i32>** | If the activity has modifiers, this will be the list of modifiers that all variants have in common. Perform lookups against DestinyActivityModifierDefinition which defines the modifier being applied to get at the modifier data.  Note that, in the DestiyActivityDefinition, you will see many more modifiers than this being referred to: those are all *possible* modifiers for the activity, not the active ones. Use only the active ones to match what&#39;s really live. | [optional] [default to null]
**boolean_activity_options** | **::std::collections::HashMap<String, bool>** | The set of activity options for this activity, keyed by an identifier that&#39;s unique for this activity (not guaranteed to be unique between or across all activities, though should be unique for every *variant* of a given *conceptual* activity: for instance, the original D2 Raid has many variant DestinyActivityDefinitions. While other activities could potentially have the same option hashes, for any given D2 base Raid variant the hash will be unique).  As a concrete example of this data, the hashes you get for Raids will correspond to the currently active \&quot;Challenge Mode\&quot;.  We don&#39;t have any human readable information for these, but saavy 3rd party app users could manually associate the key (a hash identifier for the \&quot;option\&quot; that is enabled/disabled) and the value (whether it&#39;s enabled or disabled presently)  On our side, we don&#39;t necessarily even know what these are used for (the game designers know, but we don&#39;t), and we have no human readable data for them. In order to use them, you will have to do some experimentation. | [optional] [default to null]
**loadout_requirement_index** | **i32** | If returned, this is the index into the DestinyActivityDefinition&#39;s \&quot;loadouts\&quot; property, indicating the currently active loadout requirements. | [optional] [default to null]
**phases** | [**Vec<::models::DestinyMilestonesDestinyMilestoneActivityPhase>**](Destiny.Milestones.DestinyMilestoneActivityPhase.md) | If the Activity has discrete \&quot;phases\&quot; that we can track, that info will be here. Otherwise, this value will be NULL. Note that this is a list and not a dictionary: the order implies the ascending order of phases or progression in this activity. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


