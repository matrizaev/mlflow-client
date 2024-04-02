# MlflowservicesearchexperimentsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_results** | Option<**i64**> | Maximum number of experiments desired. Servers may select a desired default `max_results` value. All servers are guaranteed to support a `max_results` threshold of at least 1,000 but may support more. Callers of this endpoint are encouraged to pass max_results explicitly and leverage page_token to iterate through experiments. | [optional]
**page_token** | Option<**String**> | Token indicating the page of experiments to fetch | [optional]
**filter** | Option<**String**> | A filter expression over experiment attributes and tags that allows returning a subset of experiments. The syntax is a subset of SQL that supports ANDing together binary operations between an attribute or tag, and a constant.  Example: ``name LIKE 'test-%' AND tags.key = 'value'``  You can select columns with special characters (hyphen, space, period, etc.) by using double quotes or backticks.  Example: ``tags.\"extra-key\" = 'value'`` or ``tags.`extra-key` = 'value'``  Supported operators are ``=``, ``!=``, ``LIKE``, and ``ILIKE``. | [optional]
**order_by** | Option<**Vec<String>**> | List of columns for ordering search results, which can include experiment name and id with an optional \"DESC\" or \"ASC\" annotation, where \"ASC\" is the default. Tiebreaks are done by experiment id DESC. | [optional]
**view_type** | Option<[**models::Mlflowviewtype**](Mlflowviewtype.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


