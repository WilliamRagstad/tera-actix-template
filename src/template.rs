use actix_web::HttpResponse;
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        }
    };
}

pub async fn render_page<T>(template: &'static str, vars: Vec<(&'static str, T)>) -> HttpResponse
where
    T: serde::Serialize,
{
    log::info!("Rendering page: {}", template);
    let mut context = tera::Context::new();
    for (key, value) in vars {
        context.insert(key.to_string(), &value);
    }
    let rendered = match TEMPLATES.render(template, &context) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Rendering error(s): {}", e);
            std::process::exit(1);
        }
    };
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}
