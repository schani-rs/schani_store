ALTER TABLE images ADD CONSTRAINT raw_image_fk FOREIGN KEY (raw_image_id) REFERENCES raw_images (id);
