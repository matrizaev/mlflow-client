# MlflowserviceupdaterunRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | Option<**String**> | ID of the run to update. Must be provided. | [optional]
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run to update.. This field will be removed in a future MLflow version. | [optional]
**status** | Option<[**models::Mlflowrunstatus**](Mlflowrunstatus.md)> |  | [optional]
**end_time** | Option<**i64**> | Unix timestamp in milliseconds of when the run ended. | [optional]
**run_name** | Option<**String**> | Updated name of the run. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


