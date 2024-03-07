CREATE TYPE status AS ENUM ('Active', 'Inactive', 'Suspended');

CREATE TABLE auth (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(50) UNIQUE NOT NULL,
  password VARCHAR(128) NOT NULL,
  security_level SMALLINT NOT NULL,
  status status DEFAULT 'Active' NOT NULL,
  last_login TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  failed_login_attempts INT DEFAULT 0,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO auth (
  email, 
  username, 
  password, 
  security_level, 
  status, 
  failed_login_attempts
)
VALUES (
  'john.doe@example.com',  -- email
  'john_doe',             -- username
  'hashed_password_here', -- password
  1,                      -- security_level
  'Active',               -- status
  0                      -- failed_login_attempts
);