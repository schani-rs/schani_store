ALTER TABLE images_collections ADD CONSTRAINT image_fk FOREIGN KEY (image_id) REFERENCES images (id);
ALTER TABLE images_collections ADD CONSTRAINT collection_fk FOREIGN KEY (collection_id) REFERENCES collections (id);
