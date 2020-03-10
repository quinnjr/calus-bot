# DestinyResponsesDestinyItemResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**character_id** | **i64** | If the item is on a character, this will return the ID of the character that is holding the item. | [optional] [default to null]
**item** | [***Value**](Value.md) | Common data for the item relevant to its non-instanced properties.  COMPONENT TYPE: ItemCommonData | [optional] [default to null]
**instance** | [***Value**](Value.md) | Basic instance data for the item.  COMPONENT TYPE: ItemInstances | [optional] [default to null]
**objectives** | [***Value**](Value.md) | Information specifically about the item&#39;s objectives.  COMPONENT TYPE: ItemObjectives | [optional] [default to null]
**perks** | [***Value**](Value.md) | Information specifically about the perks currently active on the item.  COMPONENT TYPE: ItemPerks | [optional] [default to null]
**render_data** | [***Value**](Value.md) | Information about how to render the item in 3D.  COMPONENT TYPE: ItemRenderData | [optional] [default to null]
**stats** | [***Value**](Value.md) | Information about the computed stats of the item: power, defense, etc...  COMPONENT TYPE: ItemStats | [optional] [default to null]
**talent_grid** | [***Value**](Value.md) | Information about the talent grid attached to the item. Talent nodes can provide a variety of benefits and abilities, and in Destiny 2 are used almost exclusively for the character&#39;s \&quot;Builds\&quot;.  COMPONENT TYPE: ItemTalentGrids | [optional] [default to null]
**sockets** | [***Value**](Value.md) | Information about the sockets of the item: which are currently active, what potential sockets you could have and the stats/abilities/perks you can gain from them.  COMPONENT TYPE: ItemSockets | [optional] [default to null]
**reusable_plugs** | [***Value**](Value.md) | Information about the Reusable Plugs for sockets on an item. These are plugs that you can insert into the given socket regardless of if you actually own an instance of that plug: they are logic-driven plugs rather than inventory-driven.   These may need to be combined with Plug Set component data to get a full picture of available plugs on a given socket.   COMPONENT TYPE: ItemReusablePlugs | [optional] [default to null]
**plug_objectives** | [***Value**](Value.md) | Information about objectives on Plugs for a given item. See the component&#39;s documentation for more info.  COMPONENT TYPE: ItemPlugObjectives | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


