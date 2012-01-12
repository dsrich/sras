#CREATE TABLE `assets` (
#  `id` char(36) NOT NULL,
#  `asset_type` tinyint(4) NOT NULL,
#  `sha256` char(64) NOT NULL,
#  `name` varchar(64) DEFAULT NULL,
#  `description` varchar(64) DEFAULT NULL,
#  `local` tinyint(1) DEFAULT NULL,
#  `temporary` tinyint(1) DEFAULT NULL,
#  `base_dir` varchar(64) NOT NULL,
#  `created_at` int(11) DEFAULT NULL,
#  `updated_at` int(11) DEFAULT NULL,
#  `enabled` tinyint(1) NOT NULL DEFAULT '1',
#  PRIMARY KEY (`id`),
#  KEY `assets_id_sha256` (`id`,`sha256`),
#  KEY `assets_sha256` (`sha256`)
#) ENGINE=MyISAM DEFAULT CHARSET=ascii;

class Asset
    include DataMapper::Resource
    property :id, String, :key => true
    property :asset_type, Integer
    property :sha256, String
    property :name, String
    property :description, String
    property :local, Boolean
    property :temporary, Boolean
    property :base_dir, String, :default => $default_asset_dir
    property :created_at, EpochTime
    property :updated_at, EpochTime
    property :enabled, Boolean, :default => true
end
