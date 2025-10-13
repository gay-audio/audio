CREATE TABLE content (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(256) NOT NULL,
  content_type VARCHAR(256) NOT NULL
);
