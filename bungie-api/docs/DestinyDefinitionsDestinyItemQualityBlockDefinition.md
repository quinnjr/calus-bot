# DestinyDefinitionsDestinyItemQualityBlockDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_levels** | **Vec<i32>** | The \&quot;base\&quot; defined level of an item. This is a list because, in theory, each Expansion could define its own base level for an item.  In practice, not only was that never done in Destiny 1, but now this isn&#39;t even populated at all. When it&#39;s not populated, the level at which it spawns has to be inferred by Reward information, of which BNet receives an imperfect view and will only be reliable on instanced data as a result. | [optional] [default to null]
**quality_level** | **i32** | qualityLevel is used in combination with the item&#39;s level to calculate stats like Attack and Defense. It plays a role in that calculation, but not nearly as large as itemLevel does. | [optional] [default to null]
**infusion_category_name** | **String** | The string identifier for this item&#39;s \&quot;infusability\&quot;, if any.   Items that match the same infusionCategoryName are allowed to infuse with each other.  DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead. | [optional] [default to null]
**infusion_category_hash** | **i32** | The hash identifier for the infusion. It does not map to a Definition entity.  DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead. | [optional] [default to null]
**infusion_category_hashes** | **Vec<i32>** | If any one of these hashes matches any value in another item&#39;s infusionCategoryHashes, the two can infuse with each other. | [optional] [default to null]
**progression_level_requirement_hash** | **i32** | An item can refer to pre-set level requirements. They are defined in DestinyProgressionLevelRequirementDefinition, and you can use this hash to find the appropriate definition. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


