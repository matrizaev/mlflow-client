# Modelregistryservicegetlatestversions200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model_versions** | Option<[**Vec<models::Mlflowmodelversion>**](Mlflowmodelversion.md)> | Latest version models for each requests stage. Only return models with current ``READY`` status. If no ``stages`` provided, returns the latest version for each stage, including ``\"None\"``. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


