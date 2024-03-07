CREATE TYPE assoc_table AS ENUM (
  'blog'
);

CREATE TABLE tag (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  assoc_table assoc_table NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE table_row_tags (
    assoc_table_row_id INTEGER,
    assoc_table assoc_table,
    tag_id INTEGER REFERENCES tag(id),
    PRIMARY KEY (assoc_table_row_id, assoc_table, tag_id)
);

INSERT INTO tag (name, assoc_table)
VALUES ('blog tag', 'blog');