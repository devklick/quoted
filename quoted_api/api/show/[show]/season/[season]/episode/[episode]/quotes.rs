use std::collections::HashMap;

use http::Method;
use quoted_api::{
    api_response::{ErrorResult, SuccessResult, VercelResponse},
    models::quote_models::QuotePartDBResult,
    setup::setup,
};
use quoted_api_models::quote::{
    GetQuotesInEpisodeRequest, GetQuotesInEpisodeResponse, GetQuotesInEpisodeResponseItem,
    QuotePart,
};
use quoted_db::get_default_connection;
use quoted_db_entity as entity;
use sea_orm::{entity::*, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect, QueryTrait};
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

    println!("Parsing query params {:#?}", req.uri().query());
    let query_params = match req.uri().query() {
        None => return ErrorResult::bad_request("Missing required parameters").vercel(),
        Some(query) => match serde_urlencoded::from_str::<GetQuotesInEpisodeRequest>(query) {
            Ok(query) => query,
            Err(e) => {
                println!("{:#?}", e);
                return ErrorResult::bad_request("Invalid parameters").vercel();
            }
        },
    };

    println!("Getting DB Connection");
    let db = get_default_connection().await?;

    let query = entity::quote::Entity::find()
        .select_only()
        .column_as(entity::quote::Column::Id, "quote_id")
        .column_as(entity::character::Column::Name, "character_name")
        .column_as(entity::quote_part::Column::OrderNo, "order")
        .column_as(entity::quote_part::Column::Value, "quote_text")
        .inner_join(entity::quote_part::Entity)
        .inner_join(entity::episode::Entity)
        .inner_join(entity::season::Entity)
        .inner_join(entity::show::Entity)
        .join(
            JoinType::InnerJoin,
            entity::character::Relation::QuotePart.def().rev(),
        )
        .filter(entity::show::Column::Name.eq(query_params.query.show))
        .filter(entity::season::Column::SeasonNo.eq(query_params.query.season))
        .filter(entity::episode::Column::EpisodeNo.eq(query_params.query.episode))
        .limit(query_params.limit + 1)
        .offset(query_params.limit * (query_params.page - 1))
        .order_by_asc(entity::quote::Column::SourceId)
        .as_query()
        .to_owned();

    let stmt = db.get_database_backend().build(&query);

    let quotes = QuotePartDBResult::find_by_statement(stmt).all(&db).await;

    if let Ok(quotes) = quotes {
        println!("Returning result");

        let mut quotes = process_db_results(quotes);

        let has_more = quotes.len() > query_params.limit as usize;
        if has_more {
            quotes = quotes.get(0..query_params.limit as usize).unwrap().to_vec();
        }
        return SuccessResult::ok(GetQuotesInEpisodeResponse::new(
            query_params.page,
            query_params.limit,
            quotes,
            has_more,
        ))
        .vercel();
    }

    println!("DB Returned error, {}", quotes.err().unwrap());
    return ErrorResult::server_error("Error finding episodes").vercel();
}

fn process_db_results(quotes: Vec<QuotePartDBResult>) -> Vec<GetQuotesInEpisodeResponseItem> {
    let mut map: HashMap<i32, Vec<QuotePart>> = HashMap::new();
    for quote in quotes {
        let parts = map.entry(quote.quote_id).or_insert(vec![]);
        parts.push(QuotePart {
            character_name: quote.character_name,
            order: quote.order,
            quote_text: quote.quote_text,
        });
    }
    map.values()
        .map(|q| GetQuotesInEpisodeResponseItem { parts: q.to_vec() })
        .collect::<Vec<GetQuotesInEpisodeResponseItem>>()
}
