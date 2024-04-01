# Model20MlflowModelVersionsSetTagPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Unique name of the model. This field is required. | 
**version** | **String** | Model version number. This field is required. | 
**key** | **String** | Name of the tag. Maximum size depends on storage backend. If a tag with this name already exists, its preexisting value will be replaced by the specified `value`. All storage backends are guaranteed to support key values up to 250 bytes in size. This field is required. | 
**value** | **String** | String value of the tag being logged. Maximum size depends on storage backend. This field is required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


