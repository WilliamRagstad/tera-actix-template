mod template;

use actix_web::{web, App, HttpServer};
use env_logger;
use rand::random;
use template::render_page;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let host_address = "localhost:8080";
    log::info!("Starting server at http://{}", host_address);
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| {
                    render_page(
                        "pages/index.html",
                        vec![
                            ("name", format!("User #{}", random::<u32>())),
                            ("age", (random::<u8>() % 100).to_string()),
                            (
                                "location",
                                if random::<f32>() < 0.85 {
                                    "Earth"
                                } else {
                                    "Mars"
                                }
                                .into(),
                            ),
                        ],
                    )
                }),
            )
            .service(actix_files::Files::new("public", "public").show_files_listing())
    })
    .bind(host_address)?
    .run()
    .await
}
