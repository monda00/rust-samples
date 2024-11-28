CREATE DATABASE IF NOT EXISTS `dieseldemo`;
USE `dieseldemo`;
CREATE USER 'demouser'@'%' IDENTIFIED WITH 'caching_sha2_password' BY 'password';
GRANT ALL PRIVILEGES ON dieseldemo.* TO 'demouser';
SET GLOBAL default_authentication_plugin='caching_sha2_password';