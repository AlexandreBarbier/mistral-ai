# BatchJobOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**object** | Option<**String**> |  | [optional][default to Batch]
**input_files** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | 
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**endpoint** | **String** |  | 
**model** | **String** |  | 
**output_file** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**error_file** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**errors** | [**Vec<models::BatchError>**](BatchError.md) |  | 
**status** | [**models::BatchJobStatus**](BatchJobStatus.md) |  | 
**created_at** | **i32** |  | 
**total_requests** | **i32** |  | 
**completed_requests** | **i32** |  | 
**succeeded_requests** | **i32** |  | 
**failed_requests** | **i32** |  | 
**started_at** | Option<**i32**> |  | [optional]
**completed_at** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


