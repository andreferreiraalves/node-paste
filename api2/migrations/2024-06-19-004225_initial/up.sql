-- Your SQL goes here
CREATE TABLE records (
  ID uuid DEFAULT gen_random_uuid(),
  CONTENT VARCHAR,
  FILE_NAME VARCHAR,
  PRIMARY KEY (ID)
);