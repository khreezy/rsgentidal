# \ProvidersApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_provider**](ProvidersApi.md#get_provider) | **GET** /providers/{id} | Get single provider.
[**get_providers**](ProvidersApi.md#get_providers) | **GET** /providers | Get multiple providers.



## get_provider

> models::ProvidersSingleResourceDataDocument get_provider(id)
Get single provider.

Retrieves single provider by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Provider ID | [required] |

### Return type

[**models::ProvidersSingleResourceDataDocument**](Providers_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_providers

> models::ProvidersMultiResourceDataDocument get_providers(filter_left_square_bracket_id_right_square_bracket)
Get multiple providers.

Retrieves multiple providers by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Provider ID |  |

### Return type

[**models::ProvidersMultiResourceDataDocument**](Providers_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

