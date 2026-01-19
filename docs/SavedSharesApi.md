# \SavedSharesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_saved_share**](SavedSharesApi.md#create_saved_share) | **POST** /savedShares | Create single savedShare.



## create_saved_share

> models::SavedSharesSingleResourceDataDocument create_saved_share(saved_shares_create_operation_payload)
Create single savedShare.

Creates a new savedShare.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saved_shares_create_operation_payload** | Option<[**SavedSharesCreateOperationPayload**](SavedSharesCreateOperationPayload.md)> |  |  |

### Return type

[**models::SavedSharesSingleResourceDataDocument**](SavedShares_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

