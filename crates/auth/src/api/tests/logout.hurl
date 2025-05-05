# Logout without login
POST http://{{host}}:{{port}}/logout
HTTP 200

# ------------------------------------------------------------------------------

# Logout after login
POST http://{{host}}:{{port}}/login
{
    "email": "{{admin_email}}",
    "password": "{{auth_pwd}}"
}
HTTP 200

POST http://{{host}}:{{port}}/logout
HTTP 200
