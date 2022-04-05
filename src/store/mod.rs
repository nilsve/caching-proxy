use crate::store::errors::StoreResult;
use crate::store::route::Route;
use crate::store::web_response::WebResponse;

pub mod memory_store;
pub mod route;
pub mod web_response;
pub mod method;
pub mod errors;

pub trait Store: Clone + Send {
    fn store_document(&mut self, route: Route, response: WebResponse) -> StoreResult<()>;
    fn get_document(&self, route: &Route) -> Option<&WebResponse>;
    fn delete_document(&mut self, route: &Route);
}