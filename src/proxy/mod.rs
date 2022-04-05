use std::borrow::{Borrow, BorrowMut};
use std::sync::Mutex;
use actix_web::{App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, web};
use actix_web::web::{Bytes, Data};
use awc::http::StatusCode;
use crate::{Method, Store};
use crate::store::route::Route;
use crate::store::web_response::WebResponse;

pub struct ProxyServer;

impl ProxyServer {
    pub async fn start_server<T: 'static + Store> (store: T) -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(Mutex::new(store.to_owned())))
                .route("/proxy.pac", web::get().to(ProxyServer::pac_file))
                .route("/{parts:.*}", web::get().to(ProxyServer::proxy_request::<T>))
        })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }

    async fn pac_file() -> HttpResponse {
        HttpResponse::Ok().body(include_str!("./proxy.js"))
    }

    async fn proxy_request<T: 'static + Store> (req: HttpRequest, mut store: Data<Mutex<T>>) -> HttpResponse {
        let route = Route::new(Method::from_awc(req.method()).unwrap(), req.uri().to_string());
        if let Some(document) = store.lock().unwrap().get_document(&route) {
            return HttpResponseBuilder::new(StatusCode::from_u16(document.code).unwrap()).body(document.body.to_owned());
        }
        match ProxyServer::make_request(&req).await {
            Ok((status, body)) => {
                store.lock().unwrap().store_document(route, WebResponse::new(status, body.to_owned()));
                HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap()).body(body)
            },
            Err(response) => response
        }
    }

    async fn make_request(req: &HttpRequest) -> Result<(u16, String), HttpResponse> {
        let client = awc::Client::default();

        let mut result = client.request(req.method().to_owned(), req.uri()) // <- Create request builder
            .insert_header(("User-Agent", "Actix-web"))
            .send()
            .await
            .map_err(|_| HttpResponse::BadGateway().body("Couldn't make webrequest inside proxy"))?;

        let body = result
            .body()
            .await
            .map_err(|_| HttpResponse::BadGateway().body("Error while reading body of proxied call"))?;

        Ok((result.status().as_u16(), std::str::from_utf8(&body).unwrap().to_string()))
    }
}


