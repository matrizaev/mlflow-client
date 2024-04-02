# MlflowservicesearchrunsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**experiment_ids** | Option<**Vec<String>**> | List of experiment IDs to search over. | [optional]
**filter** | Option<**String**> | A filter expression over params, metrics, and tags, that allows returning a subset of runs. The syntax is a subset of SQL that supports ANDing together binary operations between a param, metric, or tag and a constant.  Example: ``metrics.rmse < 1 and params.model_class = 'LogisticRegression'``  You can select columns with special characters (hyphen, space, period, etc.) by using double quotes: ``metrics.\"model class\" = 'LinearRegression' and tags.\"user-name\" = 'Tomas'``  Supported operators are ``=``, ``!=``, ``>``, ``>=``, ``<``, and ``<=``. | [optional]
**run_view_type** | Option<[**models::Mlflowviewtype**](Mlflowviewtype.md)> |  | [optional]
**max_results** | Option<**i32**> | Maximum number of runs desired. If unspecified, defaults to 1000. All servers are guaranteed to support a `max_results` threshold of at least 50,000 but may support more. Callers of this endpoint are encouraged to pass max_results explicitly and leverage page_token to iterate through experiments. | [optional]
**order_by** | Option<**Vec<String>**> | List of columns to be ordered by, including attributes, params, metrics, and tags with an optional \"DESC\" or \"ASC\" annotation, where \"ASC\" is the default. Example: [\"params.input DESC\", \"metrics.alpha ASC\", \"metrics.rmse\"] Tiebreaks are done by start_time DESC followed by run_id for runs with the same start time (and this is the default ordering criterion if order_by is not provided). | [optional]
**page_token** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


