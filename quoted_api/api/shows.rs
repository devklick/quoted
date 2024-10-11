use http::Method;
use quoted_api::{
    api_response::{ErrorResult, SuccessResult, VercelResponse},
    setup::setup,
};
use quoted_api_models::show::{GetShowsRequest, GetShowsResponse, GetShowsResponseItem};
use quoted_db::get_default_connection;
use quoted_db_entity as entity;
use sea_orm::{ConnectionTrait, FromQueryResult};
use sea_orm::{EntityTrait, QueryOrder, QuerySelect, QueryTrait};
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

    println!("Building query");
    let query = entity::show::Entity::find()
        .select_only()
        .column(entity::show::Column::Name)
        .order_by_asc(entity::show::Column::Name)
        .limit(query_params.limit + 1)
        .offset(query_params.limit * (query_params.page - 1))
        .as_query()
        .to_owned();

    let stmt = db.get_database_backend().build(&query);

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
    println!("DB Returned error");
    return ErrorResult::server_error("Error finding shows").vercel();
}
