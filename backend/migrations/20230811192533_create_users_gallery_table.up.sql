-- Add up migration script here
CREATE  TABLE IF NOT EXISTS users_gallery
(
 email         VARCHAR(255)  REFERENCES users ON DELETE CASCADE NOT NULL ,
 apod_id         integer REFERENCES apod ON DELETE CASCADE  NOT NULL,

    -- Create a composite unique constraint
 CONSTRAINT unique_combination UNIQUE (email, apod_id)
);
