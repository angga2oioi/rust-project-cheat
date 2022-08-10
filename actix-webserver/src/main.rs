use actix_web::{web, HttpServer, App, guard};
use listenfd::ListenFd;
use {routes::v1::register as v1_route};

mod routes;
mod controllers;
mod model;
mod error;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/v1")
                .guard(guard::Header("Accept", "application/json"))
                .configure(v1_route))
            .default_service(web::route().to(crate::controllers::handlers::page_not_found))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
// systemfd --no-pid -s http::3000 -- cargo watch -x run
// https://cheats.rs/
