# \UserEntitlementsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_entitlement**](UserEntitlementsApi.md#get_user_entitlement) | **GET** /userEntitlements/{id} | Get single userEntitlement.
[**get_user_entitlement_owners**](UserEntitlementsApi.md#get_user_entitlement_owners) | **GET** /userEntitlements/{id}/relationships/owners | Get owners relationship (\"to-many\").



## get_user_entitlement

> models::UserEntitlementsSingleResourceDataDocument get_user_entitlement(id, include)
Get single userEntitlement.

Retrieves single userEntitlement by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::UserEntitlementsSingleResourceDataDocument**](UserEntitlements_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_entitlement_owners

> models::UserEntitlementsMultiRelationshipDataDocument get_user_entitlement_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserEntitlementsMultiRelationshipDataDocument**](UserEntitlements_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

