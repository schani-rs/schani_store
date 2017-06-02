CREATE TABLE images_collections (
  id SERIAL PRIMARY KEY,
  image_id INTEGER,
  collection_id INTEGER,
  UNIQUE (image_id, collection_id)
)
