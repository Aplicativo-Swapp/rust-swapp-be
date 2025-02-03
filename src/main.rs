use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use utoipa::ToSchema;
use chrono::Utc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, Serialize, sqlx::FromRow, ToSchema)]
struct Dados {
    id_users: i32,
    id_sub_habilidade: i32,
    descricao: String,
    valor: f64,
}

// POST: Insere dados no banco
#[utoipa::path(
    post,
    path = "/inserir",
    request_body = Dados,
    responses(
        (status = 200, description = "Dados inseridos com sucesso"),
        (status = 500, description = "Erro ao inserir dados")
    )
)]
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
#[utoipa::path(
    get,
    path = "/obter/{id_users}",
    params(
        ("id_users" = i32, Path, description = "ID do usuário")
    ),
    responses(
        (status = 200, description = "Dados do usuário", body = [Dados]),
        (status = 500, description = "Erro ao buscar dados")
    )
)]
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


#[derive(Deserialize, Serialize, sqlx::FromRow, ToSchema)]
struct Habilidades {
    id: i32,
    nome: String
}

#[utoipa::path(
    get,
    path = "/habilidades",
    
    responses(
        (status = 200, description = "Habilidades retornadas", body = [Habilidades]),
        (status = 500, description = "Erro ao buscar dados")
    )
)]
async fn obter_habilidades(
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let query = r#"
        select id, nome
        from public.habilidade h 
    "#;

    let result = sqlx::query_as::<_, Habilidades>(query)
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

#[utoipa::path(
    get,
    path = "/sub_habilidades",
    
    responses(
        (status = 200, description = "Sub-Habilidades retornadas", body = [Habilidades]),
        (status = 500, description = "Erro ao buscar dados")
    )
)]
async fn obter_sub_habilidades(
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let query = r#"
        select id, nome
        from public.sub_habilidade h 
    "#;

    let result = sqlx::query_as::<_, Habilidades>(query)
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


#[utoipa::path(
    get,
    path = "/sub_habilidade_habilidade/{id}",
    params(
        ("id_habilidade" = i32, Path, description = "ID da Habilidade")
    ),
    responses(
        (status = 200, description = "Dados das Sub Habilidades", body = [Habilidades]),
        (status = 500, description = "Erro ao buscar dados")
    )
)]
async fn obter_sub_habilidades_habilidades(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o ID da habilidade como parâmetro
) -> impl Responder {
    let id_habilidade = path.into_inner();
    let query = r#"
        select id, nome
        from public.sub_habilidade sh 
        where id_habilidade = $1
    "#;

    let result = sqlx::query_as::<_, Habilidades>(query)
        .bind(id_habilidade)
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


#[utoipa::path(
    get,
    path = "/habilidade_sub_habilidade/{id}",
    params(
        ("id_sub_habilidade" = i32, Path, description = "ID da Sub-Habilidade")
    ),
    responses(
        (status = 200, description = "Dados da Habilidade", body = [Habilidades]),
        (status = 500, description = "Erro ao buscar dados")
    )
)]
async fn obter_habilidades_sub_habilidades(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o ID da sub habilidade como parâmetro
) -> impl Responder {
    let id_sub_habilidade = path.into_inner();
    let query = r#"
        select h.id, h.nome
        from public.habilidade h 
        join sub_habilidade sh on sh.id_habilidade = h.id 
        where sh.id = $1
    "#;

    let result = sqlx::query_as::<_, Habilidades>(query)
        .bind(id_sub_habilidade)
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


#[derive(Deserialize, Serialize, sqlx::FromRow, ToSchema)]

struct DadosAll {
    id_users: i32,
    first_name: String,
    last_name: String,
    nome_sub_habilidade: String, // Substituindo id_sub_habilidade
    descricao: String,
    valor: f64,
}
// GET: Retorna todos os dados da tabela
#[utoipa::path(
    get,
    path = "/obter_tudo",
    responses(
        (status = 200, description = "Todos os dados", body = [Dados_all]),
        (status = 500, description = "Erro ao buscar todos os dados")
    )
)]
async fn obter_tudo(
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let query = r#"
        SELECT 
            u.id_users, 
            us.first_name,
            us.last_name,
            u.id_sub_habilidade, 
            s.nome AS nome_sub_habilidade, 
            u.descricao, 
            u.valor, 
            u.created_at
        FROM 
            public.usuario_sub_habilidade AS u
        INNER JOIN 
            public.sub_habilidade AS s
        ON 
            u.id_sub_habilidade = s.id
        INNER JOIN 
            public.users AS us
        ON 
            u.id_users = us.id;
    "#;

    let result = sqlx::query_as::<_, DadosAll>(query)
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
#[utoipa::path(
    delete,
    path = "/deletar/{id_users}",
    params(
        ("id_users" = i32, Path, description = "ID do usuário")
    ),
    responses(
        (status = 200, description = "Dados deletados com sucesso"),
        (status = 500, description = "Erro ao deletar dados")
    )
)]
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
#[utoipa::path(
    put,
    path = "/atualizar",
    request_body = Dados,
    responses(
        (status = 200, description = "Dados atualizados com sucesso"),
        (status = 500, description = "Erro ao atualizar dados")
    )
)]
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

// ENDPOINTS DO MATCH
#[derive(Deserialize)]
struct LikeRequest {
    id_deu_like: i32,
    id_liked: i32,
}

// POST: Adiciona um novo "like" na tabela `teste_match`
#[utoipa::path(
    post,
    path = "/match/add_like",
    request_body = LikeRequest,
    params(
        ("id_deu_like" = i32, Path, description = "ID do usuário que deu os likes"),
        ("id_liked" = i32, Path, description = "ID do usuário que recebeu os likes")
    ),
    responses(
        (status = 200, description = "Like adicionado com sucesso"),
        (status = 500, description = "Erro ao adicionar like")
    )
)]
async fn adicionar_like(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<LikeRequest>, // Usa a struct LikeRequest
) -> impl Responder {
    let LikeRequest { id_deu_like, id_liked } = dados.into_inner();

    let query = r#"
        INSERT INTO public.teste_match (id_deu_like, id_liked, match)
        VALUES ($1, $2, FALSE)
    "#;

    let result = sqlx::query(query)
        .bind(id_deu_like)
        .bind(id_liked)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Like adicionado com sucesso"),
        Err(e) => {
            eprintln!("Erro ao adicionar like: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao adicionar like")
        }
    }
}


// GET: Retorna os IDs de quem deu like em um usuário específico
#[utoipa::path(
    get,
    path = "/match/buscar_likes/{id}",
    params(
        ("id" = i32, Path, description = "ID do usuário que recebeu os likes")
    ),
    responses(
        (status = 200, description = "IDs de quem deu like", body = [(i32, String, String, String)]),
        (status = 500, description = "Erro ao buscar likes")
    )
)]
async fn buscar_likes(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o ID do usuário (id_liked)
) -> impl Responder {
    let id_liked = path.into_inner();

    let query = r#"
        SELECT
            tm.id_deu_like,
            CONCAT(us.first_name, ' ', us.last_name) AS full_name,
            sh.nome AS habilidade,
            us.city
        FROM 
            public.teste_match tm
        JOIN 
            usuario_sub_habilidade ush ON tm.id_deu_like = ush.id_users 
        JOIN 
            users us ON ush.id_users = us.id
        JOIN 
            sub_habilidade sh ON ush.id_sub_habilidade = sh.id
        WHERE 
            tm.id_liked = $1 and tm.match=false
        GROUP BY 
            tm.id_deu_like, 
            sh.nome, 
            us.first_name, 
            us.last_name, 
            us.city;
    "#;

    let result = sqlx::query_as::<_, (i32, String, String, String)>(query)
        .bind(id_liked)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(dados) => {
            HttpResponse::Ok().json(dados)
        }
        Err(e) => {
            eprintln!("Erro ao buscar likes: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao buscar likes")
        }
    }
}


// GET: Retorna os IDs dos usuário quem um user deu like
#[utoipa::path(
    get,
    path = "/buscar_meus_likes/{id}",
    params(
        ("id" = i32, Path, description = "ID do usuário que deu os likes")
    ),
    responses(
        (status = 200, description = "IDs de quem deu like", body = [(i32, String, String, String)]),
        (status = 500, description = "Erro ao buscar likes")
    )
)]
async fn buscar_meus_likes(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o ID do usuário (id_deu_like)
) -> impl Responder {
    let id_deu_like = path.into_inner();

    let query = r#"
        SELECT
            tm.id_liked,
            CONCAT(us.first_name, ' ', us.last_name) AS full_name,
            sh.nome AS habilidade,
        us.city
        FROM 
            public.teste_match tm
        JOIN 
            usuario_sub_habilidade ush ON tm.id_liked = ush.id_users 
        JOIN 
            users us ON ush.id_users = us.id
        JOIN 
            sub_habilidade sh ON ush.id_sub_habilidade = sh.id
        WHERE 
            tm.id_deu_like = $1 and tm.match=false
        GROUP BY 
            tm.id_liked, 
            sh.nome, 
            us.first_name, 
            us.last_name, 
            us.city;
    "#;

    let result = sqlx::query_as::<_, (i32, String, String, String)>(query)
        .bind(id_deu_like)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(dados) => {
            HttpResponse::Ok().json(dados)
        }
        Err(e) => {
            eprintln!("Erro ao buscar likes: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao buscar likes")
        }
    }
}

#[derive(Deserialize)]
struct MatchRequest {
    id_deu_like: i32,
    id_liked: i32,
}

// PUT: Atualiza a coluna match para true em uma linha específica
#[utoipa::path(
    put,
    path = "/match",
    request_body = MatchRequest,
    params(
        ("id_deu_like" = i32, Path, description = "ID do usuário que deu os likes"),
        ("id_liked" = i32, Path, description = "ID do usuário que recebeu os likes")
    ),
    responses(
        (status = 200, description = "Match atualizado com sucesso"),
        (status = 500, description = "Erro ao atualizar match")
    )
)]
async fn atualizar_match(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<MatchRequest>, // Usa uma struct ao invés de tupla
) -> impl Responder {
    let MatchRequest { id_deu_like, id_liked } = dados.into_inner();

    let query = r#"
        UPDATE public.teste_match
        SET match = TRUE
        WHERE id_deu_like = $2 AND id_liked = $1
    "#;

    let result = sqlx::query(query)
        .bind(id_deu_like)
        .bind(id_liked)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Match atualizado com sucesso"),
        Err(e) => {
            eprintln!("Erro ao atualizar match: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao atualizar match")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/match/delete",
    request_body = MatchRequest,
    params(
        ("id_users" = i32, Path, description = "ID do usuário")
    ),
    responses(
        (status = 200, description = "Match removido com sucesso"),
        (status = 500, description = "Erro ao remover Match")
    )
)]
async fn excluir_match(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<MatchRequest>, // Usa uma struct ao invés de tupla
) -> impl Responder {
    let MatchRequest { id_deu_like, id_liked } = dados.into_inner();

    let query = r#"
        DELETE FROM teste_match
        WHERE id_deu_like = $1 AND id_liked = $2 or id_deu_like = $2 AND id_liked = $1;
    "#;

    let result = sqlx::query(query)
        .bind(id_deu_like)
        .bind(id_liked)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Match excluido com sucesso"),
        Err(e) => {
            eprintln!("Erro ao excluir match: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao excluir match")
        }
    }
}


#[utoipa::path(
    post,
    path = "/historico/add",
    request_body = MatchRequest,
    params(
        ("id_deu_like" = i32, Path, description = "ID do usuário que deu os likes"),
        ("id_liked" = i32, Path, description = "ID do usuário que recebeu os likes")
    ),
    responses(
        (status = 200, description = "Histórico atualizado com sucesso"),
        (status = 500, description = "Erro ao atualizar Histórico")
    )
)]
async fn atualizar_historico(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<MatchRequest>, // Usa uma struct ao invés de tupla
) -> impl Responder {
    let MatchRequest { id_deu_like, id_liked } = dados.into_inner();

    let query = r#"
        INSERT INTO historico_match (id1, id2)
        VALUES ($1, $2);
    "#;

    let result = sqlx::query(query)
        .bind(id_deu_like)
        .bind(id_liked)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Histórico atualizado com sucesso"),
        Err(e) => {
            eprintln!("Erro ao atualizar histórico: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao atualizar histórico")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/historico/delete",
    request_body = MatchRequest,
    params(
        ("id_deu_like" = i32, Path, description = "ID do usuário que deu os likes"),
        ("id_liked" = i32, Path, description = "ID do usuário que recebeu os likes")
    ),
    responses(
        (status = 200, description = "Histórico removido com sucesso"),
        (status = 500, description = "Erro ao remover do Histórico")
    )
)]
async fn excluir_historico(
    pool: web::Data<sqlx::PgPool>,
    dados: web::Json<MatchRequest>, // Usa uma struct ao invés de tupla
) -> impl Responder {
    let MatchRequest { id_deu_like, id_liked } = dados.into_inner();

    let query = r#"
        DELETE FROM historico_match
        WHERE (id1 = $1 AND id2 = $2) OR (id1 = $2 AND id2 = $1);
    "#;

    let result = sqlx::query(query)
        .bind(id_deu_like)
        .bind(id_liked)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Histórico excluido com sucesso"),
        Err(e) => {
            eprintln!("Erro ao excluir do histórico: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao excluir do  histórico")
        }
    }
}

// GET: Retorna todos os "matches" de um usuário especificado pelo id_liked
#[utoipa::path(
    get,
    path = "/match/{id_liked}",
    params(
        ("id_liked" = i32, Path, description = "ID do usuário que recebeu os matches")
    ),
    responses(
        (status = 200, description = "IDs de quem deu match", body = [i32]),
        (status = 500, description = "Erro ao buscar matches")
    )
)]
async fn buscar_matches(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o id_liked como parâmetro
) -> impl Responder {
    let id_liked = path.into_inner();

    let query = r#"
        SELECT id_deu_like
        FROM public.teste_match
        WHERE id_liked = $1 AND match = TRUE
    "#;

    let result = sqlx::query_scalar::<_, i32>(query)
        .bind(id_liked)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => {
            eprintln!("Erro ao buscar matches: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao buscar matches")
        }
    }
}

// GET: Retorna todos os "matches" de um usuário especificado pelo id_liked
#[utoipa::path(
    get,
    path = "/historico/{id}",
    params(
        ("id" = i32, Path, description = "ID do usuário")
    ),
    responses(
        (status = 200, description = "IDs de quem deu match", body = [i32]),
        (status = 500, description = "Erro ao buscar matches")
    )
)]
async fn buscar_historico(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o id como parâmetro
) -> impl Responder {
    let id = path.into_inner();

    let query = r#"
        SELECT id2
        FROM public.historico_match
        WHERE id1 = $1
    "#;

    let result = sqlx::query_scalar::<_, i32>(query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => {
            eprintln!("Erro ao buscar matches: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao buscar matches")
        }
    }
}

async fn all_dados_match(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<i32>, // Recebe o id como parâmetro
) -> impl Responder {
    let id = path.into_inner();

    let query = r#"
        select
            tm.id_deu_like,
            CONCAT(uss.first_name, ' ', uss.last_name) as full_name,
            shh.nome as habilidade,
            tm.id_liked,
            CONCAT(us.first_name, ' ', us.last_name) as full_name,
            sh.nome as habilidade
        from
            public.teste_match tm
        join 
            usuario_sub_habilidade ush on tm.id_liked = ush.id_users
        join
            usuario_sub_habilidade ushh on tm.id_deu_like = ushh.id_users
        join 
            users us on ush.id_users = us.id
        join 
            users uss on tm.id_deu_like = uss.id
        join 
            sub_habilidade sh on ush.id_sub_habilidade = sh.id
        join 
            sub_habilidade shh on ushh.id_sub_habilidade = shh.id
        where
            tm.id_liked = $1 and match=true
        group by
            tm.id_deu_like,
            uss.first_name,
            uss.last_name,
            tm.id_liked,
            sh.nome,
            shh.nome,
            us.first_name,
            us.last_name;
    "#;

    let result = sqlx::query_as::<_, (i32, String, String, i32, String, String)>(query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await;


    match result {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => {
            eprintln!("Erro ao buscar matches: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao buscar matches")
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = "postgres://swapp_user:swappsenha@swapp-db.cvm0qsuik7kf.us-east-1.rds.amazonaws.com:5432/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
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

    #[derive(OpenApi)]
    #[openapi(
        paths(
            inserir_dados,
            obter_dados,
            obter_tudo,
            deletar_dados,
            atualizar_dados, 
            adicionar_like,
            buscar_likes,
            atualizar_match,
            buscar_matches,
            buscar_historico,
            atualizar_historico,
            excluir_historico,
            excluir_match,
            obter_habilidades,
            obter_sub_habilidades_habilidades,
            obter_sub_habilidades,
            obter_habilidades_sub_habilidades
        ),
        components(
            schemas(Dados),
            schemas(Habilidades)
        ),
        tags(
            (name = "API - Swapp", description = "APIs para gerenciamento de habilidades de usuários e matches")
        )
    )]
struct ApiDoc;

    let openapi = ApiDoc::openapi();
    use actix_cors::Cors;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin() // Permite requisições de qualquer origem
                    .allow_any_method() // Permite qualquer método HTTP (GET, POST, etc.)
                    .allow_any_header() // Permite qualquer header
                    .max_age(3600),     // Configura o cache do CORS para 1 hora
            )
            .route("/habilidades", web::get().to(obter_habilidades))
            .route("/sub_habilidades", web::get().to(obter_sub_habilidades))
            .route("/sub_habilidade_habilidade/{id}", web::get().to(obter_sub_habilidades_habilidades))
            .route("/habilidade_sub_habilidade/{id}", web::get().to(obter_habilidades_sub_habilidades))
            .route("/inserir", web::post().to(inserir_dados))
            .route("/obter/{id_users}", web::get().to(obter_dados))
            .route("/obter_tudo", web::get().to(obter_tudo))
            .route("/deletar/{id_users}", web::delete().to(deletar_dados))
            .route("/atualizar", web::put().to(atualizar_dados))
            .route("/match/add_like", web::post().to(adicionar_like))
            .route("/match/buscar_likes/{id}", web::get().to(buscar_likes))
            .route("/match/buscar_meus_likes/{id}", web::get().to(buscar_meus_likes))
            .route("/match", web::put().to(atualizar_match))
            .route("/match/{id_liked}", web::get().to(buscar_matches))
            .route("/match/delete", web::delete().to(excluir_match))
            .route("/historico/add", web::post().to(atualizar_historico))
            .route("/historico/{id}", web::get().to(buscar_historico))
            .route("/historico/delete", web::delete().to(excluir_historico))
            .route("/match/all/{id}", web::get().to(all_dados_match))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

}
