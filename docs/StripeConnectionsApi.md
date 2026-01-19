# \StripeConnectionsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stripe_connection**](StripeConnectionsApi.md#create_stripe_connection) | **POST** /stripeConnections | Create single stripeConnection.
[**get_stripe_connection_owners**](StripeConnectionsApi.md#get_stripe_connection_owners) | **GET** /stripeConnections/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_stripe_connections**](StripeConnectionsApi.md#get_stripe_connections) | **GET** /stripeConnections | Get multiple stripeConnections.



## create_stripe_connection

> models::StripeConnectionsSingleResourceDataDocument create_stripe_connection(country_code, stripe_connections_create_operation_payload)
Create single stripeConnection.

Creates a new stripeConnection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**stripe_connections_create_operation_payload** | Option<[**StripeConnectionsCreateOperationPayload**](StripeConnectionsCreateOperationPayload.md)> |  |  |

### Return type

[**models::StripeConnectionsSingleResourceDataDocument**](StripeConnections_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stripe_connection_owners

> models::StripeConnectionsMultiRelationshipDataDocument get_stripe_connection_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Stripe connection id (same as user id) | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::StripeConnectionsMultiRelationshipDataDocument**](StripeConnections_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stripe_connections

> models::StripeConnectionsMultiResourceDataDocument get_stripe_connections(include, filter_left_square_bracket_owners_id_right_square_bracket)
Get multiple stripeConnections.

Retrieves multiple stripeConnections by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_owners_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |

### Return type

[**models::StripeConnectionsMultiResourceDataDocument**](StripeConnections_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

