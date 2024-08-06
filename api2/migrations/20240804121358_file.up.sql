-- Add up migration script here
alter table records 
   add file_guid uuid,
   add file_path varchar;
