# \TrackFilesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_track_file**](TrackFilesApi.md#get_track_file) | **GET** /trackFiles/{id} | Get single trackFile.



## get_track_file

> models::TrackFilesSingleResourceDataDocument get_track_file(id, formats, usage)
Get single trackFile.

Retrieves single trackFile by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track file id | [required] |
**formats** | [**Vec<String>**](String.md) |  | [required] |
**usage** | **String** |  | [required] |

### Return type

[**models::TrackFilesSingleResourceDataDocument**](TrackFiles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

