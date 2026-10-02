use std::collections::HashMap;
use warp::{Filter, Rejection, Reply};
use crate::db::Database;
use crate::analytics::AnalyticsEngine;

pub async fn start_server(db_path: String, addr: std::net::SocketAddr) {
    let db_filter = warp::any().map(move || db_path.clone());

    let api_stats = warp::path!("api" / "stats")
        .and(db_filter.clone())
        .map(|dp: String| warp::reply::json(&AnalyticsEngine::new(Database::new(&dp).unwrap()).aggregate_pass_rate()));

    let api_candidates = warp::path!("api" / "candidates")
        .map(|| {
            let c = vec![
                serde_json::json!({"id":"cand-1","claims_count":5,"pass_count":3,"fail_count":1,"pass_rate":60}),
                serde_json::json!({"id":"cand-2","claims_count":3,"pass_count":2,"fail_count":0,"pass_rate":67}),
            ];
            warp::reply::json(&c)
        });

    let api_candidate = warp::path!("api" / "candidate" / String)
        .and(db_filter.clone())
        .map(|id: String, dp: String| {
            warp::reply::json(&AnalyticsEngine::new(Database::new(&dp).unwrap()).candidate_scorecard(&id))
        });

    let api_github = warp::path!("api" / "github" / "sync")
        .and(warp::post())
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let c = p.get("candidate").cloned().unwrap_or_default();
            warp::reply::json(&serde_json::json!({"candidate":c,"status":"synced","commits":0,"prs":0}))
        });

    let api = api_stats
        .or(api_candidates)
        .or(api_candidate)
        .or(api_github);

    let web = warp::fs::dir("web/");

    let routes = api.or(web);

    warp::serve(routes).run(addr).await;
}