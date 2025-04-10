# \CoSnarkBlueprintApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create**](CoSnarkBlueprintApi.md#create) | **PUT** /api/v1/blueprint/create | create a new coSNARK blueprint
[**upload_circuit**](CoSnarkBlueprintApi.md#upload_circuit) | **PUT** /api/v1/blueprint/{id}/circuit | add proving key to blueprint
[**upload_pk**](CoSnarkBlueprintApi.md#upload_pk) | **PUT** /api/v1/blueprint/{id}/pk | add proving key to blueprint



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


## upload_circuit

> upload_circuit(id, file)
add proving key to blueprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the blueprint | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_pk

> upload_pk(id, file)
add proving key to blueprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the blueprint | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

