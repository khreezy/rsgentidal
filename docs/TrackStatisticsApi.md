# \TrackStatisticsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_track_statistic**](TrackStatisticsApi.md#get_track_statistic) | **GET** /trackStatistics/{id} | Get single trackStatistic.
[**get_track_statistic_owners**](TrackStatisticsApi.md#get_track_statistic_owners) | **GET** /trackStatistics/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_track_statistics**](TrackStatisticsApi.md#get_track_statistics) | **GET** /trackStatistics | Get multiple trackStatistics.



## get_track_statistic

> models::TrackStatisticsSingleResourceDataDocument get_track_statistic(id, include)
Get single trackStatistic.

Retrieves single trackStatistic by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track statistic id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::TrackStatisticsSingleResourceDataDocument**](TrackStatistics_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_statistic_owners

> models::TrackStatisticsMultiRelationshipDataDocument get_track_statistic_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track statistic id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TrackStatisticsMultiRelationshipDataDocument**](TrackStatistics_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_statistics

> models::TrackStatisticsMultiResourceDataDocument get_track_statistics(include, filter_left_square_bracket_id_right_square_bracket)
Get multiple trackStatistics.

Retrieves multiple trackStatistics by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Track id |  |

### Return type

[**models::TrackStatisticsMultiResourceDataDocument**](TrackStatistics_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

