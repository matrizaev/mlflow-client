# ModelregistryservicesetregisteredmodelaliasRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model. This field is required. | 
**alias** | **String** | Name of the alias. Maximum size depends on storage backend. If an alias with this name already exists, its preexisting value will be replaced by the specified `version`. All storage backends are guaranteed to support alias name values up to 256 bytes in size. This field is required. | 
**version** | **String** | Model version number. This field is required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


