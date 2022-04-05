use crate::proxy::ProxyServer;
use crate::store::memory_store::MemoryStore;
use crate::store::method::Method;
use crate::store::Store;

mod store;
mod proxy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = MemoryStore::new();
    ProxyServer::start_server(store).await
}


// fn main() {
//     let mut store = MemoryStore::new();
//
//     let route = Route::new(Method::Get, "localhost:8080/test".to_owned());
//
//     match store.store_document(route.to_owned(), WebResponse::new(200, "Het werkt!".to_owned())) {
//         Ok(_) => {}
//         Err(error) => {println!("{:?}", error)}
//     }
//
//     let doc = store.get_document(&route).unwrap();
//     print!("{:?}", doc);
// }
