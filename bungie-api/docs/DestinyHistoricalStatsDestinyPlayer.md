# DestinyHistoricalStatsDestinyPlayer

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destiny_user_info** | [***Value**](Value.md) | Details about the player as they are known in game (platform display name, Destiny emblem) | [optional] [default to null]
**character_class** | **String** | Class of the character if applicable and available. | [optional] [default to null]
**class_hash** | **i32** |  | [optional] [default to null]
**race_hash** | **i32** |  | [optional] [default to null]
**gender_hash** | **i32** |  | [optional] [default to null]
**character_level** | **i32** | Level of the character if available. Zero if it is not available. | [optional] [default to null]
**light_level** | **i32** | Light Level of the character if available. Zero if it is not available. | [optional] [default to null]
**bungie_net_user_info** | [***Value**](Value.md) | Details about the player as they are known on BungieNet. This will be undefined if the player has marked their credential private, or does not have a BungieNet account. | [optional] [default to null]
**clan_name** | **String** | Current clan name for the player. This value may be null or an empty string if the user does not have a clan. | [optional] [default to null]
**clan_tag** | **String** | Current clan tag for the player. This value may be null or an empty string if the user does not have a clan. | [optional] [default to null]
**emblem_hash** | **i32** | If we know the emblem&#39;s hash, this can be used to look up the player&#39;s emblem at the time of a match when receiving PGCR data, or otherwise their currently equipped emblem (if we are able to obtain it). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


