# MlflowservicelogparamRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | Option<**String**> | ID of the run under which to log the param. Must be provided. | [optional]
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run under which to log the param. This field will be removed in a future MLflow version. | [optional]
**key** | **String** | Name of the param. Maximum size is 255 bytes. This field is required. | 
**value** | **String** | String value of the param being logged. Maximum size is 500 bytes. This field is required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


