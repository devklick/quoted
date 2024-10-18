use std::collections::HashMap;

use http::Method;
use prelude::Expr;
use quoted_api::{
    api_response::{ErrorResult, SuccessResult, VercelResponse},
    models::quote_models::{QuoteDBResult, QuotePartDBResult},
    setup::setup,
};
use quoted_api_models::quote::{
    GetQuotesRequest, GetQuotesResponse, GetQuotesResponseItem, QuotePart,
};
use quoted_db::get_default_connection;
use quoted_db_entity as entity;
use sea_orm::{
    entity::*, sea_query::PgFunc, DatabaseBackend, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait, Statement,
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

    let query_params = match get_query_params(req) {
        Err(e) => return e.vercel(),
        Ok(p) => p,
    };

    println!("Getting DB Connection");
    let db = get_default_connection().await?;

    // need to query in two steps; get the page of quotes
    let quotes = match get_quotes(&query_params, &db).await {
        Err(e) => return e.vercel(),
        Ok(q) => q,
    };

    // get the parts and characters associated with the quote
    let quote_ids = quotes.iter().map(|q| q.quote_id).collect::<Vec<i32>>();
    let parts = match get_quote_parts(&quote_ids, &db).await {
        Err(e) => return e.vercel(),
        Ok(p) => p,
    };

    let (quotes, has_more) = process_db_results(quotes, parts, &query_params.limit);
    SuccessResult::ok(GetQuotesResponse {
        data: quotes,
        has_more,
        limit: query_params.limit,
        page: query_params.page,
    })
    .vercel()
}

fn get_query_params(req: Request) -> Result<GetQuotesRequest, ErrorResult> {
    println!("Parsing query params {:#?}", req.uri().query());

    return match req.uri().query() {
        None => return Err(ErrorResult::bad_request("Missing required parameters")),
        Some(query) => match serde_urlencoded::from_str::<GetQuotesRequest>(query) {
            Err(e) => {
                println!("{:#?}", e);
                return Err(ErrorResult::bad_request("Invalid parameters"));
            }
            Ok(query) => {
                println!("Parsed query as {:#?}", query);
                Ok(query)
            }
        },
    };
}

async fn get_quotes(
    query_params: &GetQuotesRequest,
    db: &DatabaseConnection,
) -> Result<Vec<QuoteDBResult>, ErrorResult> {
    let query = build_quote_query(query_params, &db.get_database_backend());

    return match QuoteDBResult::find_by_statement(query).all(db).await {
        Err(e) => {
            println!("Error fetching quotes, {}", e.to_string());
            return Err(ErrorResult::server_error("Error fetching quotes"));
        }
        Ok(quotes) => Ok(quotes),
    };
}

async fn get_quote_parts(
    quote_ids: &Vec<i32>,
    db: &DatabaseConnection,
) -> Result<Vec<QuotePartDBResult>, ErrorResult> {
    let query = build_quote_part_query(quote_ids, &db.get_database_backend());

    return match QuotePartDBResult::find_by_statement(query).all(db).await {
        Err(e) => {
            println!("Error fetching quote parts, {}", e.to_string());
            return Err(ErrorResult::server_error("Error fetching quote parts"));
        }
        Ok(parts) => Ok(parts),
    };
}

fn process_db_results(
    quotes: Vec<QuoteDBResult>,
    quote_parts: Vec<QuotePartDBResult>,
    limit: &u64,
) -> (Vec<GetQuotesResponseItem>, bool) {
    let mut map: HashMap<i32, GetQuotesResponseItem> = HashMap::new();

    for quote in quotes {
        map.insert(
            quote.quote_id,
            GetQuotesResponseItem {
                episode_name: quote.episode_name.clone(),
                episode_no: quote.episode_no,
                parts: vec![],
                season_name: quote.season_name.clone(),
                season_no: quote.season_no,
                show_name: quote.show_name.clone(),
            },
        );
    }

    for part in quote_parts {
        let quote = map
            .get_mut(&part.quote_id)
            .expect("Part not associated with a quote");
        quote.parts.push(QuotePart {
            character_name: part.character_name.clone(),
            order: part.order,
            quote_text: part.quote_text.clone(),
        });
    }

    let mut quotes = map.into_values().collect::<Vec<GetQuotesResponseItem>>();
    let has_more = quotes.len() > *limit as usize;
    if has_more {
        quotes = quotes.get(0..*limit as usize).unwrap().to_vec();
    }

    (quotes, has_more)
}

fn build_quote_query(query_params: &GetQuotesRequest, db_backend: &DatabaseBackend) -> Statement {
    // Start by wiring up the required joins
    let mut query = entity::quote::Entity::find()
        .inner_join(entity::episode::Entity)
        .inner_join(entity::season::Entity)
        .inner_join(entity::show::Entity);

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

    // Add the columns to be selected
    let query = query
        .select_only()
        .column_as(entity::quote::Column::Id, "quote_id")
        .column_as(entity::show::Column::Name, "show_name")
        .column(entity::season::Column::SeasonNo)
        .column_as(entity::season::Column::Name, "season_name")
        .column(entity::episode::Column::EpisodeNo)
        .column_as(entity::episode::Column::Name, "episode_name")
        .order_by_asc(entity::quote::Column::SourceId)
        .limit(query_params.limit + 1)
        .offset(query_params.limit * (query_params.page - 1))
        .as_query()
        .to_owned();

    // build the query
    db_backend.build(&query)
}

fn build_quote_part_query(quote_ids: &Vec<i32>, db_backend: &DatabaseBackend) -> Statement {
    let query = entity::quote_part::Entity::find()
        .inner_join(entity::character::Entity)
        .select_only()
        .column(entity::quote_part::Column::QuoteId)
        .column_as(entity::quote_part::Column::OrderNo, "order")
        .column_as(entity::quote_part::Column::Value, "quote_text")
        .column_as(entity::character::Column::Name, "character_name")
        .filter(Expr::eq(
            Expr::col(entity::quote_part::Column::QuoteId),
            Expr::expr(PgFunc::any(quote_ids.clone())),
        ))
        .as_query()
        .to_owned();

    db_backend.build(&query)
}
