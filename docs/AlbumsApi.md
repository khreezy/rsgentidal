# \AlbumsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_album**](AlbumsApi.md#create_album) | **POST** /albums | Create single album.
[**delete_album**](AlbumsApi.md#delete_album) | **DELETE** /albums/{id} | Delete single album.
[**get_album**](AlbumsApi.md#get_album) | **GET** /albums/{id} | Get single album.
[**get_album_artists**](AlbumsApi.md#get_album_artists) | **GET** /albums/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**get_album_cover_art**](AlbumsApi.md#get_album_cover_art) | **GET** /albums/{id}/relationships/coverArt | Get coverArt relationship (\"to-many\").
[**get_album_genres**](AlbumsApi.md#get_album_genres) | **GET** /albums/{id}/relationships/genres | Get genres relationship (\"to-many\").
[**get_album_items**](AlbumsApi.md#get_album_items) | **GET** /albums/{id}/relationships/items | Get items relationship (\"to-many\").
[**get_album_owners**](AlbumsApi.md#get_album_owners) | **GET** /albums/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_album_providers**](AlbumsApi.md#get_album_providers) | **GET** /albums/{id}/relationships/providers | Get providers relationship (\"to-many\").
[**get_album_replacement_replationship**](AlbumsApi.md#get_album_replacement_replationship) | **GET** /albums/{id}/relationships/replacement | Get replacement relationship (\"to-one\").
[**get_album_suggested_cover_arts**](AlbumsApi.md#get_album_suggested_cover_arts) | **GET** /albums/{id}/relationships/suggestedCoverArts | Get suggestedCoverArts relationship (\"to-many\").
[**get_albums**](AlbumsApi.md#get_albums) | **GET** /albums | Get multiple albums.
[**get_similar_albums**](AlbumsApi.md#get_similar_albums) | **GET** /albums/{id}/relationships/similarAlbums | Get similarAlbums relationship (\"to-many\").
[**patch_a_lbum_items**](AlbumsApi.md#patch_a_lbum_items) | **PATCH** /albums/{id}/relationships/items | Update items relationship (\"to-many\").
[**patch_album**](AlbumsApi.md#patch_album) | **PATCH** /albums/{id} | Update single album.
[**patch_album_cover_art**](AlbumsApi.md#patch_album_cover_art) | **PATCH** /albums/{id}/relationships/coverArt | Update coverArt relationship (\"to-many\").



## create_album

> models::AlbumsSingleResourceDataDocument create_album(album_create_operation_payload)
Create single album.

Creates a new album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_create_operation_payload** | Option<[**AlbumCreateOperationPayload**](AlbumCreateOperationPayload.md)> |  |  |

### Return type

[**models::AlbumsSingleResourceDataDocument**](Albums_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_album

> delete_album(id)
Delete single album.

Deletes existing album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album

> models::AlbumsSingleResourceDataDocument get_album(id, country_code, include, share_code)
Get single album.

Retrieves single album by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists, coverArt, genres, items, owners, providers, replacement, similarAlbums, suggestedCoverArts |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsSingleResourceDataDocument**](Albums_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_artists

> models::AlbumsMultiRelationshipDataDocument get_album_artists(id, page_cursor, country_code, include, share_code)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_cover_art

> models::AlbumsMultiRelationshipDataDocument get_album_cover_art(id, page_cursor, country_code, include, share_code)
Get coverArt relationship (\"to-many\").

Retrieves coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_genres

> models::AlbumsMultiRelationshipDataDocument get_album_genres(id, page_cursor, country_code, include, share_code)
Get genres relationship (\"to-many\").

Retrieves genres relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: genres |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_items

> models::AlbumsItemsMultiRelationshipDataDocument get_album_items(id, page_cursor, country_code, include, share_code)
Get items relationship (\"to-many\").

Retrieves items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: items |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsItemsMultiRelationshipDataDocument**](Albums_Items_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_owners

> models::AlbumsMultiRelationshipDataDocument get_album_owners(id, include, page_cursor, share_code)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_providers

> models::AlbumsMultiRelationshipDataDocument get_album_providers(id, country_code, include, page_cursor, share_code)
Get providers relationship (\"to-many\").

Retrieves providers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: providers |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_replacement_replationship

> models::AlbumsSingleRelationshipDataDocument get_album_replacement_replationship(id, country_code, include, share_code)
Get replacement relationship (\"to-one\").

Retrieves replacement relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: replacement |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsSingleRelationshipDataDocument**](Albums_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_suggested_cover_arts

> models::AlbumsSuggestedCoverArtsMultiRelationshipDataDocument get_album_suggested_cover_arts(id, include, page_cursor, share_code)
Get suggestedCoverArts relationship (\"to-many\").

Retrieves suggestedCoverArts relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: suggestedCoverArts |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsSuggestedCoverArtsMultiRelationshipDataDocument**](Albums_SuggestedCoverArts_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_albums

> models::AlbumsMultiResourceDataDocument get_albums(page_cursor, country_code, include, filter_left_square_bracket_barcode_id_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_owners_id_right_square_bracket, share_code)
Get multiple albums.

Retrieves multiple albums by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists, coverArt, genres, items, owners, providers, replacement, similarAlbums, suggestedCoverArts |  |
**filter_left_square_bracket_barcode_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Barcode Id |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Album id |  |
**filter_left_square_bracket_owners_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsMultiResourceDataDocument**](Albums_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_albums

> models::AlbumsMultiRelationshipDataDocument get_similar_albums(id, page_cursor, country_code, include, share_code)
Get similarAlbums relationship (\"to-many\").

Retrieves similarAlbums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: similarAlbums |  |
**share_code** | Option<**String**> | Share code that grants access to UNLISTED resources. When provided, allows non-owners to access resources that would otherwise be restricted. |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_a_lbum_items

> patch_a_lbum_items(id, album_items_relationship_update_operation_payload)
Update items relationship (\"to-many\").

Updates items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**album_items_relationship_update_operation_payload** | Option<[**AlbumItemsRelationshipUpdateOperationPayload**](AlbumItemsRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_album

> patch_album(id, album_update_operation_payload)
Update single album.

Updates existing album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**album_update_operation_payload** | Option<[**AlbumUpdateOperationPayload**](AlbumUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_album_cover_art

> patch_album_cover_art(id, album_cover_art_relationship_update_operation_payload)
Update coverArt relationship (\"to-many\").

Updates coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**album_cover_art_relationship_update_operation_payload** | Option<[**AlbumCoverArtRelationshipUpdateOperationPayload**](AlbumCoverArtRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
