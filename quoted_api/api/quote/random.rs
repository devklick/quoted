use http::Method;
use quoted_api::{
    api_response::{ErrorResult, SuccessResult, VercelResponse},
    models::quote_models::{RandomQuoteDBResult, RandomQuotePartDBResult},
    setup::setup,
};
use quoted_api_models::quote::RandomQuoteRequest;
use quoted_db::{enable_query_logging, get_default_connection};
use quoted_db_entity::{self as entity};
use sea_orm::{
    entity::*,
    sea_query::{Func, SimpleExpr},
    DatabaseBackend, EntityTrait, QueryFilter, QuerySelect, QueryTrait, Statement,
};
use sea_orm::{ConnectionTrait, FromQueryResult};
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
    let db_backend = db.get_database_backend();
    enable_query_logging();

    println!("Parsing query params");
    let query_params = match req.uri().query() {
        None => RandomQuoteRequest::default(),
        Some(query) => match serde_urlencoded::from_str::<RandomQuoteRequest>(query) {
            Ok(query) => query,
            Err(_) => return ErrorResult::bad_request("Invalid query parameters").vercel(),
        },
    };

    println!("{:#?}", query_params);

    println!("Building quote query");
    let query = build_quote_query(query_params, db_backend);

    println!("Executing quote query");
    let quote = match RandomQuoteDBResult::find_by_statement(query).one(&db).await {
        Err(e) => {
            println!(
                "DB Returned error when looking for quote\n{}",
                e.sql_err().unwrap().to_string()
            );
            return ErrorResult::server_error("Error finding random quote").vercel();
        }
        Ok(r) => {
            if r.is_none() {
                println!("No quotes match found");
                return ErrorResult::bad_request("Quote not found").vercel();
            }
            r.unwrap()
        }
    };

    println!("Building quote parts query");
    let query = build_quote_part_query(quote.quote_id, db_backend);

    println!("Executing quote parts query");
    let quote_parts = match RandomQuotePartDBResult::find_by_statement(query)
        .all(&db)
        .await
    {
        Err(e) => {
            println!(
                "DB Returned error when looking for quote parts\n{}",
                e.sql_err().unwrap().to_string()
            );
            return ErrorResult::server_error("Error finding random quote").vercel();
        }
        Ok(r) => r,
    };
    let mut response = quote.to_api_response();
    response.parts = quote_parts.iter().map(|qp| qp.to_api_response()).collect();

    SuccessResult::ok(response).vercel()
}

fn build_quote_query(query_params: RandomQuoteRequest, db_backend: DatabaseBackend) -> Statement {
    // Start by wiring up the required joins
    let mut query = entity::quote::Entity::find()
        .inner_join(entity::episode::Entity)
        .inner_join(entity::season::Entity)
        .inner_join(entity::show::Entity)
        .inner_join(entity::quote_part::Entity)
        .join(
            sea_orm::JoinType::InnerJoin,
            entity::character::Relation::QuotePart.def().rev(),
        );

    // Conditionally apply any filters based on query params
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

    // Add the columns to be selected
    let query = query
        .select_only()
        .column_as(entity::quote::Column::Id, "quote_id")
        .column_as(entity::show::Column::Name, "show_name")
        .column_as(entity::character::Column::Name, "character_name")
        .column(entity::season::Column::SeasonNo)
        .column_as(entity::season::Column::Name, "season_name")
        .column(entity::episode::Column::EpisodeNo)
        .column_as(entity::episode::Column::Name, "episode_name")
        .as_query()
        .to_owned()
        .order_by_expr(
            SimpleExpr::FunctionCall(Func::random()),
            sea_orm::Order::Asc,
        )
        .to_owned();

    // build the query
    db_backend.build(&query)
}

fn build_quote_part_query(quote_id: i32, db_backend: DatabaseBackend) -> Statement {
    let query = entity::quote_part::Entity::find()
        .inner_join(entity::character::Entity)
        .select_only()
        .column(entity::quote_part::Column::QuoteId)
        .column_as(entity::quote_part::Column::OrderNo, "order")
        .column_as(entity::quote_part::Column::Value, "quote_text")
        .column_as(entity::character::Column::Name, "character_name")
        .filter(entity::quote_part::Column::QuoteId.eq(quote_id))
        .as_query()
        .to_owned();

    db_backend.build(&query)
}
