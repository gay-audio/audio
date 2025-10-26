ALTER TABLE content ADD CONSTRAINT project_content_relationship FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE ON UPDATE CASCADE;

