# \ArtistsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_artist_followers**](ArtistsApi.md#add_artist_followers) | **POST** /artists/{id}/relationships/following | Add to following relationship (\"to-many\").
[**create_artist**](ArtistsApi.md#create_artist) | **POST** /artists | Create single artist.
[**delete_artist_followers**](ArtistsApi.md#delete_artist_followers) | **DELETE** /artists/{id}/relationships/following | Delete from following relationship (\"to-many\").
[**get_artist**](ArtistsApi.md#get_artist) | **GET** /artists/{id} | Get single artist.
[**get_artist_albums**](ArtistsApi.md#get_artist_albums) | **GET** /artists/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**get_artist_artist_biography**](ArtistsApi.md#get_artist_artist_biography) | **GET** /artists/{id}/relationships/biography | Get biography relationship (\"to-one\").
[**get_artist_artist_roles**](ArtistsApi.md#get_artist_artist_roles) | **GET** /artists/{id}/relationships/roles | Get roles relationship (\"to-many\").
[**get_artist_follower**](ArtistsApi.md#get_artist_follower) | **GET** /artists/{id}/relationships/following | Get following relationship (\"to-many\").
[**get_artist_followers**](ArtistsApi.md#get_artist_followers) | **GET** /artists/{id}/relationships/followers | Get followers relationship (\"to-many\").
[**get_artist_owners**](ArtistsApi.md#get_artist_owners) | **GET** /artists/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_artist_profile_art**](ArtistsApi.md#get_artist_profile_art) | **GET** /artists/{id}/relationships/profileArt | Get profileArt relationship (\"to-many\").
[**get_artist_radio**](ArtistsApi.md#get_artist_radio) | **GET** /artists/{id}/relationships/radio | Get radio relationship (\"to-many\").
[**get_artist_similar_artists**](ArtistsApi.md#get_artist_similar_artists) | **GET** /artists/{id}/relationships/similarArtists | Get similarArtists relationship (\"to-many\").
[**get_artist_track_providers**](ArtistsApi.md#get_artist_track_providers) | **GET** /artists/{id}/relationships/trackProviders | Get trackProviders relationship (\"to-many\").
[**get_artist_tracks**](ArtistsApi.md#get_artist_tracks) | **GET** /artists/{id}/relationships/tracks | Get tracks relationship (\"to-many\").
[**get_artist_videos**](ArtistsApi.md#get_artist_videos) | **GET** /artists/{id}/relationships/videos | Get videos relationship (\"to-many\").
[**get_artists**](ArtistsApi.md#get_artists) | **GET** /artists | Get multiple artists.
[**patch_artist**](ArtistsApi.md#patch_artist) | **PATCH** /artists/{id} | Update single artist.
[**patch_artist_profile_art**](ArtistsApi.md#patch_artist_profile_art) | **PATCH** /artists/{id}/relationships/profileArt | Update profileArt relationship (\"to-many\").



## add_artist_followers

> add_artist_followers(id, country_code, artist_following_relationship_add_operation_payload)
Add to following relationship (\"to-many\").

Adds item(s) to following relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**artist_following_relationship_add_operation_payload** | Option<[**ArtistFollowingRelationshipAddOperationPayload**](ArtistFollowingRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_artist

> models::ArtistsSingleResourceDataDocument create_artist(artist_create_operation_payload)
Create single artist.

Creates a new artist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_create_operation_payload** | Option<[**ArtistCreateOperationPayload**](ArtistCreateOperationPayload.md)> |  |  |

### Return type

[**models::ArtistsSingleResourceDataDocument**](Artists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_artist_followers

> delete_artist_followers(id, artist_following_relationship_remove_operation_payload)
Delete from following relationship (\"to-many\").

Deletes item(s) from following relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**artist_following_relationship_remove_operation_payload** | Option<[**ArtistFollowingRelationshipRemoveOperationPayload**](ArtistFollowingRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist

> models::ArtistsSingleResourceDataDocument get_artist(id, country_code, include)
Get single artist.

Retrieves single artist by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, biography, followers, following, owners, profileArt, radio, roles, similarArtists, trackProviders, tracks, videos |  |

### Return type

[**models::ArtistsSingleResourceDataDocument**](Artists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_albums

> models::ArtistsMultiRelationshipDataDocument get_artist_albums(id, page_cursor, country_code, include)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_artist_biography

> models::ArtistsSingleRelationshipDataDocument get_artist_artist_biography(id, country_code, include)
Get biography relationship (\"to-one\").

Retrieves biography relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: biography |  |

### Return type

[**models::ArtistsSingleRelationshipDataDocument**](Artists_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_artist_roles

> models::ArtistsMultiRelationshipDataDocument get_artist_artist_roles(id, include, page_cursor)
Get roles relationship (\"to-many\").

Retrieves roles relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: roles |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_follower

> models::ArtistsFollowingMultiRelationshipDataDocument get_artist_follower(id, viewer_context, page_cursor, include)
Get following relationship (\"to-many\").

Retrieves following relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**viewer_context** | Option<**String**> |  |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: following |  |

### Return type

[**models::ArtistsFollowingMultiRelationshipDataDocument**](Artists_Following_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_followers

> models::ArtistsFollowersMultiRelationshipDataDocument get_artist_followers(id, viewer_context, page_cursor, include)
Get followers relationship (\"to-many\").

Retrieves followers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**viewer_context** | Option<**String**> |  |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: followers |  |

### Return type

[**models::ArtistsFollowersMultiRelationshipDataDocument**](Artists_Followers_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_owners

> models::ArtistsMultiRelationshipDataDocument get_artist_owners(id, include, page_cursor)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_profile_art

> models::ArtistsMultiRelationshipDataDocument get_artist_profile_art(id, country_code, include, page_cursor)
Get profileArt relationship (\"to-many\").

Retrieves profileArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: profileArt |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_radio

> models::ArtistsMultiRelationshipDataDocument get_artist_radio(id, page_cursor, country_code, include)
Get radio relationship (\"to-many\").

Retrieves radio relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: radio |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_similar_artists

> models::ArtistsMultiRelationshipDataDocument get_artist_similar_artists(id, page_cursor, country_code, include)
Get similarArtists relationship (\"to-many\").

Retrieves similarArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: similarArtists |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_track_providers

> models::ArtistsTrackProvidersMultiRelationshipDataDocument get_artist_track_providers(id, page_cursor, include)
Get trackProviders relationship (\"to-many\").

Retrieves trackProviders relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: trackProviders |  |

### Return type

[**models::ArtistsTrackProvidersMultiRelationshipDataDocument**](Artists_TrackProviders_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_tracks

> models::ArtistsMultiRelationshipDataDocument get_artist_tracks(id, collapse_by, page_cursor, country_code, include)
Get tracks relationship (\"to-many\").

Retrieves tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**collapse_by** | **String** | Collapse by options for getting artist tracks. Available options: FINGERPRINT, ID. FINGERPRINT option might collapse similar tracks based entry fingerprints while collapsing by ID always returns all available items. | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: tracks |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_videos

> models::ArtistsMultiRelationshipDataDocument get_artist_videos(id, page_cursor, country_code, include)
Get videos relationship (\"to-many\").

Retrieves videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: videos |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artists

> models::ArtistsMultiResourceDataDocument get_artists(country_code, include, filter_left_square_bracket_handle_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple artists.

Retrieves multiple artists by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, biography, followers, following, owners, profileArt, radio, roles, similarArtists, trackProviders, tracks, videos |  |
**filter_left_square_bracket_handle_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artist handle |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artist id |  |

### Return type

[**models::ArtistsMultiResourceDataDocument**](Artists_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_artist

> patch_artist(id, artist_update_body)
Update single artist.

Updates existing artist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**artist_update_body** | Option<[**ArtistUpdateBody**](ArtistUpdateBody.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_artist_profile_art

> patch_artist_profile_art(id, artist_profile_art_relationship_update_operation_payload)
Update profileArt relationship (\"to-many\").

Updates profileArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**artist_profile_art_relationship_update_operation_payload** | Option<[**ArtistProfileArtRelationshipUpdateOperationPayload**](ArtistProfileArtRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
