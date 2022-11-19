mod routes;
mod config;

use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use std::process;
use tokio::fs;
use sqlx::{query, sqlite::{SqlitePool, SqlitePoolOptions},
    migrate::MigrateDatabase};
use env_logger::Env;
use log::{debug, error};
use tera::Tera;
use actix_files;

use crate::config::Configuration;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let content = match fs::read_to_string("config.yml")
        .await {
            Ok(value) => value,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        };
    let configuration = Configuration::new(&content)
        .expect("Someting went wrong");

    let log_level = configuration.get_log_level();
    debug!("Log level: {}", log_level);
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    let db_url = configuration.get_db_url();
    debug!("Database url: {}", db_url);
    let port = configuration.get_port();
    debug!("Port: {}", port);

    let template = match Tera::new("templates/**/*.html"){
        Ok(t) => t,
        Err(e) => {
            error!("Can not load templates, {}", e);
            process::exit(1);
        }
    };
    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap(){
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("Pool failed");

    
    init(&pool, &configuration).await;

    let conf = configuration.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(conf.clone()))
            .app_data(Data::new(template.clone()))
            .service(routes::get_form)
            .service(routes::post_form)
            .service(routes::get_results)
            .service(actix_files::Files::new("/static", "./static"))
    })
    .workers(4)
    .bind(format!("0.0.0.0:{}", &port))
    .unwrap()
    .run()
    .await
}

async fn init(pool: &SqlitePool, config: &Configuration){
    for table_name in config.tables.keys(){
        match config.get_table(table_name){
            Some(table) => {
                let sql = table.create(table_name);
                debug!("Sql creation query: {}", &sql);
                query(&sql)
                    .execute(pool)
                    .await
                    .unwrap();
            },
            None => {}
        }
    }
}
