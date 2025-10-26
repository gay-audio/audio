CREATE TABLE author_projects_relationship(
  author_id UUID REFERENCES authors(id) ON DELETE CASCADE ON UPDATE CASCADE,
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (author_id, project_id)
)
