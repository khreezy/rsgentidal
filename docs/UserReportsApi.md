# \UserReportsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_report**](UserReportsApi.md#create_user_report) | **POST** /userReports | Create single userReport.



## create_user_report

> models::UserReportsSingleResourceDataDocument create_user_report(user_report_create_operation_payload)
Create single userReport.

Creates a new userReport.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_report_create_operation_payload** | Option<[**UserReportCreateOperationPayload**](UserReportCreateOperationPayload.md)> |  |  |

### Return type

[**models::UserReportsSingleResourceDataDocument**](UserReports_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

