-- Add down migration script here
alter table records 
   drop file_guid,
   drop file_path;
