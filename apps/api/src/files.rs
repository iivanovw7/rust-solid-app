use actix_files::{Files, NamedFile};
use actix_web::web;

pub fn get_files_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Files::new("/", "../web/build/dist")
            .index_file("index.html")
            .default_handler(
                NamedFile::open(format!("{}/index.html", "../web/build/dist")).unwrap(),
            ),
    );
}
