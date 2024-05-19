mod template;

use actix_web::{web, App, HttpServer};
use template::render_page;
// Use random
use rand::random;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host_address = "localhost:8080";
    println!("Starting server at http://{}", host_address);
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
            .service(actix_files::Files::new("/static", "public").show_files_listing())
    })
    .bind(host_address)?
    .run()
    .await
}
