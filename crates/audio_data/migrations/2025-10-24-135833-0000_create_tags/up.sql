CREATE TABLE tags(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  display TEXT NOT NULL
)
