from openapi_client.paths.user_username.get import ApiForget
from openapi_client.paths.user_username.put import ApiForput
from openapi_client.paths.user_username.delete import ApiFordelete


class UserUsername(
    ApiForget,
    ApiForput,
    ApiFordelete,
):
    pass
