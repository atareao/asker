use actix_web::{get, post, web, Result, error, Error, Responder};
use actix_web_lab::respond::Html;
use sqlx::{sqlite::SqlitePool, query};
use tera::Tera;
use std::collections::HashMap;
use crate::config::Configuration;
use log::{debug, error};




#[get("/{table}")]
pub async fn get_form(template: web::Data<Tera>, configuration: web::Data<Configuration>, table_path: web::Path<String>) -> Result<impl Responder, Error>{
    let table_name = table_path.into_inner();
    let html = match configuration.get_table(&table_name){
        Some(table) => {
            let mut ctx = tera::Context::new();
            ctx.insert("table", &table_name);
            ctx.insert("title", &table.title);
            ctx.insert("instructions", &table.instructions);
            ctx.insert("fields", &table.fields);
            debug!("Template: {}", table.template);
            template.render(&table.template, &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))
            
        },
        None => 
            template.render("404.html", &tera::Context::new())
                .map_err(|_| error::ErrorInternalServerError("Template error"))
    }.unwrap();
    debug!("Content: {}", &html);
    Ok(Html(html))
}

#[post("/{table}")]
pub async fn post_form(template: web::Data<Tera>, pool: web::Data<SqlitePool>,
        configuration: web::Data<Configuration>, table_path: web::Path<String>,
        form: web::Form<HashMap<String, String>>) 
        -> Result<impl Responder, Error>{
    let table_name = table_path.into_inner();
    debug!("Content: {:?}", form);
    let html = match configuration.get_table(&table_name){
        Some(table) => {
            let sql = table.insert(&table_name);
            let mut sql_query = query::<sqlx::Sqlite>(&sql);
            for field in table.fields.as_slice(){
                match form.get(&field.name) {
                    Some(value) => sql_query = sql_query.bind(value),
                    None => sql_query = sql_query.bind(""),
                }
            }
            match sql_query.execute(pool.get_ref()).await{
                Ok(_)  => template.render("200.html", &tera::Context::new())
                            .map_err(|_| error::ErrorInternalServerError("Template error")),
                Err(e) => {
                    error!("No pude: {}", e);
                    let context = tera::Context::new();
                    let template_file = if e.to_string().to_lowercase().contains("unique"){
                        "200.html"
                    }else{
                        "500.html"
                    };
                    template.render(template_file, &context)
                            .map_err(|_| error::ErrorInternalServerError("Template error"))
                }
            }
        },
        None =>  template.render("500.html", &tera::Context::new())
                .map_err(|_| error::ErrorInternalServerError("Template error"))
    }.unwrap();
    Ok(Html(html))
}
