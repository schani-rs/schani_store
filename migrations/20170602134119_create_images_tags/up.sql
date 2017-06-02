CREATE TABLE images_tags (
  id SERIAL PRIMARY KEY,
  image_id INTEGER,
  tag_id INTEGER,
  UNIQUE (image_id, tag_id)
)
