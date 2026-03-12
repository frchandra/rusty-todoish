-- create 1 dummy admin user and 1 dummy common user
INSERT INTO users (name, password, email, role) VALUES
    ('admin_user', 'admin_password', 'admin@example.com', 'admin'),
    ('regular_user', 'regular_password', 'regular@example.com', 'common');