use http::Method;
use quoted_api::{
    api_response::{ErrorResult, SuccessResult, VercelResponse},
    setup::setup,
};
use quoted_api_models::episode::{
    GetEpisodesInSeasonRequest, GetEpisodesInSeasonResponse, GetEpisodesInSeasonResponseItem,
};
use quoted_db::get_default_connection;
use quoted_db_entity as entity;
use sea_orm::{
    prelude::Expr, sea_query::Alias, ColumnTrait, ConnectionTrait, FromQueryResult, QueryFilter,
    QueryOrder,
};
use sea_orm::{EntityTrait, QuerySelect, QueryTrait};
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
        Some(query) => match serde_urlencoded::from_str::<GetEpisodesInSeasonRequest>(query) {
            Ok(query) => query,
            Err(e) => {
                println!("{:#?}", e);
                return ErrorResult::bad_request("Invalid parameters").vercel();
            }
        },
    };

    println!("Getting DB Connection");
    let db = get_default_connection().await?;

    let query = entity::episode::Entity::find()
        .select_only()
        .column(entity::episode::Column::EpisodeNo)
        .column_as(entity::episode::Column::Name, "episode_name")
        .column_as(
            Expr::col(entity::quote::Entity)
                .count()
                .cast_as(Alias::new("integer")),
            "quote_count",
        )
        .inner_join(entity::season::Entity)
        .inner_join(entity::show::Entity)
        .left_join(entity::quote::Entity)
        .filter(entity::show::Column::Name.eq(query_params.query.show))
        .filter(entity::season::Column::SeasonNo.eq(query_params.query.season))
        .limit(query_params.limit + 1)
        .offset(query_params.limit * (query_params.page - 1))
        .order_by_asc(entity::episode::Column::EpisodeNo)
        .group_by(entity::episode::Column::EpisodeNo)
        .group_by(entity::episode::Column::Name)
        .as_query()
        .to_owned();

    let stmt = db.get_database_backend().build(&query);

    let episodes = GetEpisodesInSeasonResponseItem::find_by_statement(stmt)
        .all(&db)
        .await;

    if let Ok(mut episodes) = episodes {
        println!("Returning result");
        let has_more = episodes.len() > query_params.limit as usize;
        if has_more {
            episodes = episodes
                .get(0..query_params.limit as usize)
                .unwrap()
                .to_vec();
        }
        return SuccessResult::ok(GetEpisodesInSeasonResponse::new(
            query_params.page,
            query_params.limit,
            episodes,
            has_more,
        ))
        .vercel();
    }

    println!("DB Returned error, {}", episodes.err().unwrap());
    return ErrorResult::server_error("Error finding episodes").vercel();
}
