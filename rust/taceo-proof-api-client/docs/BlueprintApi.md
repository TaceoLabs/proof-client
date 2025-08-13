# \BlueprintApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**blueprint_ready**](BlueprintApi.md#blueprint_ready) | **GET** /api/v1/blueprint/{id}/ready | checks whether a blueprint is already ready
[**create**](BlueprintApi.md#create) | **POST** /api/v1/blueprint/create | create a new coSNARK blueprint
[**download_aux_data**](BlueprintApi.md#download_aux_data) | **GET** /api/v1/blueprint/{id}/aux/{aux_type} | 
[**issue_cosnark_code**](BlueprintApi.md#issue_cosnark_code) | **POST** /api/v1/blueprint/code | create a new job
[**revoke**](BlueprintApi.md#revoke) | **POST** /api/v1/blueprint/{id}/revoke | Revokes the blueprint identified by ID if the logged in user has the correct access rights.
[**upload_aux_data**](BlueprintApi.md#upload_aux_data) | **POST** /api/v1/blueprint/{id}/aux/{aux_type} | add proving key to blueprint



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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create

> models::CreateBlueprintResponse create(access, curve, name, typ)
create a new coSNARK blueprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access** | [**models::BlueprintAccess**](BlueprintAccess.md) |  | [required] |
**curve** | [**models::BlueprintCurve**](BlueprintCurve.md) |  | [required] |
**name** | **String** |  | [required] |
**typ** | [**models::BlueprintType**](BlueprintType.md) |  | [required] |

### Return type

[**models::CreateBlueprintResponse**](CreateBlueprintResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_aux_data

> download_aux_data(id, aux_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The id of the blueprint | [required] |
**aux_type** | [**AuxiliaryType**](.md) | The type of the auxiliary data | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_cosnark_code

> String issue_cosnark_code(amount, validity_in_seconds)
create a new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount** | **i32** |  | [required] |
**validity_in_seconds** | **i64** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke

> revoke(id)
Revokes the blueprint identified by ID if the logged in user has the correct access rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The id of the blueprint | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

[cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

