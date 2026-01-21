# \GenresApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_genre**](GenresApi.md#get_genre) | **GET** /genres/{id} | Get single genre.
[**get_genres**](GenresApi.md#get_genres) | **GET** /genres | Get multiple genres.



## get_genre

> models::GenresSingleResourceDataDocument get_genre(id, locale)
Get single genre.

Retrieves single genre by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genre id | [required] |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]

### Return type

[**models::GenresSingleResourceDataDocument**](Genres_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_genres

> models::GenresMultiResourceDataDocument get_genres(page_cursor, locale, filter_left_square_bracket_id_right_square_bracket)
Get multiple genres.

Retrieves multiple genres by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Allows filtering by genre id(s). USER_SELECTABLE is special value used to return specific genres which users can select from |  |

### Return type

[**models::GenresMultiResourceDataDocument**](Genres_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
