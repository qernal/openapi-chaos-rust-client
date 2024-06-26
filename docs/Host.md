# Host

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Host id | 
**host** | **String** | Hostname, this can be the root of a domain or a subdomain | 
**certificate** | Option<**String**> | The secret reference to the certificate | [optional]
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Project ID this is attached to | 
**read_only** | **bool** | If the host is read only and cannot be removed, primarily used for *.qrnl.app domains | 
**disabled** | **bool** | If the host is disabled, then this host won't be accessible and so the deployments will not be routable | 
**txt_verification** | **String** | TXT record of host to verify ownership - if this record is removed, it may become unverified as this is checked periodically to continually verify ownership | 
**verified_at** | Option<**String**> | UTC datetime when the host was verified (ISO 8601 date format). | [optional]
**date** | [**models::Date**](Date.md) |  | 
**verification_status** | [**models::HostVerificationStatus**](HostVerificationStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


