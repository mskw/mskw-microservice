use std::sync::{Mutex, Arc};
use crate::services::node::NodeInfo;
use std::cell::RefCell;
use std::iter::Map;
use std::collections::HashMap;
use crate::services::json_service::ServiceInfo;

pub struct SystemStatus {
    pub nodes: RefCell<HashMap<String, NodeInfo>>,
    pub services: RefCell<HashMap<String, ServiceInfo>>,
}

impl SystemStatus {
    pub fn new() -> Self {
        SystemStatus {
            nodes: RefCell::new(HashMap::new()),
            services: RefCell::new(HashMap::new()),
        }
    }
}
