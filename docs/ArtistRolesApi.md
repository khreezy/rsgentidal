# \ArtistRolesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_artist_role**](ArtistRolesApi.md#get_artist_role) | **GET** /artistRoles/{id} | Get single artistRole.
[**get_artist_roles**](ArtistRolesApi.md#get_artist_roles) | **GET** /artistRoles | Get multiple artistRoles.



## get_artist_role

> models::ArtistRolesSingleResourceDataDocument get_artist_role(id)
Get single artistRole.

Retrieves single artistRole by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist role id | [required] |

### Return type

[**models::ArtistRolesSingleResourceDataDocument**](ArtistRoles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_roles

> models::ArtistRolesMultiResourceDataDocument get_artist_roles(filter_left_square_bracket_id_right_square_bracket)
Get multiple artistRoles.

Retrieves multiple artistRoles by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artist role id |  |

### Return type

[**models::ArtistRolesMultiResourceDataDocument**](ArtistRoles_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

