----------------------------------------
--             WARNING!               --
--  if you change the table schema,   --
--  you will have to also change the  --
--               code!                --
----------------------------------------

CREATE OR REPLACE FUNCTION update_updated_on()
RETURNS TRIGGER AS $$
BEGIN
  NEW.edited = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

\i /docker-entrypoint-initdb.d/sql/blog.sql
