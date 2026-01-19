# \ArtistClaimsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_artist_claim**](ArtistClaimsApi.md#create_artist_claim) | **POST** /artistClaims | Create single artistClaim.
[**get_artist_claim**](ArtistClaimsApi.md#get_artist_claim) | **GET** /artistClaims/{id} | Get single artistClaim.
[**get_artist_claim_accepted_artists**](ArtistClaimsApi.md#get_artist_claim_accepted_artists) | **GET** /artistClaims/{id}/relationships/acceptedArtists | Get acceptedArtists relationship (\"to-many\").
[**get_artist_claim_owners**](ArtistClaimsApi.md#get_artist_claim_owners) | **GET** /artistClaims/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**get_artist_claim_recommended_artists**](ArtistClaimsApi.md#get_artist_claim_recommended_artists) | **GET** /artistClaims/{id}/relationships/recommendedArtists | Get recommendedArtists relationship (\"to-many\").
[**patch_artist_claim**](ArtistClaimsApi.md#patch_artist_claim) | **PATCH** /artistClaims/{id} | Update single artistClaim.
[**update_artist_claim_accepted_artists**](ArtistClaimsApi.md#update_artist_claim_accepted_artists) | **PATCH** /artistClaims/{id}/relationships/acceptedArtists | Update acceptedArtists relationship (\"to-many\").



## create_artist_claim

> models::ArtistClaimsSingleResourceDataDocument create_artist_claim(country_code, artist_claims_create_operation_payload)
Create single artistClaim.

Creates a new artistClaim.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**artist_claims_create_operation_payload** | Option<[**ArtistClaimsCreateOperationPayload**](ArtistClaimsCreateOperationPayload.md)> |  |  |

### Return type

[**models::ArtistClaimsSingleResourceDataDocument**](ArtistClaims_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_claim

> models::ArtistClaimsSingleResourceDataDocument get_artist_claim(id, include)
Get single artistClaim.

Retrieves single artistClaim by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: acceptedArtists, owners, recommendedArtists |  |

### Return type

[**models::ArtistClaimsSingleResourceDataDocument**](ArtistClaims_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_claim_accepted_artists

> models::ArtistClaimsMultiRelationshipDataDocument get_artist_claim_accepted_artists(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get acceptedArtists relationship (\"to-many\").

Retrieves acceptedArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: acceptedArtists |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistClaimsMultiRelationshipDataDocument**](ArtistClaims_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_claim_owners

> models::ArtistClaimsMultiRelationshipDataDocument get_artist_claim_owners(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistClaimsMultiRelationshipDataDocument**](ArtistClaims_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_claim_recommended_artists

> models::ArtistClaimsMultiRelationshipDataDocument get_artist_claim_recommended_artists(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get recommendedArtists relationship (\"to-many\").

Retrieves recommendedArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: recommendedArtists |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistClaimsMultiRelationshipDataDocument**](ArtistClaims_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_artist_claim

> patch_artist_claim(id, country_code, artist_claims_update_operation_payload)
Update single artistClaim.

Updates existing artistClaim.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**artist_claims_update_operation_payload** | Option<[**ArtistClaimsUpdateOperationPayload**](ArtistClaimsUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artist_claim_accepted_artists

> update_artist_claim_accepted_artists(id, country_code, artist_claim_accepted_artists_relationship_update_operation_payload)
Update acceptedArtists relationship (\"to-many\").

Updates acceptedArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code |  |
**artist_claim_accepted_artists_relationship_update_operation_payload** | Option<[**ArtistClaimAcceptedArtistsRelationshipUpdateOperationPayload**](ArtistClaimAcceptedArtistsRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

