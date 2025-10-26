CREATE TABLE content_mime_types(
  content_type TEXT REFERENCES content_types(content_type) ON DELETE CASCADE ON UPDATE CASCADE,
  mime_type TEXT NOT NULL,
  PRIMARY KEY (content_type, mime_type)
)

