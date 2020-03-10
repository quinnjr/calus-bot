# DestinyEntitiesProfilesDestinyProfileComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_info** | [***Value**](Value.md) | If you need to render the Profile (their platform name, icon, etc...) somewhere, this property contains that information. | [optional] [default to null]
**date_last_played** | **String** | The last time the user played with any character on this Profile. | [optional] [default to null]
**versions_owned** | **i32** | If you want to know what expansions they own, this will contain that data.   IMPORTANT: This field may not return the data you&#39;re interested in for Cross-Saved users. It returns the last ownership data we saw for this account - which is to say, what they&#39;ve purchased on the platform on which they last played, which now could be a different platform.   If you don&#39;t care about per-platform ownership and only care about whatever platform it seems they are playing on most recently, then this should be \&quot;good enough.\&quot; Otherwise, this should be considered deprecated. We do not have a good alternative to provide at this time with platform specific ownership data for DLC. | [optional] [default to null]
**character_ids** | **Vec<i64>** | A list of the character IDs, for further querying on your part. | [optional] [default to null]
**season_hashes** | **Vec<i32>** | A list of seasons that this profile owns. Unlike versionsOwned, these stay with the profile across Platforms, and thus will be valid.   It turns out that Stadia Pro subscriptions will give access to seasons but only while playing on Stadia and with an active subscription. So some users (users who have Stadia Pro but choose to play on some other platform) won&#39;t see these as available: it will be whatever seasons are available for the platform on which they last played. | [optional] [default to null]
**current_season_hash** | **i32** | If populated, this is a reference to the season that is currently active. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


