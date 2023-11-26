import typing_extensions

from openapi_client.apis.tags import TagValues
from openapi_client.apis.tags.pet_api import PetApi
from openapi_client.apis.tags.store_api import StoreApi
from openapi_client.apis.tags.user_api import UserApi

TagToApi = typing_extensions.TypedDict(
    'TagToApi',
    {
        TagValues.PET: PetApi,
        TagValues.STORE: StoreApi,
        TagValues.USER: UserApi,
    }
)

tag_to_api = TagToApi(
    {
        TagValues.PET: PetApi,
        TagValues.STORE: StoreApi,
        TagValues.USER: UserApi,
    }
)
