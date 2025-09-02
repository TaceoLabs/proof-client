# \JobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_inputs**](JobApi.md#add_inputs) | **POST** /api/v1/jobs/add-inputs | add inputs to a existing job
[**download_proof**](JobApi.md#download_proof) | **GET** /api/v1/jobs/{id}/proof | 
[**download_public_inputs**](JobApi.md#download_public_inputs) | **GET** /api/v1/jobs/{id}/public_inputs | 
[**download_signature**](JobApi.md#download_signature) | **GET** /api/v1/jobs/{id}/signature | 
[**schedule_full_job**](JobApi.md#schedule_full_job) | **POST** /api/v1/jobs/schedule-full-job | create a new full job
[**schedule_full_multiple_inputs_job**](JobApi.md#schedule_full_multiple_inputs_job) | **POST** /api/v1/jobs/schedule-full-multiple-inputs-job | create a new full job with multiple inputs
[**schedule_prove_job**](JobApi.md#schedule_prove_job) | **POST** /api/v1/jobs/schedule-prove-job | create a new prove job



## add_inputs

> add_inputs(inputs0, inputs1, inputs2, job_id, node0, node1, node2)
add inputs to a existing job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inputs0** | **std::path::PathBuf** |  | [required] |
**inputs1** | **std::path::PathBuf** |  | [required] |
**inputs2** | **std::path::PathBuf** |  | [required] |
**job_id** | **uuid::Uuid** |  | [required] |
**node0** | **i32** |  | [required] |
**node1** | **i32** |  | [required] |
**node2** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## schedule_full_job

> models::ScheduleJobResponse schedule_full_job(blueprint_id, inputs0, inputs1, inputs2, node0, node1, node2, voucher)
create a new full job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blueprint_id** | **uuid::Uuid** |  | [required] |
**inputs0** | **std::path::PathBuf** |  | [required] |
**inputs1** | **std::path::PathBuf** |  | [required] |
**inputs2** | **std::path::PathBuf** |  | [required] |
**node0** | **i32** |  | [required] |
**node1** | **i32** |  | [required] |
**node2** | **i32** |  | [required] |
**voucher** | Option<**String**> |  |  |

### Return type

[**models::ScheduleJobResponse**](ScheduleJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_full_multiple_inputs_job

> models::ScheduleJobResponse schedule_full_multiple_inputs_job(blueprint_id, node0, node1, node2, deadline, voucher)
create a new full job with multiple inputs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blueprint_id** | **uuid::Uuid** |  | [required] |
**node0** | **i32** |  | [required] |
**node1** | **i32** |  | [required] |
**node2** | **i32** |  | [required] |
**deadline** | Option<**String**> |  |  |
**voucher** | Option<**String**> |  |  |

### Return type

[**models::ScheduleJobResponse**](ScheduleJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_prove_job

> models::ScheduleJobResponse schedule_prove_job(blueprint_id, mpc_protocol, node0, node1, node2, witness0, witness1, witness2, voucher)
create a new prove job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blueprint_id** | **uuid::Uuid** |  | [required] |
**mpc_protocol** | [**models::MpcProtocol**](MpcProtocol.md) |  | [required] |
**node0** | **i32** |  | [required] |
**node1** | **i32** |  | [required] |
**node2** | **i32** |  | [required] |
**witness0** | **std::path::PathBuf** |  | [required] |
**witness1** | **std::path::PathBuf** |  | [required] |
**witness2** | **std::path::PathBuf** |  | [required] |
**voucher** | Option<**String**> |  |  |

### Return type

[**models::ScheduleJobResponse**](ScheduleJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

