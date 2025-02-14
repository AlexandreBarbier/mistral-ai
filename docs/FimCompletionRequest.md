# FimCompletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> |  | 
**temperature** | Option<**f64**> |  | [optional]
**top_p** | Option<**f64**> | Nucleus sampling, where the model considers the results of the tokens with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. We generally recommend altering this or `temperature` but not both. | [optional][default to 1]
**max_tokens** | Option<**i32**> |  | [optional]
**stream** | Option<**bool**> | Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON. | [optional][default to false]
**stop** | Option<[**models::Stop**](Stop.md)> |  | [optional]
**random_seed** | Option<**i32**> |  | [optional]
**prompt** | **String** | The text/code to complete. | 
**suffix** | Option<**String**> |  | [optional]
**min_tokens** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


