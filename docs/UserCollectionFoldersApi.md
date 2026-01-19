# \UserCollectionFoldersApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_collection_folder_items**](UserCollectionFoldersApi.md#add_user_collection_folder_items) | **POST** /userCollectionFolders/{id}/relationships/items | Add to items relationship (\"to-many\").
[**create_user_collection_folder**](UserCollectionFoldersApi.md#create_user_collection_folder) | **POST** /userCollectionFolders | Create single userCollectionFolder.
[**delete_user_collection_folder**](UserCollectionFoldersApi.md#delete_user_collection_folder) | **DELETE** /userCollectionFolders/{id} | Delete single userCollectionFolder.
[**delete_user_collection_folder_items**](UserCollectionFoldersApi.md#delete_user_collection_folder_items) | **DELETE** /userCollectionFolders/{id}/relationships/items | Delete from items relationship (\"to-many\").
[**get_user_collection_folder**](UserCollectionFoldersApi.md#get_user_collection_folder) | **GET** /userCollectionFolders/{id} | Get single userCollectionFolder.
[**get_user_collection_folder_items**](UserCollectionFoldersApi.md#get_user_collection_folder_items) | **GET** /userCollectionFolders/{id}/relationships/items | Get items relationship (\"to-many\").
[**get_user_collection_folder_owners**](UserCollectionFoldersApi.md#get_user_collection_folder_owners) | **GET** /userCollectionFolders/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_user_collection_folders**](UserCollectionFoldersApi.md#get_user_collection_folders) | **GET** /userCollectionFolders | Get multiple userCollectionFolders.
[**patch_user_collection_folder**](UserCollectionFoldersApi.md#patch_user_collection_folder) | **PATCH** /userCollectionFolders/{id} | Update single userCollectionFolder.



## add_user_collection_folder_items

> add_user_collection_folder_items(id, add_payload)
Add to items relationship (\"to-many\").

Adds item(s) to items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Folder Id | [required] |
**add_payload** | Option<[**AddPayload**](AddPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_collection_folder

> models::UserCollectionFoldersSingleResourceDataDocument create_user_collection_folder(folder_create_operation_payload)
Create single userCollectionFolder.

Creates a new userCollectionFolder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_create_operation_payload** | Option<[**FolderCreateOperationPayload**](FolderCreateOperationPayload.md)> |  |  |

### Return type

[**models::UserCollectionFoldersSingleResourceDataDocument**](UserCollectionFolders_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_collection_folder

> delete_user_collection_folder(id)
Delete single userCollectionFolder.

Deletes existing userCollectionFolder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Folder Id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_collection_folder_items

> delete_user_collection_folder_items(id, remove_payload)
Delete from items relationship (\"to-many\").

Deletes item(s) from items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Folder Id | [required] |
**remove_payload** | Option<[**RemovePayload**](RemovePayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_folder

> models::UserCollectionFoldersSingleResourceDataDocument get_user_collection_folder(id, include)
Get single userCollectionFolder.

Retrieves single userCollectionFolder by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Folder Id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: items, owners |  |

### Return type

[**models::UserCollectionFoldersSingleResourceDataDocument**](UserCollectionFolders_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_folder_items

> models::UserCollectionFoldersItemsMultiRelationshipDataDocument get_user_collection_folder_items(id, page_left_square_bracket_cursor_right_square_bracket, sort, include)
Get items relationship (\"to-many\").

Retrieves items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Folder Id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: items |  |

### Return type

[**models::UserCollectionFoldersItemsMultiRelationshipDataDocument**](UserCollectionFolders_Items_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_folder_owners

> models::UserCollectionFoldersMultiRelationshipDataDocument get_user_collection_folder_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Folder Id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserCollectionFoldersMultiRelationshipDataDocument**](UserCollectionFolders_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_collection_folders

> models::UserCollectionFoldersMultiResourceDataDocument get_user_collection_folders(include, filter_left_square_bracket_id_right_square_bracket)
Get multiple userCollectionFolders.

Retrieves multiple userCollectionFolders by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: items, owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Folder Id |  |

### Return type

[**models::UserCollectionFoldersMultiResourceDataDocument**](UserCollectionFolders_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_collection_folder

> patch_user_collection_folder(id, folder_update_operation_payload)
Update single userCollectionFolder.

Updates existing userCollectionFolder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Folder Id | [required] |
**folder_update_operation_payload** | Option<[**FolderUpdateOperationPayload**](FolderUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

