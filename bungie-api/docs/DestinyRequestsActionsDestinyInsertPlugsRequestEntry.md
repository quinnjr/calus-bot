# DestinyRequestsActionsDestinyInsertPlugsRequestEntry

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**socket_index** | **i32** | The index into the socket array, which identifies the specific socket being operated on. We also need to know the socketArrayType in order to uniquely identify the socket.  Don&#39;t point to or try to insert a plug into an infusion socket. It won&#39;t work. | [optional] [default to null]
**socket_array_type** | **i32** | This property, combined with the socketIndex, tells us which socket we are referring to (since operations can be performed on both Intrinsic and \&quot;default\&quot; sockets, and they occupy different arrays in the Inventory Item Definition). I know, I know. Don&#39;t give me that look. | [optional] [default to null]
**plug_item_hash** | **i32** | Plugs are never instanced (except in infusion). So with the hash alone, we should be able to: 1) Infer whether the player actually needs to have the item, or if it&#39;s a reusable plug 2) Perform any operation needed to use the Plug, including removing the plug item and running reward sheets. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


