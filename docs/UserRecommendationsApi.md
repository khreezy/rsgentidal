# \UserRecommendationsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_recommendation**](UserRecommendationsApi.md#get_user_recommendation) | **GET** /userRecommendations/{id} | Get single userRecommendation.
[**get_user_recommendation_discovery_mixes**](UserRecommendationsApi.md#get_user_recommendation_discovery_mixes) | **GET** /userRecommendations/{id}/relationships/discoveryMixes | Get discoveryMixes relationship (\"to-many\").
[**get_user_recommendation_my_mixes**](UserRecommendationsApi.md#get_user_recommendation_my_mixes) | **GET** /userRecommendations/{id}/relationships/myMixes | Get myMixes relationship (\"to-many\").
[**get_user_recommendation_new_arrival_mixes**](UserRecommendationsApi.md#get_user_recommendation_new_arrival_mixes) | **GET** /userRecommendations/{id}/relationships/newArrivalMixes | Get newArrivalMixes relationship (\"to-many\").



## get_user_recommendation

> models::UserRecommendationsSingleResourceDataDocument get_user_recommendation(id, country_code, locale, include)
Get single userRecommendation.

Retrieves single userRecommendation by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User recommendations id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: discoveryMixes, myMixes, newArrivalMixes |  |

### Return type

[**models::UserRecommendationsSingleResourceDataDocument**](UserRecommendations_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_recommendation_discovery_mixes

> models::UserRecommendationsMultiRelationshipDataDocument get_user_recommendation_discovery_mixes(id, page_cursor, country_code, locale, include)
Get discoveryMixes relationship (\"to-many\").

Retrieves discoveryMixes relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User recommendations id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: discoveryMixes |  |

### Return type

[**models::UserRecommendationsMultiRelationshipDataDocument**](UserRecommendations_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_recommendation_my_mixes

> models::UserRecommendationsMultiRelationshipDataDocument get_user_recommendation_my_mixes(id, page_cursor, country_code, locale, include)
Get myMixes relationship (\"to-many\").

Retrieves myMixes relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User recommendations id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: myMixes |  |

### Return type

[**models::UserRecommendationsMultiRelationshipDataDocument**](UserRecommendations_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_recommendation_new_arrival_mixes

> models::UserRecommendationsMultiRelationshipDataDocument get_user_recommendation_new_arrival_mixes(id, page_cursor, country_code, locale, include)
Get newArrivalMixes relationship (\"to-many\").

Retrieves newArrivalMixes relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User recommendations id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: newArrivalMixes |  |

### Return type

[**models::UserRecommendationsMultiRelationshipDataDocument**](UserRecommendations_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
