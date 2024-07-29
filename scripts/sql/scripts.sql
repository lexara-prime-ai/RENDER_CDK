-- PGSQL.
-- The following are sample queries for validating whether
-- the database instance has been created successfully.

-- Create a sample schema.
create schema sample_schema;

-- Create a sample table.
create table sample_schema.users(
	id serial primary key,
  username varchar(255),
  email varchar(255)
);

-- View table structure.
SELECT 
  column_name, 
  data_type, 
  character_maximum_length, 
  is_nullable, 
  column_default 
FROM 
  information_schema.columns 
WHERE 
  table_name = 'users';

-- Cleanup.
drop table sample_schema.users;
