# \ArtistBiographiesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_artist_biographies**](ArtistBiographiesApi.md#get_artist_biographies) | **GET** /artistBiographies | Get multiple artistBiographies.
[**get_artist_biography**](ArtistBiographiesApi.md#get_artist_biography) | **GET** /artistBiographies/{id} | Get single artistBiographie.
[**get_artist_biography_owners**](ArtistBiographiesApi.md#get_artist_biography_owners) | **GET** /artistBiographies/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**patch_artist_biography**](ArtistBiographiesApi.md#patch_artist_biography) | **PATCH** /artistBiographies/{id} | Update single artistBiographie.



## get_artist_biographies

> models::ArtistBiographiesMultiResourceDataDocument get_artist_biographies(country_code, include, filter_left_square_bracket_id_right_square_bracket)
Get multiple artistBiographies.

Retrieves multiple artistBiographies by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artist id |  |

### Return type

[**models::ArtistBiographiesMultiResourceDataDocument**](ArtistBiographies_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_biography

> models::ArtistBiographiesSingleResourceDataDocument get_artist_biography(id, country_code, include)
Get single artistBiographie.

Retrieves single artistBiographie by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist biography id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::ArtistBiographiesSingleResourceDataDocument**](ArtistBiographies_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_biography_owners

> models::ArtistBiographiesMultiRelationshipDataDocument get_artist_biography_owners(id, include, page_cursor)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist biography id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistBiographiesMultiRelationshipDataDocument**](ArtistBiographies_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_artist_biography

> patch_artist_biography(id, artist_biography_update_body)
Update single artistBiographie.

Updates existing artistBiographie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist biography id | [required] |
**artist_biography_update_body** | Option<[**ArtistBiographyUpdateBody**](ArtistBiographyUpdateBody.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
