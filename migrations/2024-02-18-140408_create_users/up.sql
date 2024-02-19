CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  is_admin BOOLEAN NOT NULL DEFAULT FALSE,
  first_name VARCHAR(50),
  last_name VARCHAR(50),
  username VARCHAR(50) NOT NULL,
  mobile VARCHAR(15),
  email VARCHAR(50),
  password_hash VARCHAR(32) NOT NULL,
  registered_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_login TIMESTAMP,
  intro TEXT,
  CONSTRAINT uq_username UNIQUE (username),
  CONSTRAINT uq_mobile UNIQUE (mobile),
  CONSTRAINT uq_email UNIQUE (email)
);

