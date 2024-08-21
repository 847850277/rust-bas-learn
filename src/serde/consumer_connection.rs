use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize,Serializer,ser::SerializeStruct};
use serde_json_any_key::*;


#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Connection{

    client_id: String,
    client_addr: String,
    version: i32,

}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionData {
    pub class_filter_mode: bool,
    pub topic: String,
    pub sub_string: String,
    pub tags_set: HashSet<String>,
    pub code_set: HashSet<i32>,
    pub sub_version: i64,
    pub expression_type: String,
    // In Rust, attributes like `@JSONField(serialize = false)` are typically handled through
    // documentation or external crates.
    #[serde(skip)]
    pub filter_class_source: String, // This field is not used in this example.
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default,Serialize,Deserialize)]
pub enum ConsumeType {
    #[default]
    ConsumeActively,
    ConsumePassively,
    ConsumePop,
}

#[derive(Debug, Clone, Default)]
pub struct ConsumerConnection {
    connection_set: HashSet<Connection>,
    subscription_table: Arc<RwLock<HashMap<String, SubscriptionData>>>,
    consume_type: Arc<RwLock<ConsumeType>>,
}

impl ConsumerConnection {
    pub fn new() -> Self {
        ConsumerConnection {
            connection_set: HashSet::new(),
            subscription_table: Arc::new(RwLock::new(HashMap::new())),
            consume_type: Arc::new(RwLock::new(ConsumeType::default())),
        }
    }



    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

}

impl Serialize for ConsumerConnection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("ConsumerConnection", 3)?;

        state.serialize_field("connection_set", &self.connection_set)?;
        state.serialize_field("consume_type", &self.consume_type)?;

        let subscription_table = self.subscription_table.read().unwrap();
        state.serialize_field("subscription_table", &*subscription_table)?;

        state.end()
    }
}


pub fn test(){

    let consumer_connection = ConsumerConnection::new();
    println!("{}", consumer_connection.to_json());

}