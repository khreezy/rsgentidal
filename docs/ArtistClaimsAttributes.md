# ArtistClaimsAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artist_id** | **String** | The artist id which is being claimed | 
**external_links** | Option<[**Vec<models::ExternalLink>**](External_Link.md)> | Artist claim links external to TIDAL API | [optional]
**provider** | **String** | The DSP used for authentication | 
**recommendation** | Option<**String**> | The recommended claim resolution | [optional]
**retrieved_upcs** | Option<[**Vec<models::BarcodeId>**](BarcodeId.md)> | List of UPCs retrieved from the DSP | [optional]
**status** | **String** | Current status of this claim | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


