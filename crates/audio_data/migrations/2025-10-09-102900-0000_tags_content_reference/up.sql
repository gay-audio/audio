CREATE TABLE content_tags_reference (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  tag_name VARCHAR(256) REFERENCES tags(name) NOT NULL,
  content_id UUID REFERENCES authors(id) NOT NULL
)
