# \AgentsApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**agents_completion_v1_agents_completions_post**](AgentsApi.md#agents_completion_v1_agents_completions_post) | **POST** /v1/agents/completions | Agents Completion



## agents_completion_v1_agents_completions_post

> models::ChatCompletionResponse agents_completion_v1_agents_completions_post(agents_completion_request)
Agents Completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agents_completion_request** | [**AgentsCompletionRequest**](AgentsCompletionRequest.md) |  | [required] |

### Return type

[**models::ChatCompletionResponse**](ChatCompletionResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

