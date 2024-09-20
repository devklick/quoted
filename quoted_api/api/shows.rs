use http::Method;
use quoted_api::api_response::{ErrorResult, SuccessResult, VercelResponse};
use quoted_api_models::{
    page::{GetShowsRequest, PagedData, PagedRequest},
    show::Show,
};
use quoted_db::{enable_query_logging, get_default_connection};
use quoted_db_entity as entity;
use sea_orm::{ConnectionTrait, FromQueryResult};
use sea_orm::{EntityTrait, QueryOrder, QuerySelect, QueryTrait};
use vercel_runtime::{run, Body, Error, Request, Response};

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
        None => PagedRequest::<GetShowsRequest>::default(),
        Some(query) => match serde_urlencoded::from_str::<PagedRequest<GetShowsRequest>>(query) {
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
        .limit(query_params.limit)
        .offset(query_params.limit * (query_params.page - 1))
        .as_query()
        .to_owned();

    let stmt = db.get_database_backend().build(&query);

    let shows = Show::find_by_statement(stmt).all(&db).await;

    if let Ok(shows) = shows {
        println!("Returning result");
        return SuccessResult::ok(PagedData::<Show>::new(
            query_params.page,
            query_params.limit,
            shows,
        ))
        .vercel();
    }
    println!("DB Returned error");
    return ErrorResult::server_error("Error finding shows").vercel();
}
