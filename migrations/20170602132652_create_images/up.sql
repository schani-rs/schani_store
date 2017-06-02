CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  license VARCHAR NOT NULL,
  side_car_file VARCHAR NOT NULL,
  raw_image_id INTEGER NOT NULL
)
