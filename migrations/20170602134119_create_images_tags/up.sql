CREATE TABLE images_tags (
  id SERIAL PRIMARY KEY,
  image_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  UNIQUE (image_id, tag_id)
)
