# \VideosApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_video**](VideosApi.md#get_video) | **GET** /videos/{id} | Get single video.
[**get_video_albums**](VideosApi.md#get_video_albums) | **GET** /videos/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**get_video_artists**](VideosApi.md#get_video_artists) | **GET** /videos/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**get_video_providers**](VideosApi.md#get_video_providers) | **GET** /videos/{id}/relationships/providers | Get providers relationship (\"to-many\").
[**get_video_replacement_relationship**](VideosApi.md#get_video_replacement_relationship) | **GET** /videos/{id}/relationships/replacement | Get replacement relationship (\"to-one\").
[**get_video_thumbnail_art**](VideosApi.md#get_video_thumbnail_art) | **GET** /videos/{id}/relationships/thumbnailArt | Get thumbnailArt relationship (\"to-many\").
[**get_videos**](VideosApi.md#get_videos) | **GET** /videos | Get multiple videos.



## get_video

> models::VideosSingleResourceDataDocument get_video(id, country_code, include)
Get single video.

Retrieves single video by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, providers, replacement, thumbnailArt |  |

### Return type

[**models::VideosSingleResourceDataDocument**](Videos_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_albums

> models::VideosMultiRelationshipDataDocument get_video_albums(id, page_left_square_bracket_cursor_right_square_bracket, country_code, include)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_artists

> models::VideosMultiRelationshipDataDocument get_video_artists(id, page_left_square_bracket_cursor_right_square_bracket, country_code, include)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_providers

> models::VideosMultiRelationshipDataDocument get_video_providers(id, page_left_square_bracket_cursor_right_square_bracket, country_code, include)
Get providers relationship (\"to-many\").

Retrieves providers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: providers |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_replacement_relationship

> models::VideosSingleRelationshipDataDocument get_video_replacement_relationship(id, country_code, include)
Get replacement relationship (\"to-one\").

Retrieves replacement relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: replacement |  |

### Return type

[**models::VideosSingleRelationshipDataDocument**](Videos_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_thumbnail_art

> models::VideosMultiRelationshipDataDocument get_video_thumbnail_art(id, page_left_square_bracket_cursor_right_square_bracket, country_code, include)
Get thumbnailArt relationship (\"to-many\").

Retrieves thumbnailArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: thumbnailArt |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_videos

> models::VideosMultiResourceDataDocument get_videos(country_code, include, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_isrc_right_square_bracket)
Get multiple videos.

Retrieves multiple videos by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, providers, replacement, thumbnailArt |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Video id |  |
**filter_left_square_bracket_isrc_right_square_bracket** | Option<[**Vec<String>**](String.md)> | International Standard Recording Code (ISRC) |  |

### Return type

[**models::VideosMultiResourceDataDocument**](Videos_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

