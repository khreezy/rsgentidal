# \TrackManifestsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_track_manifest**](TrackManifestsApi.md#get_track_manifest) | **GET** /trackManifests/{id} | Get single trackManifest.



## get_track_manifest

> models::TrackManifestsSingleResourceDataDocument get_track_manifest(id, manifest_type, formats, uri_scheme, usage, adaptive)
Get single trackManifest.

Retrieves single trackManifest by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track manifest id | [required] |
**manifest_type** | **String** |  | [required] |
**formats** | [**Vec<String>**](String.md) |  | [required] |
**uri_scheme** | **String** |  | [required] |
**usage** | **String** |  | [required] |
**adaptive** | **bool** |  | [required] |

### Return type

[**models::TrackManifestsSingleResourceDataDocument**](TrackManifests_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

