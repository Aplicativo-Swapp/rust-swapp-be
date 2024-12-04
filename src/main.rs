use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use dotenv::dotenv;
use std::{env, str::FromStr};
use chrono::{Utc, NaiveDateTime};

#[derive(Deserialize, Serialize)]
struct Dados {
    id_users: i32,           // Remover o campo `id` do corpo da requisição
    id_sub_habilidade: i32,
    descricao: String,
    valor: f64,
}

async fn inserir_dados(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<Dados>,
) -> impl Responder {

    // let created_at_str = Utc::now().naive_utc().to_string();
    // let created_at = NaiveDateTime::from_str(&created_at_str).unwrap();

    let created_at = Utc::now().naive_utc();

    // Consulta SQL para inserir dados, com o `id` sendo gerado automaticamente
    let query = r#"
        INSERT INTO public.usuario_sub_habilidade (id_users, id_sub_habilidade, descricao, valor, created_at)
        VALUES ($1, $2, $3, $4, $5)
    "#;

    let result = sqlx::query(query)
        .bind(dados.id_users)
        .bind(dados.id_sub_habilidade)
        .bind(&dados.descricao)
        .bind(dados.valor)
        .bind(created_at)  // Passa o NaiveDateTime convertido
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Dados inseridos com sucesso"),
        Err(e) => {
            eprintln!("Erro ao inserir dados: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao inserir dados")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL deve ser definida no .env");

    // Conecta ao banco de dados PostgreSQL com o SQLx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Erro ao conectar ao banco de dados");

    // Teste de conexão (consulta simples)
    match sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => println!("Conexão com o banco de dados bem-sucedida!"),
        Err(e) => {
            eprintln!("Erro ao testar a conexão com o banco de dados: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Erro na conexão com o banco"));
        }
    }

    // Inicia o servidor Actix
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/inserir", web::post().to(inserir_dados))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}