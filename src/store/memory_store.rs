use std::collections::HashMap;
use crate::store::errors::{StoreResult};
use crate::store::errors::StoreErrorType::DocumentAlreadyExists;
use crate::store::web_response::WebResponse;
use crate::store::route::Route;
use crate::store::Store;

#[derive(Clone)]
pub struct MemoryStore {
    routes: HashMap<Route, WebResponse>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        MemoryStore {
            routes: HashMap::new(),
        }
    }
}

impl Store for MemoryStore {
    fn store_document(&mut self, route: Route, response: WebResponse) -> StoreResult<()> {
        match self.routes.contains_key(&route) {
            true => Err(DocumentAlreadyExists),
            false => {
                self.routes.insert(route, response);
                Ok(())
            }
        }
    }

    fn get_document(&self, route: &Route) -> Option<&WebResponse> {
        self.routes.get(&route)
    }

    fn delete_document(&mut self, route: &Route) {
        todo!()
    }
}