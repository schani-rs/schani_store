CREATE TABLE images_collections (
  id SERIAL PRIMARY KEY,
  image_id INTEGER NOT NULL,
  collection_id INTEGER NOT NULL,
  UNIQUE (image_id, collection_id)
)
