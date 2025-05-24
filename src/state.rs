use dashmap::DashMap;
use std::sync::Arc;
use std::collections::HashSet;
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub type ClientId = String;
pub type GroupId = String;
pub type Topic = String;

#[allow(dead_code)]
pub struct AppState{
    pub clients: DashMap<ClientId, UnboundedSender<Message>>,
    pub groups: DashMap<GroupId, HashSet<ClientId>>, //this is for the future implementation of group features.
    pub topics: DashMap<Topic, HashSet<ClientId>>, //this is for the future implementation of topics based broadcasting feature.
    pub missed_messages: DashMap<ClientId, Vec<Message>>,
}

pub type SharedState = Arc<AppState>;

impl AppState{
    pub fn new()->SharedState{
        Arc::new(
            AppState{
                clients: DashMap::new(),
                groups: DashMap::new(),
                topics: DashMap::new(),
                missed_messages: DashMap::new()
            }
        )
    }
}
