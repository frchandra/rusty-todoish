-- create 1 dummy admin user and 1 dummy non addmin user
INSERT INTO users (name, password, email, is_admin) VALUES
    ('admin_user', 'admin_password', 'admin@example.com', TRUE),
    ('regular_user', 'regular_password', 'regular@example.com', FALSE);