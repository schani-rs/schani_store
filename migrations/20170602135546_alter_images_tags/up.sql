ALTER TABLE images_tags ADD CONSTRAINT image_fk FOREIGN KEY (image_id) REFERENCES images (id);
ALTER TABLE images_tags ADD CONSTRAINT tag_fk FOREIGN KEY (tag_id) REFERENCES tags (id);
