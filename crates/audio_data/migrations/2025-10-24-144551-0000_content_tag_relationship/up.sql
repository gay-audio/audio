CREATE TABLE content_tag_relationship(
  content_id UUID REFERENCES content(id),
  tag_id UUID REFERENCES tags(id),
  PRIMARY KEY (content_id, tag_id)
)
