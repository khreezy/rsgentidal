# \PlaylistsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_items_to_playlist**](PlaylistsApi.md#add_items_to_playlist) | **POST** /playlists/{id}/relationships/items | Add to items relationship (\"to-many\").
[**create_playlist**](PlaylistsApi.md#create_playlist) | **POST** /playlists | Create single playlist.
[**delete_playlist**](PlaylistsApi.md#delete_playlist) | **DELETE** /playlists/{id} | Delete single playlist.
[**delete_playlist_items**](PlaylistsApi.md#delete_playlist_items) | **DELETE** /playlists/{id}/relationships/items | Delete from items relationship (\"to-many\").
[**get_playlist**](PlaylistsApi.md#get_playlist) | **GET** /playlists/{id} | Get single playlist.
[**get_playlist_cover_art**](PlaylistsApi.md#get_playlist_cover_art) | **GET** /playlists/{id}/relationships/coverArt | Get coverArt relationship (\"to-many\").
[**get_playlist_items**](PlaylistsApi.md#get_playlist_items) | **GET** /playlists/{id}/relationships/items | Get items relationship (\"to-many\").
[**get_playlist_owner_profiles**](PlaylistsApi.md#get_playlist_owner_profiles) | **GET** /playlists/{id}/relationships/ownerProfiles | Get ownerProfiles relationship (\"to-many\").
[**get_playlist_owners**](PlaylistsApi.md#get_playlist_owners) | **GET** /playlists/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_playlists**](PlaylistsApi.md#get_playlists) | **GET** /playlists | Get multiple playlists.
[**patch_playlist**](PlaylistsApi.md#patch_playlist) | **PATCH** /playlists/{id} | Update single playlist.
[**patch_playlist_cover_art**](PlaylistsApi.md#patch_playlist_cover_art) | **PATCH** /playlists/{id}/relationships/coverArt | Update coverArt relationship (\"to-many\").
[**patch_playlist_items**](PlaylistsApi.md#patch_playlist_items) | **PATCH** /playlists/{id}/relationships/items | Update items relationship (\"to-many\").



## add_items_to_playlist

> add_items_to_playlist(id, country_code, playlist_items_relationship_add_operation_payload)
Add to items relationship (\"to-many\").

Adds item(s) to items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**playlist_items_relationship_add_operation_payload** | Option<[**PlaylistItemsRelationshipAddOperationPayload**](PlaylistItemsRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_playlist

> models::PlaylistsSingleResourceDataDocument create_playlist(country_code, playlist_create_operation_payload)
Create single playlist.

Creates a new playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**playlist_create_operation_payload** | Option<[**PlaylistCreateOperationPayload**](PlaylistCreateOperationPayload.md)> |  |  |

### Return type

[**models::PlaylistsSingleResourceDataDocument**](Playlists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_playlist

> delete_playlist(id)
Delete single playlist.

Deletes existing playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_playlist_items

> delete_playlist_items(id, playlist_items_relationship_remove_operation_payload)
Delete from items relationship (\"to-many\").

Deletes item(s) from items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**playlist_items_relationship_remove_operation_payload** | Option<[**PlaylistItemsRelationshipRemoveOperationPayload**](PlaylistItemsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist

> models::PlaylistsSingleResourceDataDocument get_playlist(id, country_code, include)
Get single playlist.

Retrieves single playlist by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt, items, ownerProfiles, owners |  |

### Return type

[**models::PlaylistsSingleResourceDataDocument**](Playlists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_cover_art

> models::PlaylistsMultiRelationshipDataDocument get_playlist_cover_art(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get coverArt relationship (\"to-many\").

Retrieves coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::PlaylistsMultiRelationshipDataDocument**](Playlists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_items

> models::PlaylistsItemsMultiRelationshipDataDocument get_playlist_items(id, page_left_square_bracket_cursor_right_square_bracket, country_code, include)
Get items relationship (\"to-many\").

Retrieves items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: items |  |

### Return type

[**models::PlaylistsItemsMultiRelationshipDataDocument**](Playlists_Items_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_owner_profiles

> models::PlaylistsMultiRelationshipDataDocument get_playlist_owner_profiles(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get ownerProfiles relationship (\"to-many\").

Retrieves ownerProfiles relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: ownerProfiles |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::PlaylistsMultiRelationshipDataDocument**](Playlists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_owners

> models::PlaylistsMultiRelationshipDataDocument get_playlist_owners(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::PlaylistsMultiRelationshipDataDocument**](Playlists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlists

> models::PlaylistsMultiResourceDataDocument get_playlists(page_left_square_bracket_cursor_right_square_bracket, sort, country_code, include, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_owners_id_right_square_bracket)
Get multiple playlists.

Retrieves multiple playlists by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt, items, ownerProfiles, owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Playlist id |  |
**filter_left_square_bracket_owners_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |

### Return type

[**models::PlaylistsMultiResourceDataDocument**](Playlists_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_playlist

> patch_playlist(id, country_code, playlist_update_operation_payload)
Update single playlist.

Updates existing playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**playlist_update_operation_payload** | Option<[**PlaylistUpdateOperationPayload**](PlaylistUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_playlist_cover_art

> patch_playlist_cover_art(id, playlist_cover_art_relationship_update_operation_payload)
Update coverArt relationship (\"to-many\").

Updates coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**playlist_cover_art_relationship_update_operation_payload** | Option<[**PlaylistCoverArtRelationshipUpdateOperationPayload**](PlaylistCoverArtRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_playlist_items

> patch_playlist_items(id, playlist_items_relationship_reorder_operation_payload)
Update items relationship (\"to-many\").

Updates items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**playlist_items_relationship_reorder_operation_payload** | Option<[**PlaylistItemsRelationshipReorderOperationPayload**](PlaylistItemsRelationshipReorderOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

