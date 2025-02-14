# \FilesApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_api_routes_delete_file**](FilesApi.md#files_api_routes_delete_file) | **DELETE** /v1/files/{file_id} | Delete File
[**files_api_routes_download_file**](FilesApi.md#files_api_routes_download_file) | **GET** /v1/files/{file_id}/content | Download File
[**files_api_routes_get_signed_url**](FilesApi.md#files_api_routes_get_signed_url) | **GET** /v1/files/{file_id}/url | Get Signed Url
[**files_api_routes_list_files**](FilesApi.md#files_api_routes_list_files) | **GET** /v1/files | List Files
[**files_api_routes_retrieve_file**](FilesApi.md#files_api_routes_retrieve_file) | **GET** /v1/files/{file_id} | Retrieve File
[**files_api_routes_upload_file**](FilesApi.md#files_api_routes_upload_file) | **POST** /v1/files | Upload File



## files_api_routes_delete_file

> models::DeleteFileOut files_api_routes_delete_file(file_id)
Delete File

Delete a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** |  | [required] |

### Return type

[**models::DeleteFileOut**](DeleteFileOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_download_file

> std::path::PathBuf files_api_routes_download_file(file_id)
Download File

Download a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_get_signed_url

> models::FileSignedUrl files_api_routes_get_signed_url(file_id, expiry)
Get Signed Url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** |  | [required] |
**expiry** | Option<**i32**> | Number of hours before the url becomes invalid. Defaults to 24h |  |[default to 24]

### Return type

[**models::FileSignedUrl**](FileSignedURL.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_list_files

> models::ListFilesOut files_api_routes_list_files(page, page_size, sample_type, source, search, purpose)
List Files

Returns a list of files that belong to the user's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 0]
**page_size** | Option<**i32**> |  |  |[default to 100]
**sample_type** | Option<[**Vec<models::SampleType>**](models::SampleType.md)> |  |  |
**source** | Option<[**Vec<models::Source>**](models::Source.md)> |  |  |
**search** | Option<**String**> |  |  |
**purpose** | Option<[**FilePurpose**](.md)> |  |  |

### Return type

[**models::ListFilesOut**](ListFilesOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_retrieve_file

> models::RetrieveFileOut files_api_routes_retrieve_file(file_id)
Retrieve File

Returns information about a specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** |  | [required] |

### Return type

[**models::RetrieveFileOut**](RetrieveFileOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_upload_file

> models::UploadFileOut files_api_routes_upload_file(file, purpose)
Upload File

Upload a file that can be used across various endpoints.  The size of individual files can be a maximum of 512 MB. The Fine-tuning API only supports .jsonl files.  Please contact us if you need to increase these storage limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | The File object (not file name) to be uploaded.  To upload a file and specify a custom file name you should format your request as such:  ```bash  file=@path/to/your/file.jsonl;filename=custom_name.jsonl  ```  Otherwise, you can just keep the original file name:  ```bash  file=@path/to/your/file.jsonl  ``` | [required] |
**purpose** | Option<[**models::FilePurpose**](FilePurpose.md)> |  |  |

### Return type

[**models::UploadFileOut**](UploadFileOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

