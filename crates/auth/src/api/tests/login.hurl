# Invalid email
POST http://{{host}}:{{port}}/login
{
    "email": "",
    "password": "{{auth_pwd}}"
}
HTTP 422

POST http://{{host}}:{{port}}/login
{
    "email": "{{newUuid}}@{{newUuid}}.com",
    "password": "{{auth_pwd}}"
}
HTTP 401

# ------------------------------------------------------------------------------

# Invalid password
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": ""
}
HTTP 422

POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{newUuid}}"
}
HTTP 422

# ------------------------------------------------------------------------------

# Nominal
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200
