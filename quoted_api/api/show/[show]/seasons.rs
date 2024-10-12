use http::Method;
use quoted_api::{
    api_response::{ErrorResult, SuccessResult, VercelResponse},
    setup::setup,
};
use quoted_api_models::season::{
    GetSeasonsInShowRequest, GetSeasonsInShowResponse, GetSeasonsInShowResponseItem,
};
use quoted_db::get_default_connection;
use quoted_db_entity as entity;
use sea_orm::{ColumnTrait, ConnectionTrait, FromQueryResult, QueryFilter};
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

    println!("Parsing query params {:#?}", req.uri().query());
    let query_params = match req.uri().query() {
        None => return ErrorResult::bad_request("Missing required parameters").vercel(),
        Some(query) => match serde_urlencoded::from_str::<GetSeasonsInShowRequest>(query) {
            Ok(query) => query,
            Err(e) => {
                println!("{:#?}", e);
                return ErrorResult::bad_request("Invalid parameters").vercel();
            }
        },
    };

    println!("Getting DB Connection");
    let db = get_default_connection().await?;

    let query = entity::season::Entity::find()
        .select_only()
        .column(entity::season::Column::SeasonNo)
        .column_as(entity::season::Column::Name, "season_name")
        .inner_join(entity::show::Entity)
        .filter(entity::show::Column::Name.eq(query_params.query.show))
        .order_by_asc(entity::season::Column::SeasonNo)
        .limit(query_params.limit + 1)
        .offset(query_params.limit * (query_params.page - 1))
        .as_query()
        .to_owned();

    let stmt = db.get_database_backend().build(&query);

    let seasons = GetSeasonsInShowResponseItem::find_by_statement(stmt)
        .all(&db)
        .await;

    if let Ok(mut seasons) = seasons {
        println!("Returning result");
        let has_more = seasons.len() > query_params.limit as usize;
        if has_more {
            seasons = seasons
                .get(0..query_params.limit as usize)
                .unwrap()
                .to_vec();
        }
        return SuccessResult::ok(GetSeasonsInShowResponse::new(
            query_params.page,
            query_params.limit,
            seasons,
            has_more,
        ))
        .vercel();
    }

    println!("DB Returned error, {}", seasons.err().unwrap());
    return ErrorResult::server_error("Error finding seasons").vercel();
}
