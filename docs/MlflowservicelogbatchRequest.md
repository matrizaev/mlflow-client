# MlflowservicelogbatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | Option<**String**> | ID of the run to log under | [optional]
**metrics** | Option<[**Vec<models::Mlflowmetric>**](mlflowmetric.md)> | Metrics to log. A single request can contain up to 1000 metrics, and up to 1000 metrics, params, and tags in total. | [optional]
**params** | Option<[**Vec<models::Mlflowparam>**](mlflowparam.md)> | Params to log. A single request can contain up to 100 params, and up to 1000 metrics, params, and tags in total. | [optional]
**tags** | Option<[**Vec<models::Mlflowruntag>**](mlflowruntag.md)> | Tags to log. A single request can contain up to 100 tags, and up to 1000 metrics, params, and tags in total. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


