# AuthToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**user_id** | [**uuid::Uuid**](uuid::Uuid.md) | User | 
**name** | **String** | Token name | 
**expiry_at** | Option<**String**> |  | [optional]
**token** | Option<**String**> | OAuth2 client id and client secret used to generate API access token. Client secret can't be created and must be saved on user side | [optional]
**date** | [**crate::models::Date**](Date.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


