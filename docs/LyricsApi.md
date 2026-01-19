# \LyricsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_lyric**](LyricsApi.md#create_lyric) | **POST** /lyrics | Create single lyric.
[**delete_lyric**](LyricsApi.md#delete_lyric) | **DELETE** /lyrics/{id} | Delete single lyric.
[**get_lyric**](LyricsApi.md#get_lyric) | **GET** /lyrics/{id} | Get single lyric.
[**get_lyric_owners**](LyricsApi.md#get_lyric_owners) | **GET** /lyrics/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_lyric_track**](LyricsApi.md#get_lyric_track) | **GET** /lyrics/{id}/relationships/track | Get track relationship (\"to-one\").
[**get_lyrics**](LyricsApi.md#get_lyrics) | **GET** /lyrics | Get multiple lyrics.
[**patch_lyric**](LyricsApi.md#patch_lyric) | **PATCH** /lyrics/{id} | Update single lyric.



## create_lyric

> models::LyricsSingleResourceDataDocument create_lyric(lyrics_create_operation_payload)
Create single lyric.

Creates a new lyric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lyrics_create_operation_payload** | Option<[**LyricsCreateOperationPayload**](LyricsCreateOperationPayload.md)> |  |  |

### Return type

[**models::LyricsSingleResourceDataDocument**](Lyrics_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lyric

> delete_lyric(id)
Delete single lyric.

Deletes existing lyric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lyric

> models::LyricsSingleResourceDataDocument get_lyric(id, include)
Get single lyric.

Retrieves single lyric by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, track |  |

### Return type

[**models::LyricsSingleResourceDataDocument**](Lyrics_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lyric_owners

> models::LyricsMultiRelationshipDataDocument get_lyric_owners(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::LyricsMultiRelationshipDataDocument**](Lyrics_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lyric_track

> models::LyricsSingleRelationshipDataDocument get_lyric_track(id, country_code, include)
Get track relationship (\"to-one\").

Retrieves track relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: track |  |

### Return type

[**models::LyricsSingleRelationshipDataDocument**](Lyrics_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lyrics

> models::LyricsMultiResourceDataDocument get_lyrics(include, filter_left_square_bracket_id_right_square_bracket)
Get multiple lyrics.

Retrieves multiple lyrics by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, track |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Lyrics Id |  |

### Return type

[**models::LyricsMultiResourceDataDocument**](Lyrics_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lyric

> patch_lyric(id, lyrics_update_operation_payload)
Update single lyric.

Updates existing lyric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**lyrics_update_operation_payload** | Option<[**LyricsUpdateOperationPayload**](LyricsUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

