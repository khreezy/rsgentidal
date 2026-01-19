# \DynamicPagesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dynamic_page**](DynamicPagesApi.md#get_dynamic_page) | **GET** /dynamicPages | Get multiple dynamicPages.
[**get_dynamic_pages_subject**](DynamicPagesApi.md#get_dynamic_pages_subject) | **GET** /dynamicPages/{id}/relationships/subject | Get subject relationship (\"to-one\").



## get_dynamic_page

> models::DynamicPagesMultiResourceDataDocument get_dynamic_page(client_version, device_type, platform, refresh_id, country_code, locale, include, filter_left_square_bracket_page_type_right_square_bracket, filter_left_square_bracket_subject_id_right_square_bracket)
Get multiple dynamicPages.

Retrieves multiple dynamicPages by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_version** | **String** |  | [required] |
**device_type** | **String** | The type of device making the request | [required] |
**platform** | **String** | The platform of the device making the request | [required] |
**refresh_id** | Option<**i64**> |  |  |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**locale** | Option<**String**> | BCP 47 locale (e.g., en-US, nb-NO, pt-BR). Defaults to en-US if not provided or unsupported. |  |[default to en-US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: subject |  |
**filter_left_square_bracket_page_type_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Filter by page type |  |
**filter_left_square_bracket_subject_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Filter by subject id |  |

### Return type

[**models::DynamicPagesMultiResourceDataDocument**](DynamicPages_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dynamic_pages_subject

> models::DynamicPagesSingleRelationshipDataDocument get_dynamic_pages_subject(id, include)
Get subject relationship (\"to-one\").

Retrieves subject relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | DynamicPages Id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: subject |  |

### Return type

[**models::DynamicPagesSingleRelationshipDataDocument**](DynamicPages_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

