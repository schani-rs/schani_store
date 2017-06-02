CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT,
  license VARCHAR NOT NULL,
  side_car_file VARCHAR
)
