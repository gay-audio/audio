CREATE TABLE content_types(
  content_type TEXT PRIMARY KEY
);

ALTER TABLE content ADD CONSTRAINT content_type_restraint FOREIGN KEY (content_type) REFERENCES content_types(content_type) ON UPDATE CASCADE;
