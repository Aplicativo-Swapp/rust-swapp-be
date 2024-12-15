use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use chrono::Utc;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
struct Dados {
    id_users: i32,
    id_sub_habilidade: i32,
    descricao: String,
    valor: f64,
}

// POST: Insere dados no banco
async fn inserir_dados(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<Dados>,
) -> impl Responder {
    let created_at = Utc::now().naive_utc();

    let query = r#"
        INSERT INTO public.usuario_sub_habilidade (id_users, id_sub_habilidade, descricao, valor, created_at)
        VALUES ($1, $2, $3, $4, $5)
    "#;

    let result = sqlx::query(query)
        .bind(dados.id_users)
        .bind(dados.id_sub_habilidade)
        .bind(&dados.descricao)
        .bind(dados.valor)
        .bind(created_at)
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

// GET: Retorna todos os dados de um usuário
async fn obter_dados(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o ID do usuário como parâmetro
) -> impl Responder {
    let id_users = path.into_inner();
    let query = r#"
        SELECT id_users, id_sub_habilidade, descricao, valor, created_at
        FROM public.usuario_sub_habilidade
        WHERE id_users = $1
    "#;

    let result = sqlx::query_as::<_, Dados>(query)
        .bind(id_users)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(dados) => HttpResponse::Ok().json(dados),
        Err(e) => {
            eprintln!("Erro ao buscar dados: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao buscar dados")
        }
    }
}

// GET: Retorna todos os dados da tabela
async fn obter_tudo(
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let query = r#"
        SELECT id_users, id_sub_habilidade, descricao, valor, created_at
        FROM public.usuario_sub_habilidade
    "#;

    let result = sqlx::query_as::<_, Dados>(query)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(dados) => HttpResponse::Ok().json(dados),
        Err(e) => {
            eprintln!("Erro ao buscar todos os dados: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao buscar todos os dados")
        }
    }
}

// DELETE: Deleta todos os dados de um usuário
async fn deletar_dados(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o ID do usuário como parâmetro
) -> impl Responder {
    let id_users = path.into_inner();
    let query = r#"
        DELETE FROM public.usuario_sub_habilidade
        WHERE id_users = $1
    "#;

    let result = sqlx::query(query)
        .bind(id_users)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Dados deletados com sucesso"),
        Err(e) => {
            eprintln!("Erro ao deletar dados: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao deletar dados")
        }
    }
}

// PUT: Atualiza todos os dados de uma sub-habilidade de um usuário
async fn atualizar_dados(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<Dados>, // Recebe os dados no corpo da requisição
) -> impl Responder {
    let query = r#"
        UPDATE public.usuario_sub_habilidade
        SET descricao = $1, valor = $2
        WHERE id_users = $3 AND id_sub_habilidade = $4
    "#;

    let result = sqlx::query(query)
        .bind(&dados.descricao)
        .bind(dados.valor)
        .bind(dados.id_users)
        .bind(dados.id_sub_habilidade)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Dados atualizados com sucesso"),
        Err(e) => {
            eprintln!("Erro ao atualizar dados: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao atualizar dados")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL deve ser definida no .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Erro ao conectar ao banco de dados");

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/inserir", web::post().to(inserir_dados))
            .route("/obter/{id_users}", web::get().to(obter_dados))
            .route("/obter_tudo", web::get().to(obter_tudo))
            .route("/deletar/{id_users}", web::delete().to(deletar_dados))
            .route("/atualizar", web::put().to(atualizar_dados))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
