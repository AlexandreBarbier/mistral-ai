# FtModelOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**object** | Option<**String**> |  | [optional][default to Model]
**created** | **i32** |  | 
**owned_by** | **String** |  | 
**root** | **String** |  | 
**archived** | **bool** |  | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**capabilities** | [**models::FtModelCapabilitiesOut**](FTModelCapabilitiesOut.md) |  | 
**max_context_length** | Option<**i32**> |  | [optional][default to 32768]
**aliases** | Option<**Vec<String>**> |  | [optional][default to []]
**job** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


