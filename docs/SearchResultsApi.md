# \SearchResultsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search_result**](SearchResultsApi.md#get_search_result) | **GET** /searchResults/{id} | Get single searchResult.
[**get_search_result_albums**](SearchResultsApi.md#get_search_result_albums) | **GET** /searchResults/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**get_search_result_artists**](SearchResultsApi.md#get_search_result_artists) | **GET** /searchResults/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**get_search_result_playlists**](SearchResultsApi.md#get_search_result_playlists) | **GET** /searchResults/{id}/relationships/playlists | Get playlists relationship (\"to-many\").
[**get_search_result_tracks**](SearchResultsApi.md#get_search_result_tracks) | **GET** /searchResults/{id}/relationships/tracks | Get tracks relationship (\"to-many\").
[**get_search_result_videos**](SearchResultsApi.md#get_search_result_videos) | **GET** /searchResults/{id}/relationships/videos | Get videos relationship (\"to-many\").
[**get_search_results_top_hits**](SearchResultsApi.md#get_search_results_top_hits) | **GET** /searchResults/{id}/relationships/topHits | Get topHits relationship (\"to-many\").



## get_search_result

> models::SearchResultsSingleResourceDataDocument get_search_result(id, explicit_filter, country_code, include)
Get single searchResult.

Retrieves single searchResult by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, playlists, topHits, tracks, videos |  |

### Return type

[**models::SearchResultsSingleResourceDataDocument**](SearchResults_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_result_albums

> models::SearchResultsMultiRelationshipDataDocument get_search_result_albums(id, explicit_filter, page_cursor, country_code, include)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_result_artists

> models::SearchResultsMultiRelationshipDataDocument get_search_result_artists(id, explicit_filter, page_cursor, country_code, include)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_result_playlists

> models::SearchResultsMultiRelationshipDataDocument get_search_result_playlists(id, explicit_filter, page_cursor, country_code, include)
Get playlists relationship (\"to-many\").

Retrieves playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: playlists |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_result_tracks

> models::SearchResultsMultiRelationshipDataDocument get_search_result_tracks(id, explicit_filter, page_cursor, country_code, include)
Get tracks relationship (\"to-many\").

Retrieves tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: tracks |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_result_videos

> models::SearchResultsMultiRelationshipDataDocument get_search_result_videos(id, explicit_filter, page_cursor, country_code, include)
Get videos relationship (\"to-many\").

Retrieves videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: videos |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_results_top_hits

> models::SearchResultsMultiRelationshipDataDocument get_search_results_top_hits(id, explicit_filter, page_cursor, country_code, include)
Get topHits relationship (\"to-many\").

Retrieves topHits relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: topHits |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
