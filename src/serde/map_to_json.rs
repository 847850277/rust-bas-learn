use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json_any_key::*;

pub(crate) fn test() {
    // init DedupTree
    let tree = DedupTree::new();
    // insert value to DedupTree
    let mut tree = DedupTree::new();
    let path = PathBuf::from("test");
    let hash = [0; 32];
    tree.file_tree.insert(path.clone(), hash);
    tree.hash_tree.insert(hash, vec![path.clone()]);
    // convert DedupTree to json
    let json = tree.to_json();
    // convert json to DedupTree
    let tree = DedupTree::from_json(&json).unwrap();
    println!("{:?}", tree);
}

pub(crate) fn test_2() {
    // insert value to TopicStatsTable
    let mut table = TopicStatsTable::new();
    let queue = MessageQueue::new();
    let offset = TopicOffset::new();

    // format!("{:?}", queue);

    table.offset_table.insert(queue, offset);
    // convert TopicStatsTable to json
    let json = serde_json::to_string(&table).unwrap();
    dbg!(json);
    // convert json to TopicStatsTable
    //let table: TopicStatsTable = serde_json::from_str(&json).unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DedupTree {
    #[serde(with = "any_key_map")]
    hash_tree: BTreeMap<[u8; 32], Vec<PathBuf>>,
    #[serde(with = "any_key_map")]
    file_tree: BTreeMap<PathBuf, [u8; 32]>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicStatsTable {
    #[serde(with = "any_key_map")]
    offset_table: HashMap<MessageQueue, TopicOffset>,
    //offset_table: HashMap<String, TopicOffset>,
}

impl TopicStatsTable {
    pub fn new() -> Self {
        Self {
            offset_table: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct MessageQueue {
    topic: String,
    broker_name: String,
    queue_id: i32,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            topic: String::new(),
            broker_name: String::new(),
            queue_id: 0,
        }
    }
}

impl fmt::Display for MessageQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MessageQueue [topic={}, brokerName={}, queueId={}]",
            self.topic, self.broker_name, self.queue_id
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopicOffset {
    min_offset: i64,
    max_offset: i64,
    last_update_timestamp: i64,
}

impl TopicOffset {
    pub fn new() -> Self {
        Self {
            min_offset: 0,
            max_offset: 0,
            last_update_timestamp: 0,
        }
    }
}

impl DedupTree {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn new() -> Self {
        DedupTree {
            hash_tree: BTreeMap::new(),
            file_tree: BTreeMap::new(),
        }
    }
}
