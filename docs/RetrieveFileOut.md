# RetrieveFileOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique identifier of the file. | 
**object** | **String** | The object type, which is always \"file\". | 
**bytes** | **i32** | The size of the file, in bytes. | 
**created_at** | **i32** | The UNIX timestamp (in seconds) of the event. | 
**filename** | **String** | The name of the uploaded file. | 
**purpose** | [**models::FilePurpose**](FilePurpose.md) | The intended purpose of the uploaded file. Only accepts fine-tuning (`fine-tune`) for now. | 
**sample_type** | [**models::SampleType**](SampleType.md) |  | 
**num_lines** | Option<**i32**> |  | [optional]
**source** | [**models::Source**](Source.md) |  | 
**deleted** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


