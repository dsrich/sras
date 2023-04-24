  // OpenSim assets table:
  /*
  MariaDB [opensim]> describe assets
    -> ;
+-------------+--------------+------+-----+--------------------------------------+-------+
| Field       | Type         | Null | Key | Default                              | Extra |
+-------------+--------------+------+-----+--------------------------------------+-------+
| name        | varchar(64)  | NO   |     | NULL                                 |       |
| description | varchar(64)  | NO   |     | NULL                                 |       |
| assetType   | tinyint(4)   | NO   |     | NULL                                 |       |
| local       | tinyint(1)   | NO   |     | NULL                                 |       |
| temporary   | tinyint(1)   | NO   |     | NULL                                 |       |
| data        | longblob     | NO   |     | NULL                                 |       |
| id          | char(36)     | NO   | PRI | 00000000-0000-0000-0000-000000000000 |       |
| create_time | int(11)      | YES  |     | 0                                    |       |
| access_time | int(11)      | YES  |     | 0                                    |       |
| asset_flags | int(11)      | NO   |     | 0                                    |       |
| CreatorID   | varchar(128) | NO   |     |                                      |       |
+-------------+--------------+------+-----+--------------------------------------+-------+
   */

pub struct asset {
    asset_type: i64, // Only needs 8 bits, but i64 is faster
    created_at: Option<i64>, // What format is OpenSim using? i64 is for Unix time in seconds
    updated_at: Option<i64>,
    asset_flags: i64,
    id: String,  // Do we want to parse it as a UUID? Any reason to?
    sha256: String,
    name: String,
    descripion: String,
    base_dir: String, // Allows split file structure
    creator_id: String,
    local: bool,
    temporary: bool,
    enabled: bool,
    file_store: bool, // Small assets are in the db, large ones are saved as files
    asset: Option<Vec<u8>>,
}

static SCHEMA: String = r#"
CREATE TABLE `assets` (
    `id` char(36) NOT NULL default '00000000-0000-0000-0000-000000000000',
    `asset_type` tinyint(4) NOT NULL,
    `sha256` char(64) NOT NULL,
    `name` varchar(64) not NULL,
    `description` varchar(64) not null,
    `base_dir` varchar(64) NOT NULL default '.',
    `creator_id` varchar(128) not null,
    `created_at` int(11) DEFAULT NULL,
    `updated_at` int(11) DEFAULT NULL,
    `asset_flags` int(11) not null default 0,
    `enabled` tinyint(1) NOT NULL DEFAULT '1',
    `local` tinyint(1) not null default 0,
    `temporary` tinyint(1) not null default 0,
    `file_store` tinyint(1) not null default 0,
    `asset` blob default null,
    PRIMARY KEY (`id`),
    KEY `assets_id_sha256` (`id`,`sha256`),
    KEY `assets_sha256` (`sha256`)
  ) DEFAULT CHARSET=utf8;
  "#;

