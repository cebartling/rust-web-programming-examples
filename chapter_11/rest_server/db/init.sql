CREATE USER rustwebdev WITH PASSWORD 'rustwebdev';
CREATE DATABASE rustwebdev_db;
GRANT ALL PRIVILEGES ON DATABASE rustwebdev_db TO rustwebdev;
ALTER DATABASE rustwebdev_db OWNER TO rustwebdev;