# DestinyComponentsProfilesDestinyProfileTransitoryComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**party_members** | [**Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryPartyMember>**](Destiny.Components.Profiles.DestinyProfileTransitoryPartyMember.md) | If you have any members currently in your party, this is some (very) bare-bones information about those members. | [optional] [default to null]
**current_activity** | [***Value**](Value.md) | If you are in an activity, this is some transitory info about the activity currently being played. | [optional] [default to null]
**joinability** | [***Value**](Value.md) | Information about whether and what might prevent you from joining this person on a fireteam. | [optional] [default to null]
**tracking** | [**Vec<::models::DestinyComponentsProfilesDestinyProfileTransitoryTrackingEntry>**](Destiny.Components.Profiles.DestinyProfileTransitoryTrackingEntry.md) | Information about tracked entities. | [optional] [default to null]
**last_orbited_destination_hash** | **i32** | The hash identifier for the DestinyDestinationDefinition of the last location you were orbiting when in orbit. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


