# MlflowservicesetexperimenttagRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**experiment_id** | **String** | ID of the experiment under which to log the tag. Must be provided. This field is required. | 
**key** | **String** | Name of the tag. Maximum size depends on storage backend. All storage backends are guaranteed to support key values up to 250 bytes in size. This field is required. | 
**value** | **String** | String value of the tag being logged. Maximum size depends on storage backend. All storage backends are guaranteed to support key values up to 5000 bytes in size. This field is required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


