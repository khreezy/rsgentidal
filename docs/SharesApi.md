# \SharesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_share**](SharesApi.md#create_share) | **POST** /shares | Create single share.
[**get_share**](SharesApi.md#get_share) | **GET** /shares/{id} | Get single share.
[**get_share_owners**](SharesApi.md#get_share_owners) | **GET** /shares/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_share_shared_resources**](SharesApi.md#get_share_shared_resources) | **GET** /shares/{id}/relationships/sharedResources | Get sharedResources relationship (\"to-many\").
[**get_shares**](SharesApi.md#get_shares) | **GET** /shares | Get multiple shares.



## create_share

> models::SharesSingleResourceDataDocument create_share(shares_create_operation_payload)
Create single share.

Creates a new share.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shares_create_operation_payload** | Option<[**SharesCreateOperationPayload**](SharesCreateOperationPayload.md)> |  |  |

### Return type

[**models::SharesSingleResourceDataDocument**](Shares_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_share

> models::SharesSingleResourceDataDocument get_share(id, include)
Get single share.

Retrieves single share by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User share id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, sharedResources |  |

### Return type

[**models::SharesSingleResourceDataDocument**](Shares_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_share_owners

> models::SharesMultiRelationshipDataDocument get_share_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User share id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SharesMultiRelationshipDataDocument**](Shares_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_share_shared_resources

> models::SharesMultiRelationshipDataDocument get_share_shared_resources(id, page_left_square_bracket_cursor_right_square_bracket, include)
Get sharedResources relationship (\"to-many\").

Retrieves sharedResources relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User share id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: sharedResources |  |

### Return type

[**models::SharesMultiRelationshipDataDocument**](Shares_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shares

> models::SharesMultiResourceDataDocument get_shares(include, filter_left_square_bracket_code_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple shares.

Retrieves multiple shares by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, sharedResources |  |
**filter_left_square_bracket_code_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Share code |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User share id |  |

### Return type

[**models::SharesMultiResourceDataDocument**](Shares_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

