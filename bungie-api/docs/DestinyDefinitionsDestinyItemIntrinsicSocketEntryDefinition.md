# DestinyDefinitionsDestinyItemIntrinsicSocketEntryDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plug_item_hash** | **i32** | Indicates the plug that is intrinsically inserted into this socket. | [optional] [default to null]
**socket_type_hash** | **i32** | Indicates the type of this intrinsic socket. | [optional] [default to null]
**default_visible** | **bool** | If true, then this socket is visible in the item&#39;s \&quot;default\&quot; state. If you have an instance, you should always check the runtime state, as that can override this visibility setting: but if you&#39;re looking at the item on a conceptual level, this property can be useful for hiding data such as legacy sockets - which remain defined on items for infrastructure purposes, but can be confusing for users to see. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


