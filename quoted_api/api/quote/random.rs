use http::Method;
use quoted_api::api_response::{ErrorResult, SuccessResult, VercelResponse};
use quoted_api_models::quote::{RandomQuoteRequest, RandomQuoteResponse};
use quoted_db::get_default_connection;
use quoted_db_entity as entity;
use sea_orm::{
    entity::*,
    sea_query::{Func, SimpleExpr},
    EntityTrait, QueryFilter, QuerySelect, QueryTrait,
};
use sea_orm::{ConnectionTrait, FromQueryResult};
use serde::{Deserialize, Serialize};
use vercel_runtime::{run, Body, Error, Request, Response};

#[derive(Serialize, Deserialize)]
struct MyResponse {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    println!("Parsing query params");
    let query_params = match req.uri().query() {
        None => RandomQuoteRequest::default(),
        Some(query) => match serde_urlencoded::from_str::<RandomQuoteRequest>(query) {
            Ok(query) => query,
            Err(_) => return ErrorResult::bad_request("Invalid query parameters").vercel(),
        },
    };

    println!("{:#?}", query_params);

    println!("Building query");
    let mut query = entity::quote::Entity::find()
        .inner_join(entity::episode::Entity)
        .inner_join(entity::season::Entity)
        .inner_join(entity::show::Entity)
        .inner_join(entity::character::Entity);

    if let Some(show_name) = &query_params.show_name {
        query = query.filter(entity::show::Column::Name.eq(show_name));
    }
    if let Some(season_no) = &query_params.season_no {
        query = query.filter(entity::season::Column::SeasonNo.eq(*season_no));
    }
    if let Some(episode_no) = &query_params.episode_no {
        query = query.filter(entity::episode::Column::EpisodeNo.eq(*episode_no));
    }
    if let Some(character_name) = &query_params.character_name {
        query = query.filter(entity::character::Column::Name.eq(character_name));
    }

    let query = query
        .select_only()
        .column_as(entity::show::Column::Name, "show_name")
        .column_as(entity::character::Column::Name, "character_name")
        .column(entity::season::Column::SeasonNo)
        .column(entity::episode::Column::EpisodeNo)
        .column_as(entity::episode::Column::Name, "episode_name")
        .column_as(entity::quote::Column::Value, "quote_text")
        .as_query()
        .to_owned()
        .order_by_expr(
            SimpleExpr::FunctionCall(Func::random()),
            sea_orm::Order::Asc,
        )
        .to_owned();

    let stmt = db.get_database_backend().build(&query);

    println!("Executing query");
    let quote = RandomQuoteResponse::find_by_statement(stmt).one(&db).await;

    if let Ok(quote) = quote {
        if let Some(quote) = quote {
            println!("Returning result");
            return SuccessResult::ok(quote).vercel();
        }
        println!("No match found");
        return ErrorResult::bad_request("Quote not found").vercel();
    }
    println!("DB Returned error");
    return ErrorResult::server_error("Error finding random quote").vercel();
}
