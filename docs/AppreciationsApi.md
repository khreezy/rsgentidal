# \AppreciationsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_appreciation**](AppreciationsApi.md#create_appreciation) | **POST** /appreciations | Create single appreciation.



## create_appreciation

> models::AppreciationsSingleResourceDataDocument create_appreciation(appreciations_create_operation_payload)
Create single appreciation.

Creates a new appreciation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appreciations_create_operation_payload** | Option<[**AppreciationsCreateOperationPayload**](AppreciationsCreateOperationPayload.md)> |  |  |

### Return type

[**models::AppreciationsSingleResourceDataDocument**](Appreciations_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

