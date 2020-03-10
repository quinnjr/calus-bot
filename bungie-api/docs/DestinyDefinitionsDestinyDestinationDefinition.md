# DestinyDefinitionsDestinyDestinationDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md) |  | [optional] [default to null]
**place_hash** | **i32** | The place that \&quot;owns\&quot; this Destination. Use this hash to look up the DestinyPlaceDefinition. | [optional] [default to null]
**default_freeroam_activity_hash** | **i32** | If this Destination has a default Free-Roam activity, this is the hash for that Activity. Use it to look up the DestinyActivityDefintion. | [optional] [default to null]
**activity_graph_entries** | [**Vec<::models::DestinyDefinitionsDestinyActivityGraphListEntryDefinition>**](Destiny.Definitions.DestinyActivityGraphListEntryDefinition.md) | If the Destination has default Activity Graphs (i.e. \&quot;Map\&quot;) that should be shown in the director, this is the list of those Graphs. At most, only one should be active at any given time for a Destination: these would represent, for example, different variants on a Map if the Destination is changing on a macro level based on game state. | [optional] [default to null]
**bubble_settings** | [**Vec<::models::DestinyDefinitionsDestinyDestinationBubbleSettingDefinition>**](Destiny.Definitions.DestinyDestinationBubbleSettingDefinition.md) | A Destination may have many \&quot;Bubbles\&quot; zones with human readable properties.  We don&#39;t get as much info as I&#39;d like about them - I&#39;d love to return info like where on the map they are located - but at least this gives you the name of those bubbles. bubbleSettings and bubbles both have the identical number of entries, and you should match up their indexes to provide matching bubble and bubbleSettings data.  DEPRECATED - Just use bubbles, it now has this data. | [optional] [default to null]
**bubbles** | [**Vec<::models::DestinyDefinitionsDestinyBubbleDefinition>**](Destiny.Definitions.DestinyBubbleDefinition.md) | This provides the unique identifiers for every bubble in the destination (only guaranteed unique within the destination), and any intrinsic properties of the bubble.  bubbleSettings and bubbles both have the identical number of entries, and you should match up their indexes to provide matching bubble and bubbleSettings data. | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


