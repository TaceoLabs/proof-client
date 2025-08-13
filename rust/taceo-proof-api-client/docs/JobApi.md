# \JobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_proof**](JobApi.md#download_proof) | **GET** /api/v1/jobs/{id}/proof | 
[**download_public_inputs**](JobApi.md#download_public_inputs) | **GET** /api/v1/jobs/{id}/public_inputs | 
[**download_signature**](JobApi.md#download_signature) | **GET** /api/v1/jobs/{id}/signature | 
[**schedule_job**](JobApi.md#schedule_job) | **POST** /api/v1/jobs/schedule | create a new job



## download_proof

> download_proof(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_public_inputs

> download_public_inputs(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_signature

> download_signature(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_job

> models::ScheduleJobResponse schedule_job(a_blueprint_id, b_job_type, c_node0, c_node1, c_node2, input_party0, input_party1, input_party2, d_code)
create a new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a_blueprint_id** | **uuid::Uuid** |  | [required] |
**b_job_type** | [**models::JobType**](JobType.md) |  | [required] |
**c_node0** | **i32** |  | [required] |
**c_node1** | **i32** |  | [required] |
**c_node2** | **i32** |  | [required] |
**input_party0** | **std::path::PathBuf** |  | [required] |
**input_party1** | **std::path::PathBuf** |  | [required] |
**input_party2** | **std::path::PathBuf** |  | [required] |
**d_code** | Option<**String**> |  |  |

### Return type

[**models::ScheduleJobResponse**](ScheduleJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

