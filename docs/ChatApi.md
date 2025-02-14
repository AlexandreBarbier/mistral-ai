# \ChatApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_completion_v1_chat_completions_post**](ChatApi.md#chat_completion_v1_chat_completions_post) | **POST** /v1/chat/completions | Chat Completion



## chat_completion_v1_chat_completions_post

> models::ChatCompletionResponse chat_completion_v1_chat_completions_post(chat_completion_request)
Chat Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_completion_request** | [**ChatCompletionRequest**](ChatCompletionRequest.md) |  | [required] |

### Return type

[**models::ChatCompletionResponse**](ChatCompletionResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

