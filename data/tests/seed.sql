DELETE FROM users WHERE email = ANY(ARRAY['giga@chad.com', 'nor@mal.com', 'gue@st.com']);

INSERT INTO users (id, first_name, last_name, email, role, password)
VALUES
    ('00000000-1111-2222-3333-444444444444', 'Giga', 'Chad', 'giga@chad.com', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$aexquLlpwWwwVO2G0wlLLw$rbl/hO+tDnJwAJDJk5ZjI3kk6pyIIh9m8p2GhAgqaWM'),
    ('00000001-1111-2222-3333-444444444444', 'Nor', 'Mal', 'nor@mal.com', 'normal', '$argon2id$v=19$m=19456,t=2,p=1$aexquLlpwWwwVO2G0wlLLw$rbl/hO+tDnJwAJDJk5ZjI3kk6pyIIh9m8p2GhAgqaWM'),
    ('00000002-1111-2222-3333-444444444444', 'Gue', 'St', 'gue@st.com', 'guest', '$argon2id$v=19$m=19456,t=2,p=1$aexquLlpwWwwVO2G0wlLLw$rbl/hO+tDnJwAJDJk5ZjI3kk6pyIIh9m8p2GhAgqaWM');
