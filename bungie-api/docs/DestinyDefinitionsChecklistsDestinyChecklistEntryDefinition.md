# DestinyDefinitionsChecklistsDestinyChecklistEntryDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **i32** | The identifier for this Checklist entry. Guaranteed unique only within this Checklist Definition, and not globally/for all checklists. | [optional] [default to null]
**display_properties** | [***Value**](Value.md) | Even if no other associations exist, we will give you *something* for display properties. In cases where we have no associated entities, it may be as simple as a numerical identifier. | [optional] [default to null]
**destination_hash** | **i32** |  | [optional] [default to null]
**location_hash** | **i32** |  | [optional] [default to null]
**bubble_hash** | **i32** | Note that a Bubble&#39;s hash doesn&#39;t uniquely identify a \&quot;top level\&quot; entity in Destiny. Only the combination of location and bubble can uniquely identify a place in the world of Destiny: so if bubbleHash is populated, locationHash must too be populated for it to have any meaning.  You can use this property if it is populated to look up the DestinyLocationDefinition&#39;s associated .locationReleases[].activityBubbleName property. | [optional] [default to null]
**activity_hash** | **i32** |  | [optional] [default to null]
**item_hash** | **i32** |  | [optional] [default to null]
**vendor_hash** | **i32** |  | [optional] [default to null]
**vendor_interaction_index** | **i32** |  | [optional] [default to null]
**scope** | **i32** | The scope at which this specific entry can be computed. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


