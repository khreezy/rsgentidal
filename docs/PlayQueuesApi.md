# \PlayQueuesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_future_play_queue**](PlayQueuesApi.md#create_future_play_queue) | **POST** /playQueues/{id}/relationships/future | Add to future relationship (\"to-many\").
[**create_play_queue**](PlayQueuesApi.md#create_play_queue) | **POST** /playQueues | Create single playQueue.
[**delete_future_play_queues**](PlayQueuesApi.md#delete_future_play_queues) | **DELETE** /playQueues/{id}/relationships/future | Delete from future relationship (\"to-many\").
[**delete_play_queue**](PlayQueuesApi.md#delete_play_queue) | **DELETE** /playQueues/{id} | Delete single playQueue.
[**get_current_play_queue**](PlayQueuesApi.md#get_current_play_queue) | **GET** /playQueues/{id}/relationships/current | Get current relationship (\"to-one\").
[**get_future_play_queue**](PlayQueuesApi.md#get_future_play_queue) | **GET** /playQueues/{id}/relationships/future | Get future relationship (\"to-many\").
[**get_past_play_queues**](PlayQueuesApi.md#get_past_play_queues) | **GET** /playQueues/{id}/relationships/past | Get past relationship (\"to-many\").
[**get_play_queue**](PlayQueuesApi.md#get_play_queue) | **GET** /playQueues/{id} | Get single playQueue.
[**get_play_queue_owners**](PlayQueuesApi.md#get_play_queue_owners) | **GET** /playQueues/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_play_queues**](PlayQueuesApi.md#get_play_queues) | **GET** /playQueues | Get multiple playQueues.
[**patch_current_play_queue**](PlayQueuesApi.md#patch_current_play_queue) | **PATCH** /playQueues/{id}/relationships/current | Update current relationship (\"to-one\").
[**patch_future_play_queue**](PlayQueuesApi.md#patch_future_play_queue) | **PATCH** /playQueues/{id}/relationships/future | Update future relationship (\"to-many\").
[**patch_play_queue**](PlayQueuesApi.md#patch_play_queue) | **PATCH** /playQueues/{id} | Update single playQueue.



## create_future_play_queue

> create_future_play_queue(id, play_queue_add_future_operation_payload)
Add to future relationship (\"to-many\").

Adds item(s) to future relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**play_queue_add_future_operation_payload** | Option<[**PlayQueueAddFutureOperationPayload**](PlayQueueAddFutureOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_play_queue

> models::PlayQueuesSingleResourceDataDocument create_play_queue(play_queue_create_operation_payload)
Create single playQueue.

Creates a new playQueue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**play_queue_create_operation_payload** | Option<[**PlayQueueCreateOperationPayload**](PlayQueueCreateOperationPayload.md)> |  |  |

### Return type

[**models::PlayQueuesSingleResourceDataDocument**](PlayQueues_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_future_play_queues

> delete_future_play_queues(id, play_queue_remove_future_operation_payload)
Delete from future relationship (\"to-many\").

Deletes item(s) from future relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**play_queue_remove_future_operation_payload** | Option<[**PlayQueueRemoveFutureOperationPayload**](PlayQueueRemoveFutureOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_play_queue

> delete_play_queue(id)
Delete single playQueue.

Deletes existing playQueue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_play_queue

> models::PlayQueuesCurrentSingleRelationshipDataDocument get_current_play_queue(id, include)
Get current relationship (\"to-one\").

Retrieves current relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: current |  |

### Return type

[**models::PlayQueuesCurrentSingleRelationshipDataDocument**](PlayQueues_Current_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_future_play_queue

> models::PlayQueuesFutureMultiRelationshipDataDocument get_future_play_queue(id, page_left_square_bracket_cursor_right_square_bracket, include)
Get future relationship (\"to-many\").

Retrieves future relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: future |  |

### Return type

[**models::PlayQueuesFutureMultiRelationshipDataDocument**](PlayQueues_Future_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_past_play_queues

> models::PlayQueuesPastMultiRelationshipDataDocument get_past_play_queues(id, page_left_square_bracket_cursor_right_square_bracket, include)
Get past relationship (\"to-many\").

Retrieves past relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: past |  |

### Return type

[**models::PlayQueuesPastMultiRelationshipDataDocument**](PlayQueues_Past_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_play_queue

> models::PlayQueuesSingleResourceDataDocument get_play_queue(id, include)
Get single playQueue.

Retrieves single playQueue by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: current, future, owners, past |  |

### Return type

[**models::PlayQueuesSingleResourceDataDocument**](PlayQueues_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_play_queue_owners

> models::PlayQueuesMultiRelationshipDataDocument get_play_queue_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::PlayQueuesMultiRelationshipDataDocument**](PlayQueues_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_play_queues

> models::PlayQueuesMultiResourceDataDocument get_play_queues(page_left_square_bracket_cursor_right_square_bracket, include, filter_left_square_bracket_owners_id_right_square_bracket)
Get multiple playQueues.

Retrieves multiple playQueues by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: current, future, owners, past |  |
**filter_left_square_bracket_owners_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |

### Return type

[**models::PlayQueuesMultiResourceDataDocument**](PlayQueues_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_current_play_queue

> patch_current_play_queue(id, play_queue_update_current_operations_payload)
Update current relationship (\"to-one\").

Updates current relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**play_queue_update_current_operations_payload** | Option<[**PlayQueueUpdateCurrentOperationsPayload**](PlayQueueUpdateCurrentOperationsPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_future_play_queue

> patch_future_play_queue(id, play_queue_update_future_operation_payload)
Update future relationship (\"to-many\").

Updates future relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**play_queue_update_future_operation_payload** | Option<[**PlayQueueUpdateFutureOperationPayload**](PlayQueueUpdateFutureOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_play_queue

> patch_play_queue(id, play_queue_update_operation_payload)
Update single playQueue.

Updates existing playQueue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Play queue id | [required] |
**play_queue_update_operation_payload** | Option<[**PlayQueueUpdateOperationPayload**](PlayQueueUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

