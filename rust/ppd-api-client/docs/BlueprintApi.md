# \BlueprintApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**blueprint_ready**](BlueprintApi.md#blueprint_ready) | **GET** /api/v1/blueprint/{id}/ready | checks whether a blueprint is already ready
[**create**](BlueprintApi.md#create) | **PUT** /api/v1/blueprint/create | create a new coSNARK blueprint
[**issue_cosnark_code**](BlueprintApi.md#issue_cosnark_code) | **GET** /api/v1/blueprint/code | create a new job
[**upload_aux_data**](BlueprintApi.md#upload_aux_data) | **PUT** /api/v1/blueprint/{id}/aux/{aux_type} | add proving key to blueprint



## blueprint_ready

> models::BlueprintReadyProbe blueprint_ready(id)
checks whether a blueprint is already ready

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The id of the blueprint | [required] |

### Return type

[**models::BlueprintReadyProbe**](BlueprintReadyProbe.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create

> models::CreateBlueprintResponse create(create_blueprint_request)
create a new coSNARK blueprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_blueprint_request** | [**CreateBlueprintRequest**](CreateBlueprintRequest.md) |  | [required] |

### Return type

[**models::CreateBlueprintResponse**](CreateBlueprintResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_cosnark_code

> String issue_cosnark_code(issue_co_snark_code_request)
create a new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_co_snark_code_request** | [**IssueCoSnarkCodeRequest**](IssueCoSnarkCodeRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_aux_data

> upload_aux_data(id, aux_type, file)
add proving key to blueprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The id of the blueprint | [required] |
**aux_type** | [**AuxiliaryType**](.md) | The type of the auxiliary data | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

