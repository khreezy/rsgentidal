# \SearchSuggestionsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search_suggestion**](SearchSuggestionsApi.md#get_search_suggestion) | **GET** /searchSuggestions/{id} | Get single searchSuggestion.
[**get_search_suggestion_direct_hits**](SearchSuggestionsApi.md#get_search_suggestion_direct_hits) | **GET** /searchSuggestions/{id}/relationships/directHits | Get directHits relationship (\"to-many\").



## get_search_suggestion

> models::SearchSuggestionsSingleResourceDataDocument get_search_suggestion(id, explicit_filter, country_code, include)
Get single searchSuggestion.

Retrieves single searchSuggestion by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: directHits |  |

### Return type

[**models::SearchSuggestionsSingleResourceDataDocument**](SearchSuggestions_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_suggestion_direct_hits

> models::SearchSuggestionsMultiRelationshipDataDocument get_search_suggestion_direct_hits(id, explicit_filter, country_code, include, page_cursor)
Get directHits relationship (\"to-many\").

Retrieves directHits relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query string used as the resource identifier | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: directHits |  |
**page_cursor** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchSuggestionsMultiRelationshipDataDocument**](SearchSuggestions_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
