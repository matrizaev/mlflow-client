# MlflowservicecreateexperimentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Experiment name. This field is required. | 
**artifact_location** | Option<**String**> | Location where all artifacts for the experiment are stored. If not provided, the remote server will select an appropriate default. | [optional]
**tags** | Option<[**Vec<models::Mlflowexperimenttag>**](mlflowexperimenttag.md)> | A collection of tags to set on the experiment. Maximum tag size and number of tags per request depends on the storage backend. All storage backends are guaranteed to support tag keys up to 250 bytes in size and tag values up to 5000 bytes in size. All storage backends are also guaranteed to support up to 20 tags per request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


