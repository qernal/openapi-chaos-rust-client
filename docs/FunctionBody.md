# FunctionBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the project this function belongs to | 
**version** | **String** | Function spec version | 
**name** | **String** | Name of the function | 
**description** | **String** | Description of what the function does | 
**image** | **String** | Path to container image | 
**r#type** | [**models::FunctionType**](FunctionType.md) |  | 
**size** | [**models::FunctionSize**](FunctionSize.md) |  | 
**port** | **i32** | Port the application runs on | 
**routes** | Option<[**Vec<models::FunctionRoute>**](FunctionRoute.md)> | The public route/path to this function, only applicable to http type functions | [optional]
**scaling** | [**models::FunctionScaling**](FunctionScaling.md) |  | 
**deployments** | [**Vec<models::FunctionDeploymentBody>**](FunctionDeploymentBody.md) | List of deployments for this function | 
**secrets** | [**Vec<models::FunctionEnv>**](FunctionEnv.md) | List of environment variables for secrets | 
**compliance** | Option<[**Vec<models::FunctionCompliance>**](FunctionCompliance.md)> | Tags to limit deployment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


