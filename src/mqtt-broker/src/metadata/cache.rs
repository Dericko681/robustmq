use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::{cluster::Cluster, session::Session, user::User};

#[derive(Serialize, Deserialize)]
pub enum MetadataCacheAction {
    Set,
    Del,
}

#[derive(Serialize, Deserialize)]
pub enum MetadataCacheType {
    Cluster,
    User,
    Topic,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataChangeData {
    pub action: MetadataCacheAction,
    pub data_type: MetadataCacheType,
    pub value: String,
}

pub struct MetadataCache {
    pub cluster_info: Cluster,
    pub user_info: HashMap<String, User>,
    pub session_info: HashMap<String, Session>,
}

impl MetadataCache {
    pub fn new() -> Self {
        return MetadataCache {
            user_info: HashMap::new(),
            session_info: HashMap::new(),
            cluster_info: Cluster::default(),
        };
    }

    pub fn apply(&mut self, data: String) {
        let data: MetadataChangeData = serde_json::from_str(&data).unwrap();
        match data.data_type {
            MetadataCacheType::User => match data.action {
                MetadataCacheAction::Set => self.set_user(data.value),
                MetadataCacheAction::Del => self.del_user(data.value),
            },
            MetadataCacheType::Topic => match data.action {
                MetadataCacheAction::Set => {}
                MetadataCacheAction::Del => {}
            },
            MetadataCacheType::Cluster => match data.action {
                MetadataCacheAction::Set => {}
                MetadataCacheAction::Del => {}
            },
        }
    }

    pub fn set_user(&mut self, value: String) {
        let data: User = serde_json::from_str(&value).unwrap();
        self.user_info.insert(data.username.clone(), data);
    }

    pub fn del_user(&mut self, value: String) {
        let data: User = serde_json::from_str(&value).unwrap();
        self.user_info.remove(&data.username);
    }
}
