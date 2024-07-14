-- Add up migration script here
CREATE TABLE records (
  ID uuid DEFAULT gen_random_uuid(),
  MESSAGE VARCHAR,
  FILE_NAME VARCHAR,
  PRIMARY KEY (ID)
);
