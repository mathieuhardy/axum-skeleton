# TEST_PLAN: /TC/USERS/GET/ME

# ------------------------------------------------------------------------------

# Get without login
GET http://{{host}}:{{port}}/api/users/current
HTTP 401

# ------------------------------------------------------------------------------

# Get current as admin
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users/current
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$['id']" == "{{admin_id}}"
jsonpath "$['email']" == "{{admin_email}}"

# ------------------------------------------------------------------------------

# Get current as normal
POST http://{{host}}:{{port}}/login
{
    "email": "{{normal_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users/current
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$['id']" == "{{normal_id}}"
jsonpath "$['email']" == "{{normal_email}}"

# ------------------------------------------------------------------------------

# Get current as guest
POST http://{{host}}:{{port}}/login
{
    "email": "{{guest_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users/current
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$['id']" == "{{guest_id}}"
jsonpath "$['email']" == "{{guest_email}}"

# ------------------------------------------------------------------------------

# Get current after logout
POST http://{{host}}:{{port}}/logout
HTTP 200

GET http://{{host}}:{{port}}/api/users/current
HTTP 401
