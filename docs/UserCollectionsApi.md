# \UserCollectionsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_collection_albums**](UserCollectionsApi.md#add_user_collection_albums) | **POST** /userCollections/{id}/relationships/albums | Add to albums relationship (\"to-many\").
[**add_user_collection_artists**](UserCollectionsApi.md#add_user_collection_artists) | **POST** /userCollections/{id}/relationships/artists | Add to artists relationship (\"to-many\").
[**add_user_collection_playlists**](UserCollectionsApi.md#add_user_collection_playlists) | **POST** /userCollections/{id}/relationships/playlists | Add to playlists relationship (\"to-many\").
[**add_user_collection_tracks**](UserCollectionsApi.md#add_user_collection_tracks) | **POST** /userCollections/{id}/relationships/tracks | Add to tracks relationship (\"to-many\").
[**add_user_collection_videos**](UserCollectionsApi.md#add_user_collection_videos) | **POST** /userCollections/{id}/relationships/videos | Add to videos relationship (\"to-many\").
[**delete_user_collection_albums**](UserCollectionsApi.md#delete_user_collection_albums) | **DELETE** /userCollections/{id}/relationships/albums | Delete from albums relationship (\"to-many\").
[**delete_user_collection_artists**](UserCollectionsApi.md#delete_user_collection_artists) | **DELETE** /userCollections/{id}/relationships/artists | Delete from artists relationship (\"to-many\").
[**delete_user_collection_tracks**](UserCollectionsApi.md#delete_user_collection_tracks) | **DELETE** /userCollections/{id}/relationships/tracks | Delete from tracks relationship (\"to-many\").
[**delete_user_collection_videos**](UserCollectionsApi.md#delete_user_collection_videos) | **DELETE** /userCollections/{id}/relationships/videos | Delete from videos relationship (\"to-many\").
[**deleteuser_collection_playlists**](UserCollectionsApi.md#deleteuser_collection_playlists) | **DELETE** /userCollections/{id}/relationships/playlists | Delete from playlists relationship (\"to-many\").
[**get_user_collection**](UserCollectionsApi.md#get_user_collection) | **GET** /userCollections/{id} | Get single userCollection.
[**get_user_collection_albums**](UserCollectionsApi.md#get_user_collection_albums) | **GET** /userCollections/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**get_user_collection_artists**](UserCollectionsApi.md#get_user_collection_artists) | **GET** /userCollections/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**get_user_collection_owners**](UserCollectionsApi.md#get_user_collection_owners) | **GET** /userCollections/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_user_collection_playlists**](UserCollectionsApi.md#get_user_collection_playlists) | **GET** /userCollections/{id}/relationships/playlists | Get playlists relationship (\"to-many\").
[**get_user_collection_tracks**](UserCollectionsApi.md#get_user_collection_tracks) | **GET** /userCollections/{id}/relationships/tracks | Get tracks relationship (\"to-many\").
[**get_user_collection_videos**](UserCollectionsApi.md#get_user_collection_videos) | **GET** /userCollections/{id}/relationships/videos | Get videos relationship (\"to-many\").



## add_user_collection_albums

> add_user_collection_albums(id, country_code, user_collection_albums_relationship_add_operation_payload)
Add to albums relationship (\"to-many\").

Adds item(s) to albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**user_collection_albums_relationship_add_operation_payload** | Option<[**UserCollectionAlbumsRelationshipAddOperationPayload**](UserCollectionAlbumsRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_collection_artists

> add_user_collection_artists(id, country_code, user_collection_artists_relationship_add_operation_payload)
Add to artists relationship (\"to-many\").

Adds item(s) to artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**user_collection_artists_relationship_add_operation_payload** | Option<[**UserCollectionArtistsRelationshipAddOperationPayload**](UserCollectionArtistsRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_collection_playlists

> add_user_collection_playlists(id, user_collection_playlists_relationship_remove_operation_payload)
Add to playlists relationship (\"to-many\").

Adds item(s) to playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**user_collection_playlists_relationship_remove_operation_payload** | Option<[**UserCollectionPlaylistsRelationshipRemoveOperationPayload**](UserCollectionPlaylistsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_collection_tracks

> add_user_collection_tracks(id, country_code, user_collection_tracks_relationship_add_operation_payload)
Add to tracks relationship (\"to-many\").

Adds item(s) to tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**user_collection_tracks_relationship_add_operation_payload** | Option<[**UserCollectionTracksRelationshipAddOperationPayload**](UserCollectionTracksRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_collection_videos

> add_user_collection_videos(id, country_code, user_collection_videos_relationship_add_operation_payload)
Add to videos relationship (\"to-many\").

Adds item(s) to videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**user_collection_videos_relationship_add_operation_payload** | Option<[**UserCollectionVideosRelationshipAddOperationPayload**](UserCollectionVideosRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_collection_albums

> delete_user_collection_albums(id, user_collection_albums_relationship_remove_operation_payload)
Delete from albums relationship (\"to-many\").

Deletes item(s) from albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**user_collection_albums_relationship_remove_operation_payload** | Option<[**UserCollectionAlbumsRelationshipRemoveOperationPayload**](UserCollectionAlbumsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_collection_artists

> delete_user_collection_artists(id, user_collection_artists_relationship_remove_operation_payload)
Delete from artists relationship (\"to-many\").

Deletes item(s) from artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**user_collection_artists_relationship_remove_operation_payload** | Option<[**UserCollectionArtistsRelationshipRemoveOperationPayload**](UserCollectionArtistsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_collection_tracks

> delete_user_collection_tracks(id, user_collection_tracks_relationship_remove_operation_payload)
Delete from tracks relationship (\"to-many\").

Deletes item(s) from tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**user_collection_tracks_relationship_remove_operation_payload** | Option<[**UserCollectionTracksRelationshipRemoveOperationPayload**](UserCollectionTracksRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_collection_videos

> delete_user_collection_videos(id, user_collection_videos_relationship_remove_operation_payload)
Delete from videos relationship (\"to-many\").

Deletes item(s) from videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**user_collection_videos_relationship_remove_operation_payload** | Option<[**UserCollectionVideosRelationshipRemoveOperationPayload**](UserCollectionVideosRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deleteuser_collection_playlists

> deleteuser_collection_playlists(id, user_collection_playlists_relationship_remove_operation_payload)
Delete from playlists relationship (\"to-many\").

Deletes item(s) from playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**user_collection_playlists_relationship_remove_operation_payload** | Option<[**UserCollectionPlaylistsRelationshipRemoveOperationPayload**](UserCollectionPlaylistsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection

> models::UserCollectionsSingleResourceDataDocument get_user_collection(id, country_code, locale, include)
Get single userCollection.

Retrieves single userCollection by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, owners, playlists, tracks, videos |  |

### Return type

[**models::UserCollectionsSingleResourceDataDocument**](UserCollections_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_albums

> models::UserCollectionsAlbumsMultiRelationshipDataDocument get_user_collection_albums(id, page_left_square_bracket_cursor_right_square_bracket, sort, country_code, locale, include)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |

### Return type

[**models::UserCollectionsAlbumsMultiRelationshipDataDocument**](UserCollections_Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_artists

> models::UserCollectionsArtistsMultiRelationshipDataDocument get_user_collection_artists(id, page_left_square_bracket_cursor_right_square_bracket, sort, country_code, locale, include)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |

### Return type

[**models::UserCollectionsArtistsMultiRelationshipDataDocument**](UserCollections_Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_owners

> models::UserCollectionsMultiRelationshipDataDocument get_user_collection_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserCollectionsMultiRelationshipDataDocument**](UserCollections_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_playlists

> models::UserCollectionsPlaylistsMultiRelationshipDataDocument get_user_collection_playlists(id, collection_view, page_left_square_bracket_cursor_right_square_bracket, sort, include)
Get playlists relationship (\"to-many\").

Retrieves playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**collection_view** | Option<**String**> |  |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: playlists |  |

### Return type

[**models::UserCollectionsPlaylistsMultiRelationshipDataDocument**](UserCollections_Playlists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_tracks

> models::UserCollectionsTracksMultiRelationshipDataDocument get_user_collection_tracks(id, page_left_square_bracket_cursor_right_square_bracket, sort, country_code, locale, include)
Get tracks relationship (\"to-many\").

Retrieves tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: tracks |  |

### Return type

[**models::UserCollectionsTracksMultiRelationshipDataDocument**](UserCollections_Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_videos

> models::UserCollectionsVideosMultiRelationshipDataDocument get_user_collection_videos(id, page_left_square_bracket_cursor_right_square_bracket, sort, country_code, locale, include)
Get videos relationship (\"to-many\").

Retrieves videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User collection id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: videos |  |

### Return type

[**models::UserCollectionsVideosMultiRelationshipDataDocument**](UserCollections_Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

