# \NodeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list**](NodeApi.md#list) | **GET** /api/v1/node/list | get node providers
[**node_provider**](NodeApi.md#node_provider) | **GET** /api/v1/node/{id} | returns the node for the given id
[**random_node_providers**](NodeApi.md#random_node_providers) | **GET** /api/v1/node/random-nodes | returns 3 randomly chosen node providers



## list

> Vec<models::NodeProvider> list(cursor, per_page)
get node providers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<models::NodeProvider>**](NodeProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## node_provider

> models::NodeProvider node_provider(id)
returns the node for the given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::NodeProvider**](NodeProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## random_node_providers

> models::NodeProviders random_node_providers()
returns 3 randomly chosen node providers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NodeProviders**](NodeProviders.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

