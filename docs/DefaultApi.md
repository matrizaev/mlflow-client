# \DefaultApi

All URIs are relative to *http://localhost:5000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mlflowservicecreateexperiment**](DefaultApi.md#mlflowservicecreateexperiment) | **POST** /2.0/mlflow/experiments/create | 
[**mlflowservicecreaterun**](DefaultApi.md#mlflowservicecreaterun) | **POST** /2.0/mlflow/runs/create | 
[**mlflowservicedeleteexperiment**](DefaultApi.md#mlflowservicedeleteexperiment) | **POST** /2.0/mlflow/experiments/delete | 
[**mlflowservicedeleterun**](DefaultApi.md#mlflowservicedeleterun) | **POST** /2.0/mlflow/runs/delete | 
[**mlflowservicedeletetag**](DefaultApi.md#mlflowservicedeletetag) | **POST** /2.0/mlflow/runs/delete-tag | 
[**mlflowservicegetexperiment**](DefaultApi.md#mlflowservicegetexperiment) | **GET** /2.0/mlflow/experiments/get | 
[**mlflowservicegetexperimentbyname**](DefaultApi.md#mlflowservicegetexperimentbyname) | **GET** /2.0/mlflow/experiments/get-by-name | 
[**mlflowservicegetmetrichistory**](DefaultApi.md#mlflowservicegetmetrichistory) | **GET** /2.0/mlflow/metrics/get-history | 
[**mlflowservicegetrun**](DefaultApi.md#mlflowservicegetrun) | **GET** /2.0/mlflow/runs/get | 
[**mlflowservicelistartifacts**](DefaultApi.md#mlflowservicelistartifacts) | **GET** /2.0/mlflow/artifacts/list | 
[**mlflowservicelogbatch**](DefaultApi.md#mlflowservicelogbatch) | **POST** /2.0/mlflow/runs/log-batch | 
[**mlflowserviceloginputs**](DefaultApi.md#mlflowserviceloginputs) | **POST** /2.0/mlflow/runs/log-inputs | 
[**mlflowservicelogmetric**](DefaultApi.md#mlflowservicelogmetric) | **POST** /2.0/mlflow/runs/log-metric | 
[**mlflowservicelogmodel**](DefaultApi.md#mlflowservicelogmodel) | **POST** /2.0/mlflow/runs/log-model | 
[**mlflowservicelogparam**](DefaultApi.md#mlflowservicelogparam) | **POST** /2.0/mlflow/runs/log-parameter | 
[**mlflowservicerestoreexperiment**](DefaultApi.md#mlflowservicerestoreexperiment) | **POST** /2.0/mlflow/experiments/restore | 
[**mlflowservicerestorerun**](DefaultApi.md#mlflowservicerestorerun) | **POST** /2.0/mlflow/runs/restore | 
[**mlflowservicesearchexperiments**](DefaultApi.md#mlflowservicesearchexperiments) | **POST** /2.0/mlflow/experiments/search | 
[**mlflowservicesearchruns**](DefaultApi.md#mlflowservicesearchruns) | **POST** /2.0/mlflow/runs/search | 
[**mlflowservicesetexperimenttag**](DefaultApi.md#mlflowservicesetexperimenttag) | **POST** /2.0/mlflow/experiments/set-experiment-tag | 
[**mlflowservicesettag**](DefaultApi.md#mlflowservicesettag) | **POST** /2.0/mlflow/runs/set-tag | 
[**mlflowserviceupdateexperiment**](DefaultApi.md#mlflowserviceupdateexperiment) | **POST** /2.0/mlflow/experiments/update | 
[**mlflowserviceupdaterun**](DefaultApi.md#mlflowserviceupdaterun) | **POST** /2.0/mlflow/runs/update | 
[**modelregistryservicecreatemodelversion**](DefaultApi.md#modelregistryservicecreatemodelversion) | **POST** /2.0/mlflow/model-versions/create | 
[**modelregistryservicecreateregisteredmodel**](DefaultApi.md#modelregistryservicecreateregisteredmodel) | **POST** /2.0/mlflow/registered-models/create | 
[**modelregistryservicedeletemodelversion**](DefaultApi.md#modelregistryservicedeletemodelversion) | **DELETE** /2.0/mlflow/model-versions/delete | 
[**modelregistryservicedeletemodelversiontag**](DefaultApi.md#modelregistryservicedeletemodelversiontag) | **DELETE** /2.0/mlflow/model-versions/delete-tag | 
[**modelregistryservicedeleteregisteredmodel**](DefaultApi.md#modelregistryservicedeleteregisteredmodel) | **DELETE** /2.0/mlflow/registered-models/delete | 
[**modelregistryservicedeleteregisteredmodeltag**](DefaultApi.md#modelregistryservicedeleteregisteredmodeltag) | **DELETE** /2.0/mlflow/registered-models/delete-tag | 
[**modelregistryservicegetlatestversions**](DefaultApi.md#modelregistryservicegetlatestversions) | **POST** /2.0/mlflow/registered-models/get-latest-versions | 
[**modelregistryservicegetmodelversion**](DefaultApi.md#modelregistryservicegetmodelversion) | **GET** /2.0/mlflow/model-versions/get | 
[**modelregistryservicegetmodelversiondownloaduri**](DefaultApi.md#modelregistryservicegetmodelversiondownloaduri) | **GET** /2.0/mlflow/model-versions/get-download-uri | 
[**modelregistryservicegetregisteredmodel**](DefaultApi.md#modelregistryservicegetregisteredmodel) | **GET** /2.0/mlflow/registered-models/get | 
[**modelregistryservicerenameregisteredmodel**](DefaultApi.md#modelregistryservicerenameregisteredmodel) | **POST** /2.0/mlflow/registered-models/rename | 
[**modelregistryservicesearchmodelversions**](DefaultApi.md#modelregistryservicesearchmodelversions) | **GET** /2.0/mlflow/model-versions/search | 
[**modelregistryservicesearchregisteredmodels**](DefaultApi.md#modelregistryservicesearchregisteredmodels) | **GET** /2.0/mlflow/registered-models/search | 
[**modelregistryservicesetmodelversiontag**](DefaultApi.md#modelregistryservicesetmodelversiontag) | **POST** /2.0/mlflow/model-versions/set-tag | 
[**modelregistryservicesetregisteredmodelalias**](DefaultApi.md#modelregistryservicesetregisteredmodelalias) | **POST** /2.0/mlflow/registered-models/alias | 
[**modelregistryservicesetregisteredmodeltag**](DefaultApi.md#modelregistryservicesetregisteredmodeltag) | **POST** /2.0/mlflow/registered-models/set-tag | 
[**modelregistryservicetransitionmodelversionstage**](DefaultApi.md#modelregistryservicetransitionmodelversionstage) | **POST** /2.0/mlflow/model-versions/transition-stage | 
[**modelregistryserviceupdatemodelversion**](DefaultApi.md#modelregistryserviceupdatemodelversion) | **PATCH** /2.0/mlflow/model-versions/update | 
[**modelregistryserviceupdateregisteredmodel**](DefaultApi.md#modelregistryserviceupdateregisteredmodel) | **PATCH** /2.0/mlflow/registered-models/update | 



## mlflowservicecreateexperiment

> models::Mlflowservicecreateexperiment200Response mlflowservicecreateexperiment(mlflowservicecreateexperiment_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicecreateexperiment_request** | [**MlflowservicecreateexperimentRequest**](MlflowservicecreateexperimentRequest.md) | Request body | [required] |

### Return type

[**models::Mlflowservicecreateexperiment200Response**](mlflowservicecreateexperiment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicecreaterun

> models::Mlflowservicecreaterun200Response mlflowservicecreaterun(mlflowservicecreaterun_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicecreaterun_request** | Option<[**MlflowservicecreaterunRequest**](MlflowservicecreaterunRequest.md)> | Request body |  |

### Return type

[**models::Mlflowservicecreaterun200Response**](mlflowservicecreaterun_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicedeleteexperiment

> mlflowservicedeleteexperiment(mlflowservicedeleteexperiment_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicedeleteexperiment_request** | [**MlflowservicedeleteexperimentRequest**](MlflowservicedeleteexperimentRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicedeleterun

> mlflowservicedeleterun(mlflowservicedeleterun_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicedeleterun_request** | [**MlflowservicedeleterunRequest**](MlflowservicedeleterunRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicedeletetag

> mlflowservicedeletetag(mlflowservicedeletetag_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicedeletetag_request** | [**MlflowservicedeletetagRequest**](MlflowservicedeletetagRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicegetexperiment

> models::Mlflowservicegetexperiment200Response mlflowservicegetexperiment(experiment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_id** | **String** | ID of the associated experiment. This field is required. | [required] |

### Return type

[**models::Mlflowservicegetexperiment200Response**](mlflowservicegetexperiment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicegetexperimentbyname

> models::Mlflowservicegetexperiment200Response mlflowservicegetexperimentbyname(experiment_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_name** | **String** | Name of the associated experiment. This field is required. | [required] |

### Return type

[**models::Mlflowservicegetexperiment200Response**](mlflowservicegetexperiment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicegetmetrichistory

> models::Mlflowservicegetmetrichistory200Response mlflowservicegetmetrichistory(metric_key, run_id, run_uuid, page_token, max_results)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_key** | **String** | Name of the metric. This field is required. | [required] |
**run_id** | Option<**String**> | ID of the run from which to fetch metric values. Must be provided. |  |
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run from which to fetch metric values. This field will be removed in a future MLflow version. |  |
**page_token** | Option<**String**> | Token indicating the page of metric history to fetch |  |
**max_results** | Option<**i32**> | Maximum number of logged instances of a metric for a run to return per call. Backend servers may restrict the value of `max_results` depending on performance requirements. Requests that do not specify this value will behave as non-paginated queries where all metric history values for a given metric within a run are returned in a single response. |  |

### Return type

[**models::Mlflowservicegetmetrichistory200Response**](mlflowservicegetmetrichistory_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicegetrun

> models::Mlflowservicecreaterun200Response mlflowservicegetrun(run_id, run_uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | Option<**String**> | ID of the run to fetch. Must be provided. |  |
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run to fetch. This field will be removed in a future MLflow version. |  |

### Return type

[**models::Mlflowservicecreaterun200Response**](mlflowservicecreaterun_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicelistartifacts

> models::Mlflowservicelistartifacts200Response mlflowservicelistartifacts(run_id, run_uuid, path, page_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | Option<**String**> | ID of the run whose artifacts to list. Must be provided. |  |
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run whose artifacts to list. This field will be removed in a future MLflow version. |  |
**path** | Option<**String**> | Filter artifacts matching this path (a relative path from the root artifact directory). |  |
**page_token** | Option<**String**> | Token indicating the page of artifact results to fetch |  |

### Return type

[**models::Mlflowservicelistartifacts200Response**](mlflowservicelistartifacts_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicelogbatch

> mlflowservicelogbatch(mlflowservicelogbatch_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicelogbatch_request** | Option<[**MlflowservicelogbatchRequest**](MlflowservicelogbatchRequest.md)> | Request body |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowserviceloginputs

> mlflowserviceloginputs(mlflowserviceloginputs_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowserviceloginputs_request** | [**MlflowserviceloginputsRequest**](MlflowserviceloginputsRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicelogmetric

> mlflowservicelogmetric(mlflowservicelogmetric_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicelogmetric_request** | [**MlflowservicelogmetricRequest**](MlflowservicelogmetricRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicelogmodel

> mlflowservicelogmodel(mlflowservicelogmodel_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicelogmodel_request** | Option<[**MlflowservicelogmodelRequest**](MlflowservicelogmodelRequest.md)> | Request body |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicelogparam

> mlflowservicelogparam(mlflowservicelogparam_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicelogparam_request** | [**MlflowservicelogparamRequest**](MlflowservicelogparamRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicerestoreexperiment

> mlflowservicerestoreexperiment(mlflowservicedeleteexperiment_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicedeleteexperiment_request** | [**MlflowservicedeleteexperimentRequest**](MlflowservicedeleteexperimentRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicerestorerun

> mlflowservicerestorerun(mlflowservicerestorerun_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicerestorerun_request** | [**MlflowservicerestorerunRequest**](MlflowservicerestorerunRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicesearchexperiments

> models::Mlflowservicesearchexperiments200Response mlflowservicesearchexperiments(mlflowservicesearchexperiments_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicesearchexperiments_request** | Option<[**MlflowservicesearchexperimentsRequest**](MlflowservicesearchexperimentsRequest.md)> | Request body |  |

### Return type

[**models::Mlflowservicesearchexperiments200Response**](mlflowservicesearchexperiments_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicesearchruns

> models::Mlflowservicesearchruns200Response mlflowservicesearchruns(mlflowservicesearchruns_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicesearchruns_request** | Option<[**MlflowservicesearchrunsRequest**](MlflowservicesearchrunsRequest.md)> | Request body |  |

### Return type

[**models::Mlflowservicesearchruns200Response**](mlflowservicesearchruns_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicesetexperimenttag

> mlflowservicesetexperimenttag(mlflowservicesetexperimenttag_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicesetexperimenttag_request** | [**MlflowservicesetexperimenttagRequest**](MlflowservicesetexperimenttagRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowservicesettag

> mlflowservicesettag(mlflowservicesettag_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowservicesettag_request** | [**MlflowservicesettagRequest**](MlflowservicesettagRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowserviceupdateexperiment

> mlflowserviceupdateexperiment(mlflowserviceupdateexperiment_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowserviceupdateexperiment_request** | [**MlflowserviceupdateexperimentRequest**](MlflowserviceupdateexperimentRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mlflowserviceupdaterun

> models::Mlflowserviceupdaterun200Response mlflowserviceupdaterun(mlflowserviceupdaterun_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mlflowserviceupdaterun_request** | Option<[**MlflowserviceupdaterunRequest**](MlflowserviceupdaterunRequest.md)> | Request body |  |

### Return type

[**models::Mlflowserviceupdaterun200Response**](mlflowserviceupdaterun_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicecreatemodelversion

> models::Modelregistryservicecreatemodelversion200Response modelregistryservicecreatemodelversion(modelregistryservicecreatemodelversion_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicecreatemodelversion_request** | [**ModelregistryservicecreatemodelversionRequest**](ModelregistryservicecreatemodelversionRequest.md) | Request body | [required] |

### Return type

[**models::Modelregistryservicecreatemodelversion200Response**](modelregistryservicecreatemodelversion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicecreateregisteredmodel

> models::Modelregistryservicecreateregisteredmodel200Response modelregistryservicecreateregisteredmodel(modelregistryservicecreateregisteredmodel_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicecreateregisteredmodel_request** | [**ModelregistryservicecreateregisteredmodelRequest**](ModelregistryservicecreateregisteredmodelRequest.md) | Request body | [required] |

### Return type

[**models::Modelregistryservicecreateregisteredmodel200Response**](modelregistryservicecreateregisteredmodel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicedeletemodelversion

> modelregistryservicedeletemodelversion(name, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model This field is required. | [required] |
**version** | **String** | Model version number This field is required. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicedeletemodelversiontag

> modelregistryservicedeletemodelversiontag(name, version, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model that the tag was logged under. This field is required. | [required] |
**version** | **String** | Model version number that the tag was logged under. This field is required. | [required] |
**key** | **String** | Name of the tag. The name must be an exact match; wild-card deletion is not supported. Maximum size is 250 bytes. This field is required. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicedeleteregisteredmodel

> modelregistryservicedeleteregisteredmodel(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Registered model unique name identifier. This field is required. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicedeleteregisteredmodeltag

> modelregistryservicedeleteregisteredmodeltag(name, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model that the tag was logged under. This field is required. | [required] |
**key** | **String** | Name of the tag. The name must be an exact match; wild-card deletion is not supported. Maximum size is 250 bytes. This field is required. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicegetlatestversions

> models::Modelregistryservicegetlatestversions200Response modelregistryservicegetlatestversions(modelregistryservicegetlatestversions_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicegetlatestversions_request** | [**ModelregistryservicegetlatestversionsRequest**](ModelregistryservicegetlatestversionsRequest.md) | Request body | [required] |

### Return type

[**models::Modelregistryservicegetlatestversions200Response**](modelregistryservicegetlatestversions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicegetmodelversion

> models::Modelregistryservicecreatemodelversion200Response modelregistryservicegetmodelversion(name, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model This field is required. | [required] |
**version** | **String** | Model version number This field is required. | [required] |

### Return type

[**models::Modelregistryservicecreatemodelversion200Response**](modelregistryservicecreatemodelversion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicegetmodelversiondownloaduri

> models::Modelregistryservicegetmodelversiondownloaduri200Response modelregistryservicegetmodelversiondownloaduri(name, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model This field is required. | [required] |
**version** | **String** | Model version number This field is required. | [required] |

### Return type

[**models::Modelregistryservicegetmodelversiondownloaduri200Response**](modelregistryservicegetmodelversiondownloaduri_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicegetregisteredmodel

> models::Modelregistryservicecreateregisteredmodel200Response modelregistryservicegetregisteredmodel(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Registered model unique name identifier. This field is required. | [required] |

### Return type

[**models::Modelregistryservicecreateregisteredmodel200Response**](modelregistryservicecreateregisteredmodel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicerenameregisteredmodel

> models::Modelregistryservicecreateregisteredmodel200Response modelregistryservicerenameregisteredmodel(modelregistryservicerenameregisteredmodel_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicerenameregisteredmodel_request** | [**ModelregistryservicerenameregisteredmodelRequest**](ModelregistryservicerenameregisteredmodelRequest.md) | Request body | [required] |

### Return type

[**models::Modelregistryservicecreateregisteredmodel200Response**](modelregistryservicecreateregisteredmodel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicesearchmodelversions

> models::Modelregistryservicesearchmodelversions200Response modelregistryservicesearchmodelversions(filter, max_results, order_by, page_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | String filter condition, like \"name='my-model-name'\". Must be a single boolean condition, with string values wrapped in single quotes. |  |
**max_results** | Option<**i64**> | Maximum number of models desired. Max threshold is 200K. Backends may choose a lower default value and maximum threshold. |  |
**order_by** | Option<[**Vec<String>**](String.md)> | List of columns to be ordered by including model name, version, stage with an optional \"DESC\" or \"ASC\" annotation, where \"ASC\" is the default. Tiebreaks are done by latest stage transition timestamp, followed by name ASC, followed by version DESC. |  |
**page_token** | Option<**String**> | Pagination token to go to next page based on previous search query. |  |

### Return type

[**models::Modelregistryservicesearchmodelversions200Response**](modelregistryservicesearchmodelversions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicesearchregisteredmodels

> models::Modelregistryservicesearchregisteredmodels200Response modelregistryservicesearchregisteredmodels(filter, max_results, order_by, page_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | String filter condition, like \"name LIKE 'my-model-name'\". Interpreted in the backend automatically as \"name LIKE '%my-model-name%'\". Single boolean condition, with string values wrapped in single quotes. |  |
**max_results** | Option<**i64**> | Maximum number of models desired. Default is 100. Max threshold is 1000. |  |
**order_by** | Option<[**Vec<String>**](String.md)> | List of columns for ordering search results, which can include model name and last updated timestamp with an optional \"DESC\" or \"ASC\" annotation, where \"ASC\" is the default. Tiebreaks are done by model name ASC. |  |
**page_token** | Option<**String**> | Pagination token to go to the next page based on a previous search query. |  |

### Return type

[**models::Modelregistryservicesearchregisteredmodels200Response**](modelregistryservicesearchregisteredmodels_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicesetmodelversiontag

> modelregistryservicesetmodelversiontag(modelregistryservicesetmodelversiontag_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicesetmodelversiontag_request** | [**ModelregistryservicesetmodelversiontagRequest**](ModelregistryservicesetmodelversiontagRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicesetregisteredmodelalias

> modelregistryservicesetregisteredmodelalias(modelregistryservicesetregisteredmodelalias_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicesetregisteredmodelalias_request** | [**ModelregistryservicesetregisteredmodelaliasRequest**](ModelregistryservicesetregisteredmodelaliasRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicesetregisteredmodeltag

> modelregistryservicesetregisteredmodeltag(modelregistryservicesetregisteredmodeltag_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicesetregisteredmodeltag_request** | [**ModelregistryservicesetregisteredmodeltagRequest**](ModelregistryservicesetregisteredmodeltagRequest.md) | Request body | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryservicetransitionmodelversionstage

> models::Modelregistryservicecreatemodelversion200Response modelregistryservicetransitionmodelversionstage(modelregistryservicetransitionmodelversionstage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**modelregistryservicetransitionmodelversionstage_request** | [**ModelregistryservicetransitionmodelversionstageRequest**](ModelregistryservicetransitionmodelversionstageRequest.md) | Request body | [required] |

### Return type

[**models::Modelregistryservicecreatemodelversion200Response**](modelregistryservicecreatemodelversion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryserviceupdatemodelversion

> models::Modelregistryservicecreatemodelversion200Response modelregistryserviceupdatemodelversion(name, version, description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model This field is required. | [required] |
**version** | **String** | Model version number This field is required. | [required] |
**description** | Option<**String**> | If provided, updates the description for this ``registered_model``. |  |

### Return type

[**models::Modelregistryservicecreatemodelversion200Response**](modelregistryservicecreatemodelversion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modelregistryserviceupdateregisteredmodel

> models::Modelregistryservicecreateregisteredmodel200Response modelregistryserviceupdateregisteredmodel(name, description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Registered model unique name identifier. This field is required. | [required] |
**description** | Option<**String**> | If provided, updates the description for this ``registered_model``. |  |

### Return type

[**models::Modelregistryservicecreateregisteredmodel200Response**](modelregistryservicecreateregisteredmodel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

