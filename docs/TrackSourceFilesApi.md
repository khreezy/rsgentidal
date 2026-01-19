# \TrackSourceFilesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_track_source_file**](TrackSourceFilesApi.md#create_track_source_file) | **POST** /trackSourceFiles | Create single trackSourceFile.
[**get_track_source_file**](TrackSourceFilesApi.md#get_track_source_file) | **GET** /trackSourceFiles/{id} | Get single trackSourceFile.
[**get_track_source_file_owners**](TrackSourceFilesApi.md#get_track_source_file_owners) | **GET** /trackSourceFiles/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_track_source_files**](TrackSourceFilesApi.md#get_track_source_files) | **GET** /trackSourceFiles | Get multiple trackSourceFiles.



## create_track_source_file

> models::TrackSourceFilesSingleResourceDataDocument create_track_source_file(track_source_file_create_operation_payload)
Create single trackSourceFile.

Create a track source file. <p/> The response contains a upload link that must be used to upload the actual content.<p/> The headers in the upload link response must be sent doing the actual upload. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_source_file_create_operation_payload** | Option<[**TrackSourceFileCreateOperationPayload**](TrackSourceFileCreateOperationPayload.md)> |  |  |

### Return type

[**models::TrackSourceFilesSingleResourceDataDocument**](TrackSourceFiles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_source_file

> models::TrackSourceFilesSingleResourceDataDocument get_track_source_file(id, include)
Get single trackSourceFile.

Retrieves single trackSourceFile by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track source file id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::TrackSourceFilesSingleResourceDataDocument**](TrackSourceFiles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_source_file_owners

> models::TrackSourceFilesMultiRelationshipDataDocument get_track_source_file_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track source file id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TrackSourceFilesMultiRelationshipDataDocument**](TrackSourceFiles_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_source_files

> models::TrackSourceFilesMultiResourceDataDocument get_track_source_files(include, filter_left_square_bracket_id_right_square_bracket)
Get multiple trackSourceFiles.

Retrieves multiple trackSourceFiles by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Track source file id |  |

### Return type

[**models::TrackSourceFilesMultiResourceDataDocument**](TrackSourceFiles_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

