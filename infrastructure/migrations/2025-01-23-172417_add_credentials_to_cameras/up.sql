-- Your SQL goes here

ALTER TABLE `camera` ADD COLUMN `port` SMALLINT UNSIGNED NOT NULL COMMENT 'Porta per connettersi alla camera';
ALTER TABLE `camera` ADD COLUMN `username` VARCHAR(255) NOT NULL COMMENT 'Credenziali per connettersi alla camera';
ALTER TABLE `camera` ADD COLUMN `password` VARCHAR(255) NOT NULL COMMENT 'Credenziali per connettersi alla camera';






























