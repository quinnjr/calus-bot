# DestinyDefinitionsPresentationDestinyPresentationNodeDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md) |  | [optional] [default to null]
**original_icon** | **String** | The original icon for this presentation node, before we futzed with it. | [optional] [default to null]
**root_view_icon** | **String** | Some presentation nodes are meant to be explicitly shown on the \&quot;root\&quot; or \&quot;entry\&quot; screens for the feature to which they are related. You should use this icon when showing them on such a view, if you have a similar \&quot;entry point\&quot; view in your UI. If you don&#39;t have a UI, then I guess it doesn&#39;t matter either way does it? | [optional] [default to null]
**node_type** | **i32** |  | [optional] [default to null]
**scope** | **i32** | Indicates whether this presentation node&#39;s state is determined on a per-character or on an account-wide basis. | [optional] [default to null]
**objective_hash** | **i32** | If this presentation node shows a related objective (for instance, if it tracks the progress of its children), the objective being tracked is indicated here. | [optional] [default to null]
**completion_record_hash** | **i32** | If this presentation node has an associated \&quot;Record\&quot; that you can accomplish for completing its children, this is the identifier of that Record. | [optional] [default to null]
**children** | [***Value**](Value.md) | The child entities contained by this presentation node. | [optional] [default to null]
**display_style** | **i32** | A hint for how to display this presentation node when it&#39;s shown in a list. | [optional] [default to null]
**screen_style** | **i32** | A hint for how to display this presentation node when it&#39;s shown in its own detail screen. | [optional] [default to null]
**requirements** | [***Value**](Value.md) | The requirements for being able to interact with this presentation node and its children. | [optional] [default to null]
**disable_child_subscreen_navigation** | **bool** | If this presentation node has children, but the game doesn&#39;t let you inspect the details of those children, that is indicated here. | [optional] [default to null]
**parent_node_hashes** | **Vec<i32>** | A quick reference to presentation nodes that have this node as a child. (presentation nodes can be parented under multiple parents) | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


