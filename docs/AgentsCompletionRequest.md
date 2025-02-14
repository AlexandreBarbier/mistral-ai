# AgentsCompletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_tokens** | Option<**i32**> |  | [optional]
**stream** | Option<**bool**> | Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON. | [optional][default to false]
**stop** | Option<[**models::Stop**](Stop.md)> |  | [optional]
**random_seed** | Option<**i32**> |  | [optional]
**messages** | [**Vec<models::ChatCompletionRequestMessagesInner>**](ChatCompletionRequest_messages_inner.md) | The prompt(s) to generate completions for, encoded as a list of dict with role and content. | 
**response_format** | Option<[**models::ResponseFormat**](ResponseFormat.md)> |  | [optional]
**tools** | Option<[**Vec<models::Tool>**](Tool.md)> |  | [optional]
**tool_choice** | Option<[**models::ToolChoice**](Tool_Choice.md)> |  | [optional]
**presence_penalty** | Option<**f64**> | presence_penalty determines how much the model penalizes the repetition of words or phrases. A higher presence penalty encourages the model to use a wider variety of words and phrases, making the output more diverse and creative. | [optional][default to 0]
**frequency_penalty** | Option<**f64**> | frequency_penalty penalizes the repetition of words based on their frequency in the generated text. A higher frequency penalty discourages the model from repeating words that have already appeared frequently in the output, promoting diversity and reducing repetition. | [optional][default to 0]
**n** | Option<**i32**> |  | [optional]
**prediction** | Option<[**models::Prediction**](Prediction.md)> | Enable users to specify expected results, optimizing response times by leveraging known or predictable content. This approach is especially effective for updating text documents or code files with minimal changes, reducing latency while maintaining high-quality results. | [optional][default to {type=content, content=}]
**agent_id** | **String** | The ID of the agent to use for this completion. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


