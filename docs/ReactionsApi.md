# \ReactionsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_reaction**](ReactionsApi.md#create_reaction) | **POST** /reactions | Create single reaction.
[**delete_reaction**](ReactionsApi.md#delete_reaction) | **DELETE** /reactions/{id} | Delete single reaction.
[**get_reaction_owner_profile**](ReactionsApi.md#get_reaction_owner_profile) | **GET** /reactions/{id}/relationships/ownerProfiles | Get ownerProfiles relationship (\"to-many\").
[**get_reactions**](ReactionsApi.md#get_reactions) | **GET** /reactions | Get multiple reactions.



## create_reaction

> models::ReactionsSingleResourceDataDocument create_reaction(create_reaction_payload)
Create single reaction.

Creates a new reaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_reaction_payload** | Option<[**CreateReactionPayload**](CreateReactionPayload.md)> |  |  |

### Return type

[**models::ReactionsSingleResourceDataDocument**](Reactions_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_reaction

> delete_reaction(id)
Delete single reaction.

Deletes existing reaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Reaction Id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reaction_owner_profile

> models::ReactionsMultiRelationshipDataDocument get_reaction_owner_profile(id, include, page_cursor)
Get ownerProfiles relationship (\"to-many\").

Retrieves ownerProfiles relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Reaction Id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: ownerProfiles |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ReactionsMultiRelationshipDataDocument**](Reactions_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reactions

> models::ReactionsMultiResourceDataDocument get_reactions(stats, stats_only, page_cursor, include, filter_left_square_bracket_owner_id_right_square_bracket, filter_left_square_bracket_reacted_resource_id_right_square_bracket, filter_left_square_bracket_reacted_resource_type_right_square_bracket, filter_left_square_bracket_reaction_type_right_square_bracket)
Get multiple reactions.

Retrieves multiple reactions by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stats** | Option<**String**> |  |  |
**stats_only** | Option<**bool**> |  |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: ownerProfiles |  |
**filter_left_square_bracket_owner_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Filter by owner id |  |
**filter_left_square_bracket_reacted_resource_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Filter by resource ID |  |
**filter_left_square_bracket_reacted_resource_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Filter by resource type |  |
**filter_left_square_bracket_reaction_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Filter by reaction type |  |

### Return type

[**models::ReactionsMultiResourceDataDocument**](Reactions_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
