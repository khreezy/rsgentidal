# \ManualArtistClaimsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_manual_artist_claim**](ManualArtistClaimsApi.md#create_manual_artist_claim) | **POST** /manualArtistClaims | Create single manualArtistClaim.



## create_manual_artist_claim

> models::ManualArtistClaimsSingleResourceDataDocument create_manual_artist_claim(manual_artist_claims_create_operation_payload)
Create single manualArtistClaim.

Creates a new manualArtistClaim.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manual_artist_claims_create_operation_payload** | Option<[**ManualArtistClaimsCreateOperationPayload**](ManualArtistClaimsCreateOperationPayload.md)> |  |  |

### Return type

[**models::ManualArtistClaimsSingleResourceDataDocument**](ManualArtistClaims_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

