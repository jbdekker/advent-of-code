<a name="__pageTop"></a>
# openapi_client.apis.tags.pet_api.PetApi

All URIs are relative to *http://petstore.swagger.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_pet**](#add_pet) | **post** /pet | Add a new pet to the store
[**delete_pet**](#delete_pet) | **delete** /pet/{petId} | Deletes a pet
[**find_pets_by_status**](#find_pets_by_status) | **get** /pet/findByStatus | Finds Pets by status
[**find_pets_by_tags**](#find_pets_by_tags) | **get** /pet/findByTags | Finds Pets by tags
[**get_pet_by_id**](#get_pet_by_id) | **get** /pet/{petId} | Find pet by ID
[**update_pet**](#update_pet) | **put** /pet | Update an existing pet
[**update_pet_with_form**](#update_pet_with_form) | **post** /pet/{petId} | Updates a pet in the store with form data
[**upload_file**](#upload_file) | **post** /pet/{petId}/uploadImage | uploads an image

# **add_pet**
<a name="add_pet"></a>
> Pet add_pet(pet)

Add a new pet to the store

### Example

* OAuth Authentication (petstore_auth):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from openapi_client.model.pet import Pet
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure OAuth2 access token for authorization: petstore_auth
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2",
    access_token = 'YOUR_ACCESS_TOKEN'
)
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    body = Pet(
        id=1,
        category=Category(
            id=1,
            name="CbUUGjjNSwg0_bs9ZayIMrKdgNvb6gvxmPb9GcsM61ate1RA89q3w1l4eH4XxEz.5awLMdeXylwK0lMGUSM4jsrh4dstlnQUN5vVdMLPA",
        ),
        name="doggie",
        photo_urls=[
            "photo_urls_example"
        ],
        tags=[
            Tag(
                id=1,
                name="name_example",
            )
        ],
        status="available",
    )
    try:
        # Add a new pet to the store
        api_response = api_instance.add_pet(
            body=body,
        )
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->add_pet: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyApplicationJson, SchemaForRequestBodyApplicationXml] | required |
content_type | str | optional, default is 'application/json' | Selects the schema and serialization of the request body
accept_content_types | typing.Tuple[str] | default is ('application/xml', 'application/json', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

# SchemaForRequestBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


# SchemaForRequestBodyApplicationXml
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | [ApiResponseFor200](#add_pet.ApiResponseFor200) | successful operation
405 | [ApiResponseFor405](#add_pet.ApiResponseFor405) | Invalid input

#### add_pet.ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationXml, SchemaFor200ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

# SchemaFor200ResponseBodyApplicationXml
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


# SchemaFor200ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


#### add_pet.ApiResponseFor405
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

### Authorization

[petstore_auth](../../../README.md#petstore_auth)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

# **delete_pet**
<a name="delete_pet"></a>
> delete_pet(pet_id)

Deletes a pet

### Example

* OAuth Authentication (petstore_auth):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure OAuth2 access token for authorization: petstore_auth
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2",
    access_token = 'YOUR_ACCESS_TOKEN'
)
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'petId': 1,
    }
    header_params = {
    }
    try:
        # Deletes a pet
        api_response = api_instance.delete_pet(
            path_params=path_params,
            header_params=header_params,
        )
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->delete_pet: %s\n" % e)

    # example passing only optional values
    path_params = {
        'petId': 1,
    }
    header_params = {
        'api_key': "api_key_example",
    }
    try:
        # Deletes a pet
        api_response = api_instance.delete_pet(
            path_params=path_params,
            header_params=header_params,
        )
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->delete_pet: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
header_params | RequestHeaderParams | |
path_params | RequestPathParams | |
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### header_params
#### RequestHeaderParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
api_key | ApiKeySchema | | optional

# ApiKeySchema

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
str,  | str,  |  | 

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
petId | PetIdSchema | | 

# PetIdSchema

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
decimal.Decimal, int,  | decimal.Decimal,  |  | value must be a 64 bit integer

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
400 | [ApiResponseFor400](#delete_pet.ApiResponseFor400) | Invalid pet value

#### delete_pet.ApiResponseFor400
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

### Authorization

[petstore_auth](../../../README.md#petstore_auth)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

# **find_pets_by_status**
<a name="find_pets_by_status"></a>
> [Pet] find_pets_by_status(status)

Finds Pets by status

Multiple status values can be provided with comma separated strings

### Example

* OAuth Authentication (petstore_auth):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from openapi_client.model.pet import Pet
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure OAuth2 access token for authorization: petstore_auth
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2",
    access_token = 'YOUR_ACCESS_TOKEN'
)
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    query_params = {
        'status': [
        "available"
    ],
    }
    try:
        # Finds Pets by status
        api_response = api_instance.find_pets_by_status(
            query_params=query_params,
        )
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->find_pets_by_status: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
accept_content_types | typing.Tuple[str] | default is ('application/xml', 'application/json', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### query_params
#### RequestQueryParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
status | StatusSchema | | 


# StatusSchema

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
list, tuple,  | tuple,  |  | 

### Tuple Items
Class Name | Input Type | Accessed Type | Description | Notes
------------- | ------------- | ------------- | ------------- | -------------
items | str,  | str,  |  | must be one of ["available", "pending", "sold", ] if omitted the server will use the default value of "available"

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | [ApiResponseFor200](#find_pets_by_status.ApiResponseFor200) | successful operation
400 | [ApiResponseFor400](#find_pets_by_status.ApiResponseFor400) | Invalid status value

#### find_pets_by_status.ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationXml, SchemaFor200ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

# SchemaFor200ResponseBodyApplicationXml

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
list, tuple,  | tuple,  |  | 

### Tuple Items
Class Name | Input Type | Accessed Type | Description | Notes
------------- | ------------- | ------------- | ------------- | -------------
[**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) |  | 

# SchemaFor200ResponseBodyApplicationJson

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
list, tuple,  | tuple,  |  | 

### Tuple Items
Class Name | Input Type | Accessed Type | Description | Notes
------------- | ------------- | ------------- | ------------- | -------------
[**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) |  | 

#### find_pets_by_status.ApiResponseFor400
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

### Authorization

[petstore_auth](../../../README.md#petstore_auth)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

# **find_pets_by_tags**
<a name="find_pets_by_tags"></a>
> [Pet] find_pets_by_tags(tags)

Finds Pets by tags

Multiple tags can be provided with comma separated strings. Use tag1, tag2, tag3 for testing.

### Example

* OAuth Authentication (petstore_auth):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from openapi_client.model.pet import Pet
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure OAuth2 access token for authorization: petstore_auth
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2",
    access_token = 'YOUR_ACCESS_TOKEN'
)
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    query_params = {
        'tags': [
        "tags_example"
    ],
    }
    try:
        # Finds Pets by tags
        api_response = api_instance.find_pets_by_tags(
            query_params=query_params,
        )
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->find_pets_by_tags: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
accept_content_types | typing.Tuple[str] | default is ('application/xml', 'application/json', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### query_params
#### RequestQueryParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
tags | TagsSchema | | 


# TagsSchema

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
list, tuple,  | tuple,  |  | 

### Tuple Items
Class Name | Input Type | Accessed Type | Description | Notes
------------- | ------------- | ------------- | ------------- | -------------
items | str,  | str,  |  | 

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | [ApiResponseFor200](#find_pets_by_tags.ApiResponseFor200) | successful operation
400 | [ApiResponseFor400](#find_pets_by_tags.ApiResponseFor400) | Invalid tag value

#### find_pets_by_tags.ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationXml, SchemaFor200ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

# SchemaFor200ResponseBodyApplicationXml

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
list, tuple,  | tuple,  |  | 

### Tuple Items
Class Name | Input Type | Accessed Type | Description | Notes
------------- | ------------- | ------------- | ------------- | -------------
[**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) |  | 

# SchemaFor200ResponseBodyApplicationJson

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
list, tuple,  | tuple,  |  | 

### Tuple Items
Class Name | Input Type | Accessed Type | Description | Notes
------------- | ------------- | ------------- | ------------- | -------------
[**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) | [**Pet**]({{complexTypePrefix}}Pet.md) |  | 

#### find_pets_by_tags.ApiResponseFor400
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

### Authorization

[petstore_auth](../../../README.md#petstore_auth)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

# **get_pet_by_id**
<a name="get_pet_by_id"></a>
> Pet get_pet_by_id(pet_id)

Find pet by ID

Returns a single pet

### Example

* Api Key Authentication (api_key):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from openapi_client.model.pet import Pet
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = 'YOUR_API_KEY'

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'petId': 1,
    }
    try:
        # Find pet by ID
        api_response = api_instance.get_pet_by_id(
            path_params=path_params,
        )
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->get_pet_by_id: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/xml', 'application/json', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
petId | PetIdSchema | | 

# PetIdSchema

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
decimal.Decimal, int,  | decimal.Decimal,  |  | value must be a 64 bit integer

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | [ApiResponseFor200](#get_pet_by_id.ApiResponseFor200) | successful operation
400 | [ApiResponseFor400](#get_pet_by_id.ApiResponseFor400) | Invalid ID supplied
404 | [ApiResponseFor404](#get_pet_by_id.ApiResponseFor404) | Pet not found

#### get_pet_by_id.ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationXml, SchemaFor200ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

# SchemaFor200ResponseBodyApplicationXml
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


# SchemaFor200ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


#### get_pet_by_id.ApiResponseFor400
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

#### get_pet_by_id.ApiResponseFor404
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

### Authorization

[api_key](../../../README.md#api_key)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

# **update_pet**
<a name="update_pet"></a>
> Pet update_pet(pet)

Update an existing pet

### Example

* OAuth Authentication (petstore_auth):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from openapi_client.model.pet import Pet
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure OAuth2 access token for authorization: petstore_auth
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2",
    access_token = 'YOUR_ACCESS_TOKEN'
)
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    body = Pet(
        id=1,
        category=Category(
            id=1,
            name="CbUUGjjNSwg0_bs9ZayIMrKdgNvb6gvxmPb9GcsM61ate1RA89q3w1l4eH4XxEz.5awLMdeXylwK0lMGUSM4jsrh4dstlnQUN5vVdMLPA",
        ),
        name="doggie",
        photo_urls=[
            "photo_urls_example"
        ],
        tags=[
            Tag(
                id=1,
                name="name_example",
            )
        ],
        status="available",
    )
    try:
        # Update an existing pet
        api_response = api_instance.update_pet(
            body=body,
        )
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->update_pet: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyApplicationJson, SchemaForRequestBodyApplicationXml] | required |
content_type | str | optional, default is 'application/json' | Selects the schema and serialization of the request body
accept_content_types | typing.Tuple[str] | default is ('application/xml', 'application/json', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

# SchemaForRequestBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


# SchemaForRequestBodyApplicationXml
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | [ApiResponseFor200](#update_pet.ApiResponseFor200) | successful operation
400 | [ApiResponseFor400](#update_pet.ApiResponseFor400) | Invalid ID supplied
404 | [ApiResponseFor404](#update_pet.ApiResponseFor404) | Pet not found
405 | [ApiResponseFor405](#update_pet.ApiResponseFor405) | Validation exception

#### update_pet.ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationXml, SchemaFor200ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

# SchemaFor200ResponseBodyApplicationXml
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


# SchemaFor200ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**Pet**](../../models/Pet.md) |  | 


#### update_pet.ApiResponseFor400
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

#### update_pet.ApiResponseFor404
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

#### update_pet.ApiResponseFor405
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

### Authorization

[petstore_auth](../../../README.md#petstore_auth)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

# **update_pet_with_form**
<a name="update_pet_with_form"></a>
> update_pet_with_form(pet_id)

Updates a pet in the store with form data

### Example

* OAuth Authentication (petstore_auth):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure OAuth2 access token for authorization: petstore_auth
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2",
    access_token = 'YOUR_ACCESS_TOKEN'
)
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'petId': 1,
    }
    try:
        # Updates a pet in the store with form data
        api_response = api_instance.update_pet_with_form(
            path_params=path_params,
        )
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->update_pet_with_form: %s\n" % e)

    # example passing only optional values
    path_params = {
        'petId': 1,
    }
    body = dict(
        name="name_example",
        status="status_example",
    )
    try:
        # Updates a pet in the store with form data
        api_response = api_instance.update_pet_with_form(
            path_params=path_params,
            body=body,
        )
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->update_pet_with_form: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyApplicationXWwwFormUrlencoded, Unset] | optional, default is unset |
path_params | RequestPathParams | |
content_type | str | optional, default is 'application/x-www-form-urlencoded' | Selects the schema and serialization of the request body
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

# SchemaForRequestBodyApplicationXWwwFormUrlencoded

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
dict, frozendict.frozendict,  | frozendict.frozendict,  |  | 

### Dictionary Keys
Key | Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | ------------- | -------------
**name** | str,  | str,  | Updated name of the pet | [optional] 
**status** | str,  | str,  | Updated status of the pet | [optional] 
**any_string_name** | dict, frozendict.frozendict, str, date, datetime, int, float, bool, decimal.Decimal, None, list, tuple, bytes, io.FileIO, io.BufferedReader | frozendict.frozendict, str, BoolClass, decimal.Decimal, NoneClass, tuple, bytes, FileIO | any string name can be used but the value must be the correct type | [optional]

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
petId | PetIdSchema | | 

# PetIdSchema

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
decimal.Decimal, int,  | decimal.Decimal,  |  | value must be a 64 bit integer

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
405 | [ApiResponseFor405](#update_pet_with_form.ApiResponseFor405) | Invalid input

#### update_pet_with_form.ApiResponseFor405
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | Unset | body was not defined |
headers | Unset | headers were not defined |

### Authorization

[petstore_auth](../../../README.md#petstore_auth)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

# **upload_file**
<a name="upload_file"></a>
> ApiResponse upload_file(pet_id)

uploads an image

### Example

* OAuth Authentication (petstore_auth):
```python
import openapi_client
from openapi_client.apis.tags import pet_api
from openapi_client.model.api_response import ApiResponse
from pprint import pprint
# Defining the host is optional and defaults to http://petstore.swagger.io/v2
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure OAuth2 access token for authorization: petstore_auth
configuration = openapi_client.Configuration(
    host = "http://petstore.swagger.io/v2",
    access_token = 'YOUR_ACCESS_TOKEN'
)
# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = pet_api.PetApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'petId': 1,
    }
    try:
        # uploads an image
        api_response = api_instance.upload_file(
            path_params=path_params,
        )
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->upload_file: %s\n" % e)

    # example passing only optional values
    path_params = {
        'petId': 1,
    }
    body = dict(
        additional_metadata="additional_metadata_example",
        file=open('/path/to/file', 'rb'),
    )
    try:
        # uploads an image
        api_response = api_instance.upload_file(
            path_params=path_params,
            body=body,
        )
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling PetApi->upload_file: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyMultipartFormData, Unset] | optional, default is unset |
path_params | RequestPathParams | |
content_type | str | optional, default is 'multipart/form-data' | Selects the schema and serialization of the request body
accept_content_types | typing.Tuple[str] | default is ('application/json', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

# SchemaForRequestBodyMultipartFormData

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
dict, frozendict.frozendict,  | frozendict.frozendict,  |  | 

### Dictionary Keys
Key | Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | ------------- | -------------
**additionalMetadata** | str,  | str,  | Additional data to pass to server | [optional] 
**file** | bytes, io.FileIO, io.BufferedReader,  | bytes, FileIO,  | file to upload | [optional] 
**any_string_name** | dict, frozendict.frozendict, str, date, datetime, int, float, bool, decimal.Decimal, None, list, tuple, bytes, io.FileIO, io.BufferedReader | frozendict.frozendict, str, BoolClass, decimal.Decimal, NoneClass, tuple, bytes, FileIO | any string name can be used but the value must be the correct type | [optional]

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
petId | PetIdSchema | | 

# PetIdSchema

## Model Type Info
Input Type | Accessed Type | Description | Notes
------------ | ------------- | ------------- | -------------
decimal.Decimal, int,  | decimal.Decimal,  |  | value must be a 64 bit integer

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | [ApiResponseFor200](#upload_file.ApiResponseFor200) | successful operation

#### upload_file.ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

# SchemaFor200ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**ApiResponse**](../../models/ApiResponse.md) |  | 


### Authorization

[petstore_auth](../../../README.md#petstore_auth)

[[Back to top]](#__pageTop) [[Back to API list]](../../../README.md#documentation-for-api-endpoints) [[Back to Model list]](../../../README.md#documentation-for-models) [[Back to README]](../../../README.md)

