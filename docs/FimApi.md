# \FimApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fim_completion_v1_fim_completions_post**](FimApi.md#fim_completion_v1_fim_completions_post) | **POST** /v1/fim/completions | Fim Completion



## fim_completion_v1_fim_completions_post

> models::FimCompletionResponse fim_completion_v1_fim_completions_post(fim_completion_request)
Fim Completion

FIM completion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fim_completion_request** | [**FimCompletionRequest**](FimCompletionRequest.md) |  | [required] |

### Return type

[**models::FimCompletionResponse**](FIMCompletionResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

