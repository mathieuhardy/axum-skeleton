# ------------------------------------------------------------------------------
# Confirm when already confirmed
# ------------------------------------------------------------------------------

POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

POST http://{{host}}:{{port}}/confirm
[Query]
token: {{newUuid}}
HTTP 404

# ------------------------------------------------------------------------------
# Confirm nominal
# ------------------------------------------------------------------------------

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
user_email: jsonpath "$['email']"
user_confirmation_id: jsonpath "$['pending_confirmation'].id"

POST http://{{host}}:{{port}}/confirm
[Query]
token: {{user_confirmation_id}}
HTTP 200

# ------------------------------------------------------------------------------
# Re-send confirmation
# ------------------------------------------------------------------------------

POST http://{{host}}:{{port}}/login
{
    "email": "{{user_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

POST http://{{host}}:{{port}}/send_confirmation
HTTP 200

GET http://{{host}}:{{port}}/api/users/current
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$['email']" == "{{user_email}}"
jsonpath "$['pending_confirmation']" != null
