# TEST_PLAN: /TC/USERS/GET/ALL
# TEST_PLAN: /TC/USERS/GET/FILTERED

# ------------------------------------------------------------------------------

# Get without login
GET http://{{host}}:{{port}}/api/users
HTTP 401

# ------------------------------------------------------------------------------

# Get all as guest
POST http://{{host}}:{{port}}/login
{
    "email": "{{guest_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users
HTTP 403

# ------------------------------------------------------------------------------

# Get all as normal
POST http://{{host}}:{{port}}/login
{
    "email": "{{normal_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users
HTTP 403

# ------------------------------------------------------------------------------

# Get all as admin
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

GET http://{{host}}:{{port}}/api/users
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" contains "{{admin_email}}"
jsonpath "$[*].email" contains "{{normal_email}}"
jsonpath "$[*].email" contains "{{guest_email}}"

# ------------------------------------------------------------------------------

# Filtered by first name
GET http://{{host}}:{{port}}/api/users
[Query]
first_name: NOT_FOUND
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" count == 0

GET http://{{host}}:{{port}}/api/users
[Query]
first_name: {{admin_first_name}}
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" count == 1
jsonpath "$[*].email" contains "{{admin_email}}"

# ------------------------------------------------------------------------------

# Filtered by last name
GET http://{{host}}:{{port}}/api/users
[Query]
last_name: NOT_FOUND
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" count == 0

GET http://{{host}}:{{port}}/api/users
[Query]
last_name: {{admin_last_name}}
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" count == 1
jsonpath "$[*].email" contains "{{admin_email}}"

# ------------------------------------------------------------------------------

# Filtered by email
GET http://{{host}}:{{port}}/api/users
[Query]
email: NOT_FOUND
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" count == 0

GET http://{{host}}:{{port}}/api/users
[Query]
email: {{admin_email}}
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" count == 1
jsonpath "$[*].email" contains "{{admin_email}}"

# ------------------------------------------------------------------------------

# Filtered by role
GET http://{{host}}:{{port}}/api/users
[Query]
role: NOT_FOUND
HTTP 400

GET http://{{host}}:{{port}}/api/users
[Query]
role: admin
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$[*].email" count >= 1
jsonpath "$[*].email" contains "{{admin_email}}"
