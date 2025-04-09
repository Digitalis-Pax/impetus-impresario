# Permission Model

II uses a Role Based Access Control model. Each User is assigned to zero or more roles, and each role is assigned zero or more Grants to resources. The effective permissions for the User are the combination of all Grants from all Roles to which they are assigned, with _Deny_ grants taking precedence over _Allow_ grants. This means that if a User is assigned to two Roles, one with an _Allow_ grant and the other with a _Deny_ grant for the same resource, the _Deny_ grant will take precedence and the User will not have access to that resource.

## Grants

There are three types of Grant:

- **Inherit**: The User inherits the permissions of the parent Role, if one exists, or _Deny_ otherwise.
- **Allow**: The User is allowed to perform the action on the resource.
- **Deny**: The User is denied permission to perform the action on the resource.
