mod handlers;
mod routes;
mod services;
mod types;

use routes::setup_routes;
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = setup_routes();
    router.run(req, env).await
}
