# Create a user to be deleted
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

POST http://{{host}}:{{port}}/api/users
{
    "first_name": "{{newUuid}}",
    "last_name": "{{newUuid}}",
    "email": "{{newUuid}}@{{newUuid}}.com",
    "role": "guest",
    "password": "{{auth_pwd}}"
}
HTTP 201
[Asserts]
header "Content-Type" == "application/json"
[Captures]
user_id: jsonpath "$['id']"

POST http://{{host}}:{{port}}/logout
HTTP 200

# ------------------------------------------------------------------------------

# Delete without login
DELETE http://{{host}}:{{port}}/api/users/{{user_id}}
HTTP 401

# ------------------------------------------------------------------------------

# Try to delete as guest
POST http://{{host}}:{{port}}/login
{
    "email": "{{guest_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

DELETE http://{{host}}:{{port}}/api/users/{{user_id}}
HTTP 403

# ------------------------------------------------------------------------------

# Try to delete as normal
POST http://{{host}}:{{port}}/login
{
    "email": "{{normal_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

DELETE http://{{host}}:{{port}}/api/users/{{user_id}}
HTTP 403

# ------------------------------------------------------------------------------

# Try to delete as admin
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

DELETE http://{{host}}:{{port}}/api/users/{{user_id}}
HTTP 204
