# DestinyDefinitionsDestinyObjectiveDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***Value**](Value.md) | Ideally, this should tell you what your task is. I&#39;m not going to lie to you though. Sometimes this doesn&#39;t have useful information at all. Which sucks, but there&#39;s nothing either of us can do about it. | [optional] [default to null]
**completion_value** | **i32** | The value that the unlock value defined in unlockValueHash must reach in order for the objective to be considered Completed. Used in calculating progress and completion status. | [optional] [default to null]
**scope** | **i32** | A shortcut for determining the most restrictive gating that this Objective is set to use. This includes both the dynamic determination of progress and of completion values. See the DestinyGatingScope enum&#39;s documentation for more details. | [optional] [default to null]
**location_hash** | **i32** | OPTIONAL: a hash identifier for the location at which this objective must be accomplished, if there is a location defined. Look up the DestinyLocationDefinition for this hash for that additional location info. | [optional] [default to null]
**allow_negative_value** | **bool** | If true, the value is allowed to go negative. | [optional] [default to null]
**allow_value_change_when_completed** | **bool** | If true, you can effectively \&quot;un-complete\&quot; this objective if you lose progress after crossing the completion threshold.   If False, once you complete the task it will remain completed forever by locking the value. | [optional] [default to null]
**is_counting_downward** | **bool** | If true, completion means having an unlock value less than or equal to the completionValue.  If False, completion means having an unlock value greater than or equal to the completionValue. | [optional] [default to null]
**value_style** | **i32** | The UI style applied to the objective. It&#39;s an enum, take a look at DestinyUnlockValueUIStyle for details of the possible styles. Use this info as you wish to customize your UI. | [optional] [default to null]
**progress_description** | **String** | Text to describe the progress bar. | [optional] [default to null]
**perks** | [***Value**](Value.md) | If this objective enables Perks intrinsically, the conditions for that enabling are defined here. | [optional] [default to null]
**stats** | [***Value**](Value.md) | If this objective enables modifications on a player&#39;s stats intrinsically, the conditions are defined here. | [optional] [default to null]
**minimum_visibility_threshold** | **i32** | If nonzero, this is the minimum value at which the objective&#39;s progression should be shown. Otherwise, don&#39;t show it yet. | [optional] [default to null]
**allow_overcompletion** | **bool** | If True, the progress will continue even beyond the point where the objective met its minimum completion requirements. Your UI will have to accommodate it. | [optional] [default to null]
**show_value_on_complete** | **bool** | If True, you should continue showing the progression value in the UI after it&#39;s complete. I mean, we already do that in BNet anyways, but if you want to be better behaved than us you could honor this flag. | [optional] [default to null]
**completed_value_style** | **i32** | The style to use when the objective is completed. | [optional] [default to null]
**in_progress_value_style** | **i32** | The style to use when the objective is still in progress. | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


