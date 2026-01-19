# \TracksApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_track**](TracksApi.md#create_track) | **POST** /tracks | Create single track.
[**delete_track**](TracksApi.md#delete_track) | **DELETE** /tracks/{id} | Delete single track.
[**get_track**](TracksApi.md#get_track) | **GET** /tracks/{id} | Get single track.
[**get_track_albums**](TracksApi.md#get_track_albums) | **GET** /tracks/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**get_track_genres**](TracksApi.md#get_track_genres) | **GET** /tracks/{id}/relationships/genres | Get genres relationship (\"to-many\").
[**get_track_lyrics**](TracksApi.md#get_track_lyrics) | **GET** /tracks/{id}/relationships/lyrics | Get lyrics relationship (\"to-many\").
[**get_track_owners**](TracksApi.md#get_track_owners) | **GET** /tracks/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_track_providers**](TracksApi.md#get_track_providers) | **GET** /tracks/{id}/relationships/providers | Get providers relationship (\"to-many\").
[**get_track_radio**](TracksApi.md#get_track_radio) | **GET** /tracks/{id}/relationships/radio | Get radio relationship (\"to-many\").
[**get_track_replacement_relationship**](TracksApi.md#get_track_replacement_relationship) | **GET** /tracks/{id}/relationships/replacement | Get replacement relationship (\"to-one\").
[**get_track_shares**](TracksApi.md#get_track_shares) | **GET** /tracks/{id}/relationships/shares | Get shares relationship (\"to-many\").
[**get_track_similar_tracks**](TracksApi.md#get_track_similar_tracks) | **GET** /tracks/{id}/relationships/similarTracks | Get similarTracks relationship (\"to-many\").
[**get_track_track_source_file**](TracksApi.md#get_track_track_source_file) | **GET** /tracks/{id}/relationships/sourceFile | Get sourceFile relationship (\"to-one\").
[**get_track_track_statistics**](TracksApi.md#get_track_track_statistics) | **GET** /tracks/{id}/relationships/trackStatistics | Get trackStatistics relationship (\"to-one\").
[**get_tracks**](TracksApi.md#get_tracks) | **GET** /tracks | Get multiple tracks.
[**patch_track**](TracksApi.md#patch_track) | **PATCH** /tracks/{id} | Update single track.
[**patch_track_albums**](TracksApi.md#patch_track_albums) | **PATCH** /tracks/{id}/relationships/albums | Update albums relationship (\"to-many\").
[**patch_track_artists**](TracksApi.md#patch_track_artists) | **GET** /tracks/{id}/relationships/artists | Get artists relationship (\"to-many\").



## create_track

> models::TracksSingleResourceDataDocument create_track(track_create_operation_payload)
Create single track.

Creates a new track.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_create_operation_payload** | Option<[**TrackCreateOperationPayload**](TrackCreateOperationPayload.md)> |  |  |

### Return type

[**models::TracksSingleResourceDataDocument**](Tracks_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_track

> delete_track(id)
Delete single track.

Deletes existing track.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track

> models::TracksSingleResourceDataDocument get_track(id, country_code, include, share_code)
Get single track.

Retrieves single track by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, genres, lyrics, owners, providers, radio, replacement, shares, similarTracks, sourceFile, trackStatistics |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksSingleResourceDataDocument**](Tracks_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_albums

> models::TracksMultiRelationshipDataDocument get_track_albums(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket, share_code)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_genres

> models::TracksMultiRelationshipDataDocument get_track_genres(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket, share_code)
Get genres relationship (\"to-many\").

Retrieves genres relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: genres |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_lyrics

> models::TracksMultiRelationshipDataDocument get_track_lyrics(id, include, page_left_square_bracket_cursor_right_square_bracket, share_code)
Get lyrics relationship (\"to-many\").

Retrieves lyrics relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: lyrics |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_owners

> models::TracksMultiRelationshipDataDocument get_track_owners(id, include, page_left_square_bracket_cursor_right_square_bracket, share_code)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_providers

> models::TracksMultiRelationshipDataDocument get_track_providers(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket, share_code)
Get providers relationship (\"to-many\").

Retrieves providers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: providers |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_radio

> models::TracksMultiRelationshipDataDocument get_track_radio(id, include, page_left_square_bracket_cursor_right_square_bracket, share_code)
Get radio relationship (\"to-many\").

Retrieves radio relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: radio |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_replacement_relationship

> models::TracksSingleRelationshipDataDocument get_track_replacement_relationship(id, country_code, include, share_code)
Get replacement relationship (\"to-one\").

Retrieves replacement relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: replacement |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksSingleRelationshipDataDocument**](Tracks_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_shares

> models::TracksMultiRelationshipDataDocument get_track_shares(id, include, page_left_square_bracket_cursor_right_square_bracket, share_code)
Get shares relationship (\"to-many\").

Retrieves shares relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: shares |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_similar_tracks

> models::TracksMultiRelationshipDataDocument get_track_similar_tracks(id, page_left_square_bracket_cursor_right_square_bracket, country_code, include, share_code)
Get similarTracks relationship (\"to-many\").

Retrieves similarTracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: similarTracks |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_track_source_file

> models::TracksSingleRelationshipDataDocument get_track_track_source_file(id, include, share_code)
Get sourceFile relationship (\"to-one\").

Retrieves sourceFile relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: sourceFile |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksSingleRelationshipDataDocument**](Tracks_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_track_statistics

> models::TracksSingleRelationshipDataDocument get_track_track_statistics(id, include, share_code)
Get trackStatistics relationship (\"to-one\").

Retrieves trackStatistics relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: trackStatistics |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksSingleRelationshipDataDocument**](Tracks_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tracks

> models::TracksMultiResourceDataDocument get_tracks(page_left_square_bracket_cursor_right_square_bracket, country_code, include, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_isrc_right_square_bracket, filter_left_square_bracket_owners_id_right_square_bracket, share_code)
Get multiple tracks.

Retrieves multiple tracks by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, genres, lyrics, owners, providers, radio, replacement, shares, similarTracks, sourceFile, trackStatistics |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Track id |  |
**filter_left_square_bracket_isrc_right_square_bracket** | Option<[**Vec<String>**](String.md)> | International Standard Recording Code (ISRC) |  |
**filter_left_square_bracket_owners_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiResourceDataDocument**](Tracks_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_track

> patch_track(id, track_update_operation_payload)
Update single track.

Updates existing track.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**track_update_operation_payload** | Option<[**TrackUpdateOperationPayload**](TrackUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_track_albums

> patch_track_albums(id, track_albums_relationship_update_operation_payload)
Update albums relationship (\"to-many\").

Updates albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**track_albums_relationship_update_operation_payload** | Option<[**TrackAlbumsRelationshipUpdateOperationPayload**](TrackAlbumsRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_track_artists

> models::TracksMultiRelationshipDataDocument patch_track_artists(id, page_left_square_bracket_cursor_right_square_bracket, country_code, include, share_code)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

