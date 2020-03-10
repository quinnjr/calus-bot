# DestinyDestinyActivity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | **i32** | The hash identifier of the Activity. Use this to look up the DestinyActivityDefinition of the activity. | [optional] [default to null]
**is_new** | **bool** | If true, then the activity should have a \&quot;new\&quot; indicator in the Director UI. | [optional] [default to null]
**can_lead** | **bool** | If true, the user is allowed to lead a Fireteam into this activity. | [optional] [default to null]
**can_join** | **bool** | If true, the user is allowed to join with another Fireteam in this activity. | [optional] [default to null]
**is_completed** | **bool** | If true, we both have the ability to know that the user has completed this activity and they have completed it. Unfortunately, we can&#39;t necessarily know this for all activities. As such, this should probably only be used if you already know in advance which specific activities you wish to check. | [optional] [default to null]
**is_visible** | **bool** | If true, the user should be able to see this activity. | [optional] [default to null]
**display_level** | **i32** | The difficulty level of the activity, if applicable. | [optional] [default to null]
**recommended_light** | **i32** | The recommended light level for the activity, if applicable. | [optional] [default to null]
**difficulty_tier** | **i32** | A DestinyActivityDifficultyTier enum value indicating the difficulty of the activity. | [optional] [default to null]
**challenges** | [**Vec<::models::DestinyChallengesDestinyChallengeStatus>**](Destiny.Challenges.DestinyChallengeStatus.md) |  | [optional] [default to null]
**modifier_hashes** | **Vec<i32>** | If the activity has modifiers, this will be the list of modifiers that all variants have in common. Perform lookups against DestinyActivityModifierDefinition which defines the modifier being applied to get at the modifier data.  Note that, in the DestiyActivityDefinition, you will see many more modifiers than this being referred to: those are all *possible* modifiers for the activity, not the active ones. Use only the active ones to match what&#39;s really live. | [optional] [default to null]
**boolean_activity_options** | **::std::collections::HashMap<String, bool>** | The set of activity options for this activity, keyed by an identifier that&#39;s unique for this activity (not guaranteed to be unique between or across all activities, though should be unique for every *variant* of a given *conceptual* activity: for instance, the original D2 Raid has many variant DestinyActivityDefinitions. While other activities could potentially have the same option hashes, for any given D2 base Raid variant the hash will be unique).  As a concrete example of this data, the hashes you get for Raids will correspond to the currently active \&quot;Challenge Mode\&quot;.  We don&#39;t have any human readable information for these, but saavy 3rd party app users could manually associate the key (a hash identifier for the \&quot;option\&quot; that is enabled/disabled) and the value (whether it&#39;s enabled or disabled presently)  On our side, we don&#39;t necessarily even know what these are used for (the game designers know, but we don&#39;t), and we have no human readable data for them. In order to use them, you will have to do some experimentation. | [optional] [default to null]
**loadout_requirement_index** | **i32** | If returned, this is the index into the DestinyActivityDefinition&#39;s \&quot;loadouts\&quot; property, indicating the currently active loadout requirements. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


