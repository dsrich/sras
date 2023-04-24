use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

static SCHEMA: &str = r#"
DROP TABLE if exists `sras_assets`;
CREATE TABLE `sras_assets` (
    `id` char(36) NOT NULL default '00000000-0000-0000-0000-000000000000',
    `asset_type` tinyint(4) NOT NULL,
    `sha256` char(64) NOT NULL,
    `name` varchar(64) not NULL,
    `description` varchar(64) not null,
    `base_dir` varchar(64) NOT NULL,
    `creator_id` varchar(128) not null,
    `created_at` int(11) DEFAULT NULL,
    `updated_at` int(11) DEFAULT NULL,
    `asset_flags` int(11) not null default 0,
    `bools`tinyint(4) not null default 4,
    `strings` varchar(512) not null,
    `asset` blob default null,
    PRIMARY KEY (`id`),
    KEY `assets_id_sha256` (`id`,`sha256`),
    KEY `assets_sha256` (`sha256`)
) DEFAULT CHARSET=utf8;
"#;

// For ease of handling merged bools
// Or together for true
static LOCAL: i8 = 1;
static TEMPORARY: i8 = 2;
static ENABLED: i8 = 4;
static FILESTORE: i8 = 8;
// And in for false
static NOTLOCAL: i8 = 126;
static NOTTEMPORARY: i8 = 125;
static NOTENABLED: i8 = 123;
static NOTFILESTORE: i8 = 119;

// OpenSim assets table:
/*
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct AssetSql {

    id: String,  // Do we want to parse it as a UUID? Any reason to?
    sha256: String,
    packed: String,
/* 
    asset_type: i64, // Only needs 8 bits, but i64 is faster
    created_at: Option<i64>, // What format is OpenSim using? i64 is for Unix time in seconds
    updated_at: Option<i64>,
    asset_flags: i64,
    name: String,
    descripion: String,
    base_dir: String, // Allows split file structure
    creator_id: String,
*/
    bools: i8,
/*
    local: bool, = 1
    temporary: bool, = 2
    enabled: bool, = 4
    file_store: bool, = 8
        // Small assets are in the db, large ones are saved as files
*/
    asset: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssetStr {
    asset_type: i64, // Only needs 8 bits, but i64 is faster
    created_at: Option<i64>, // What format is OpenSim using? i64 is for Unix time in seconds
    updated_at: Option<i64>,
    asset_flags: i64,
    name: String,
    description: String,
    base_dir: String,
    creator_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Asset {
    asset_type: i64, // Only needs 8 bits, but i64 is faster
    created_at: Option<i64>, // What format is OpenSim using? i64 is for Unix time in seconds
    updated_at: Option<i64>,
    asset_flags: i64,
    id: String,  // Do we want to parse it as a UUID? Any reason to?
    sha256: String,
    name: String,
    description: String,
    base_dir: String, // Allows split file structure
    creator_id: String,
    local: bool, // bools & LOCAL != 0
    temporary: bool, // bools & TEMPORARY != 0
    enabled: bool, // bools & ENABLED != 0
    file_store: bool, // bools & FILESTORE != 0
        // Small assets are in the db, large ones are saved as files
    asset: Vec<u8>,
}

fn new_asset() -> Asset {
    Asset {
        created_at: None,
        updated_at: None,
        asset_flags: 0,
        asset_type: 0,
        id: "".to_string(),
        sha256: "".to_string(),
        name: "".to_string(),
        description: "".to_string(),
        base_dir: "".to_string(),
        creator_id: "".to_string(),
        local: false,
        temporary: false,
        enabled: true,
        file_store: false,
        asset: vec![],
    }
}

fn new_asset_sql() -> AssetSql {
    AssetSql{
        id: "".to_string(),
        sha256: "".to_string(),
        packed: "".to_string(),
        bools: ENABLED,
        asset: None,
    }
}

fn unpack_sql(inp: AssetSql) -> Asset {
    let mut outp = new_asset();
    outp.id = inp.id;
    outp.sha256 = inp.sha256;
    let inpstr: AssetStr = serde_json::from_str(&inp.packed)
        .expect("Failed to parse packed strings JSON from database");
    outp.name = inpstr.name;
    outp.description = inpstr.description;
    outp.base_dir = inpstr.base_dir;
    outp.creator_id = inpstr.creator_id;
    outp.asset_type = inpstr.asset_type;
    outp.created_at = inpstr.created_at;
    outp.updated_at = inpstr.updated_at;
    outp.local = (inp.bools & LOCAL) != 0;
    outp.temporary = (inp.bools & TEMPORARY) != 0;
    outp.enabled = (inp.bools & ENABLED) != 0;
    outp.file_store = (inp.bools & FILESTORE) != 0;
    if outp.file_store {
        // TODO: Need code here to read asset from file
    } else {
        match inp.asset {
            Some(asset1) => outp.asset = asset1,
            None => outp.asset = vec![] // Missing asset data...
        }
    }
    outp
}