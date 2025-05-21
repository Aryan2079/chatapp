use DashMap;
use std::sync::Arc;
use std::collections::HashSet;

pub type ClientId = String;
pub type GroupId = String;
pub type Topic = String;

pub struct AppState{
    pub clients: DashMap<ClientId, UnboundedSender<Message>>,
    pub groups: DashMap<GroupId, HashSet<ClientId>>,
    pub topics: DashMap<Topic, HashSet<ClientId>>,
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
