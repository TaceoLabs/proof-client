# \AdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_node_invite_code**](AdminApi.md#create_node_invite_code) | **POST** /admin/node/invitation/create | 
[**create_node_invite_code_batch**](AdminApi.md#create_node_invite_code_batch) | **POST** /admin/node/invitation/batch_create | 
[**create_user**](AdminApi.md#create_user) | **POST** /admin/user/create | 
[**create_user_invite_code**](AdminApi.md#create_user_invite_code) | **POST** /admin/user/invitation/create | 
[**create_user_invite_code_batch**](AdminApi.md#create_user_invite_code_batch) | **POST** /admin/user/invitation/batch_create | 
[**paginate_open_node_invitations**](AdminApi.md#paginate_open_node_invitations) | **GET** /admin/node/invitation/list | 
[**paginate_open_user_invitations**](AdminApi.md#paginate_open_user_invitations) | **GET** /admin/user/invitation/list | 
[**revoke_node_invitation_code**](AdminApi.md#revoke_node_invitation_code) | **POST** /admin/node/invitation/revoke | 
[**revoke_user_invitation_code**](AdminApi.md#revoke_user_invitation_code) | **POST** /admin/user/invitation/revoke | 
[**update_blueprint_access**](AdminApi.md#update_blueprint_access) | **POST** /admin/blueprint/update/access | update the blueprint access



## create_node_invite_code

> String create_node_invite_code(distribution_method)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**distribution_method** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_node_invite_code_batch

> create_node_invite_code_batch(batch_size, distribution_method)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_size** | **i32** |  | [required] |
**distribution_method** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> create_user(email, password)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**password** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_invite_code

> String create_user_invite_code(distribution_method)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**distribution_method** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_invite_code_batch

> create_user_invite_code_batch(batch_size, distribution_method)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_size** | **i32** |  | [required] |
**distribution_method** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## paginate_open_node_invitations

> models::PaginationResultInviteCode paginate_open_node_invitations(cursor, per_page, filter)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |
**filter** | Option<**String**> |  |  |

### Return type

[**models::PaginationResultInviteCode**](PaginationResult_InviteCode.md)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## paginate_open_user_invitations

> models::PaginationResultInviteCode paginate_open_user_invitations(cursor, per_page, filter)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |
**filter** | Option<**String**> |  |  |

### Return type

[**models::PaginationResultInviteCode**](PaginationResult_InviteCode.md)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_node_invitation_code

> revoke_node_invitation_code(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_invitation_code

> revoke_user_invitation_code(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_blueprint_access

> update_blueprint_access(access, id)
update the blueprint access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access** | [**models::BlueprintAccess**](BlueprintAccess.md) |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[adminAuth](../README.md#adminAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

