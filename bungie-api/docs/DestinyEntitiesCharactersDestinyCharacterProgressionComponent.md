# DestinyEntitiesCharactersDestinyCharacterProgressionComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**progressions** | [**::std::collections::HashMap<String, ::models::DestinyDestinyProgression>**](Destiny.DestinyProgression.md) | A Dictionary of all known progressions for the Character, keyed by the Progression&#39;s hash.  Not all progressions have user-facing data, but those who do will have that data contained in the DestinyProgressionDefinition. | [optional] [default to null]
**factions** | [**::std::collections::HashMap<String, ::models::DestinyProgressionDestinyFactionProgression>**](Destiny.Progression.DestinyFactionProgression.md) | A dictionary of all known Factions, keyed by the Faction&#39;s hash. It contains data about this character&#39;s status with the faction. | [optional] [default to null]
**milestones** | [**::std::collections::HashMap<String, ::models::DestinyMilestonesDestinyMilestone>**](Destiny.Milestones.DestinyMilestone.md) | Milestones are related to the simple progressions shown in the game, but return additional and hopefully helpful information for users about the specifics of the Milestone&#39;s status. | [optional] [default to null]
**quests** | [**Vec<::models::DestinyQuestsDestinyQuestStatus>**](Destiny.Quests.DestinyQuestStatus.md) | If the user has any active quests, the quests&#39; statuses will be returned here.   Note that quests have been largely supplanted by Milestones, but that doesn&#39;t mean that they won&#39;t make a comeback independent of milestones at some point.   (Fun fact: quests came back as I feared they would, but we never looped back to populate this... I&#39;m going to put that in the backlog.) | [optional] [default to null]
**uninstanced_item_objectives** | [**::std::collections::HashMap<String, Vec<::models::DestinyQuestsDestinyObjectiveProgress>>**](array.md) | Sometimes, you have items in your inventory that don&#39;t have instances, but still have Objective information. This provides you that objective information for uninstanced items.   This dictionary is keyed by the item&#39;s hash: which you can use to look up the name and description for the overall task(s) implied by the objective. The value is the list of objectives for this item, and their statuses. | [optional] [default to null]
**checklists** | [**::std::collections::HashMap<String, ::std::collections::HashMap<String, bool>>**](map.md) | The set of checklists that can be examined for this specific character, keyed by the hash identifier of the Checklist (DestinyChecklistDefinition)  For each checklist returned, its value is itself a Dictionary keyed by the checklist&#39;s hash identifier with the value being a boolean indicating if it&#39;s been discovered yet. | [optional] [default to null]
**seasonal_artifact** | [***Value**](Value.md) | Data related to your progress on the current season&#39;s artifact that can vary per character. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


