# DestinyComponentsCollectiblesDestinyProfileCollectiblesComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recent_collectible_hashes** | **Vec<i32>** | The list of collectibles determined by the game as having been \&quot;recently\&quot; acquired. | [optional] [default to null]
**newness_flagged_collectible_hashes** | **Vec<i32>** | The list of collectibles determined by the game as having been \&quot;recently\&quot; acquired.  The game client itself actually controls this data, so I personally question whether anyone will get much use out of this: because we can&#39;t edit this value through the API. But in case anyone finds it useful, here it is. | [optional] [default to null]
**collectibles** | [**::std::collections::HashMap<String, ::models::DestinyComponentsCollectiblesDestinyCollectibleComponent>**](Destiny.Components.Collectibles.DestinyCollectibleComponent.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


