# Model20MlflowModelVersionsTransitionStagePostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model This field is required. | 
**version** | **String** | Model version number This field is required. | 
**stage** | **String** | Transition `model_version` to new stage. This field is required. | 
**archive_existing_versions** | **bool** | When transitioning a model version to a particular stage, this flag dictates whether all existing model versions in that stage should be atomically moved to the \"archived\" stage. This ensures that at-most-one model version exists in the target stage. This field is *required* when transitioning a model versions's stage This field is required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


