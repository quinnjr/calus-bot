# DestinyResponsesDestinyCharacterResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inventory** | [***Value**](Value.md) | The character-level non-equipped inventory items.  COMPONENT TYPE: CharacterInventories | [optional] [default to null]
**character** | [***Value**](Value.md) | Base information about the character in question.  COMPONENT TYPE: Characters | [optional] [default to null]
**progressions** | [***Value**](Value.md) | Character progression data, including Milestones.  COMPONENT TYPE: CharacterProgressions | [optional] [default to null]
**render_data** | [***Value**](Value.md) | Character rendering data - a minimal set of information about equipment and dyes used for rendering.  COMPONENT TYPE: CharacterRenderData | [optional] [default to null]
**activities** | [***Value**](Value.md) | Activity data - info about current activities available to the player.  COMPONENT TYPE: CharacterActivities | [optional] [default to null]
**equipment** | [***Value**](Value.md) | Equipped items on the character.  COMPONENT TYPE: CharacterEquipment | [optional] [default to null]
**kiosks** | [***Value**](Value.md) | Items available from Kiosks that are available to this specific character.   COMPONENT TYPE: Kiosks | [optional] [default to null]
**plug_sets** | [***Value**](Value.md) | When sockets refer to reusable Plug Sets (see DestinyPlugSetDefinition for more info), this is the set of plugs and their states that are scoped to this character.  This comes back with ItemSockets, as it is needed for a complete picture of the sockets on requested items.  COMPONENT TYPE: ItemSockets | [optional] [default to null]
**presentation_nodes** | [***Value**](Value.md) | COMPONENT TYPE: PresentationNodes | [optional] [default to null]
**records** | [***Value**](Value.md) | COMPONENT TYPE: Records | [optional] [default to null]
**collectibles** | [***Value**](Value.md) | COMPONENT TYPE: Collectibles | [optional] [default to null]
**item_components** | [***Value**](Value.md) | The set of components belonging to the player&#39;s instanced items.  COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.] | [optional] [default to null]
**uninstanced_item_components** | [***Value**](Value.md) | The set of components belonging to the player&#39;s UNinstanced items. Because apparently now those too can have information relevant to the character&#39;s state.  COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.] | [optional] [default to null]
**currency_lookups** | [***Value**](Value.md) | A \&quot;lookup\&quot; convenience component that can be used to quickly check if the character has access to items that can be used for purchasing.  COMPONENT TYPE: CurrencyLookups | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


