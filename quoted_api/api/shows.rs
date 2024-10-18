use http::Method;
use quoted_api::{
    api_response::{ErrorResult, SuccessResult, VercelResponse},
    setup::setup,
};
use quoted_api_models::show::{GetShowsRequest, GetShowsResponse, GetShowsResponseItem};
use quoted_db::get_default_connection;
use quoted_db_entity as entity;
use sea_orm::{
    prelude::Expr,
    sea_query::{extension::postgres::PgExpr, Alias},
    DatabaseBackend, EntityTrait, QueryOrder, QuerySelect, QueryTrait, Statement,
};
use sea_orm::{ConnectionTrait, FromQueryResult, QueryFilter};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    return match *req.method() {
        Method::GET => get(req).await,
        _ => ErrorResult::not_found().vercel(),
    };
}

async fn get(req: Request) -> Result<Response<Body>, Error> {
    println!("Request received");

    println!("Getting DB Connection");
    let db = get_default_connection().await?;

    println!("Parsing query params {:#?}", req.uri().query());
    let query_params = match req.uri().query() {
        None => GetShowsRequest::default(),
        Some(query) => match serde_urlencoded::from_str::<GetShowsRequest>(query) {
            Ok(query) => query,
            Err(_) => return ErrorResult::bad_request("Invalid query parameters").vercel(),
        },
    };

    println!("{:#?}", query_params);

    let db_backend = db.get_database_backend();

    println!("Building query");
    let stmt = build_query(&query_params, db_backend);
    let shows = GetShowsResponseItem::find_by_statement(stmt).all(&db).await;

    if let Ok(mut shows) = shows {
        println!("Returning result");
        let has_more = shows.len() > query_params.limit as usize;
        if has_more {
            shows = shows.get(0..query_params.limit as usize).unwrap().to_vec();
        }
        return SuccessResult::ok(GetShowsResponse::new(
            query_params.page,
            query_params.limit,
            shows,
            has_more,
        ))
        .vercel();
    }
    println!("DB Returned error, ${}", shows.err().unwrap());
    return ErrorResult::server_error("Error finding shows").vercel();
}

fn build_query(request: &GetShowsRequest, db_backend: DatabaseBackend) -> Statement {
    let mut query = entity::show::Entity::find()
        .select_only()
        .column(entity::show::Column::Name)
        .column_as(
            Expr::col(entity::quote::Entity)
                .count()
                .cast_as(Alias::new("integer")),
            "quote_count",
        )
        .left_join(entity::quote::Entity)
        .group_by(entity::show::Column::Name)
        .order_by_asc(entity::show::Column::Name)
        .limit(request.limit + 1)
        .offset(request.limit * (request.page - 1));

    if let Some(name) = &request.query.name {
        query = query.filter(Expr::col(entity::show::Column::Name).ilike(format!("%{}%", name)));
    }
    let query = query.as_query().to_owned();

    db_backend.build(&query)
}
