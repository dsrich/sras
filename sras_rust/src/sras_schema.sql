CREATE TABLE `assets` (
  `id` char(36) NOT NULL,
  `asset_type` tinyint(4) NOT NULL,
  `sha256` char(64) NOT NULL,
  `name` varchar(256) DEFAULT NULL,
  `description` varchar(256) DEFAULT NULL,
  `local` tinyint(1) DEFAULT NULL,
  `temporary` tinyint(1) DEFAULT NULL,
  `base_dir` varchar(64) NOT NULL,
  `created_at` int(11) DEFAULT NULL,
  `updated_at` int(11) DEFAULT NULL,
  `enabled` tinyint(1) NOT NULL DEFAULT '1',
  PRIMARY KEY (`id`),
  KEY `assets_id_sha256` (`id`,`sha256`),
  KEY `assets_sha256` (`sha256`)
) DEFAULT CHARSET=utf8;
