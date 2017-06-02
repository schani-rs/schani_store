CREATE TABLE raw_images (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  camera VARCHAR NOT NULL,
  latitude DOUBLE PRECISION,
  longitude DOUBLE PRECISION,
  creation DATE NOT NULL
)
