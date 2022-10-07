mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let db = db::db::init_db();
    let customer_routes = routes::routes::customer_routes(db);

    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3004))
        .await;
}
