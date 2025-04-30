# TEST_PLAN: /TC/USERS/GET/ID

# ------------------------------------------------------------------------------

# Get without login
GET http://{{host}}:{{port}}/api/users/{{admin_id}}
HTTP 401

# ------------------------------------------------------------------------------

# Get user by ID as guest
POST http://{{host}}:{{port}}/login
{
    "email": "{{guest_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users/{{guest_id}}
HTTP 403

# ------------------------------------------------------------------------------

# Get user by ID as normal
POST http://{{host}}:{{port}}/login
{
    "email": "{{normal_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users/{{normal_id}}
HTTP 403

# ------------------------------------------------------------------------------

# Get user by ID as admin
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users/{{admin_id}}
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$['id']" == "{{admin_id}}"
jsonpath "$['first_name']" == "{{admin_first_name}}"
jsonpath "$['last_name']" == "{{admin_last_name}}"
jsonpath "$['email']" == "{{admin_email}}"
jsonpath "$['role']" == "admin"
