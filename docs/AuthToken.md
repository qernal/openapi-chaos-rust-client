# AuthToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Auth token uuid | 
**user_id** | [**uuid::Uuid**](uuid::Uuid.md) | User | 
**name** | **String** | Name of token | 
**expiry_at** | Option<**String**> | When the token expires | [optional]
**token** | Option<**String**> | Combined token required for requesting an access token, this field is only returned once on creation or update (during regeneration). | [optional]
**date** | [**models::Date**](Date.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


