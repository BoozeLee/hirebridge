use std::collections::HashMap;
use warp::{Filter, Reply};
use crate::db::Database;
use crate::analytics::AnalyticsEngine;

pub async fn start_server(db_path: String, addr: std::net::SocketAddr) {
    let db_filter = warp::any().map(move || db_path.clone());

    let api_stats = warp::path!("api" / "stats")
        .and(db_filter.clone())
        .map(|dp: String| {
            let db = Database::new(&dp).unwrap();
            warp::reply::json(&AnalyticsEngine::new(db).aggregate_pass_rate())
        });

    let api_candidates = warp::path!("api" / "candidates")
        .and(db_filter.clone())
        .map(|dp: String| {
            let db = Database::new(&dp).unwrap();
            let candidates = db.get_all_candidates().unwrap_or_default();
            warp::reply::json(&candidates)
        });

    let api_candidate = warp::path!("api" / "candidate" / String)
        .and(db_filter.clone())
        .map(|id: String, dp: String| {
            let db = Database::new(&dp).unwrap();
            let analytics = AnalyticsEngine::new(db);
            warp::reply::json(&analytics.candidate_scorecard(&id))
        });

    let api_github = warp::path!("api" / "github" / "sync")
        .and(warp::post())
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let candidate = p.get("candidate").cloned().unwrap_or_default();
            warp::reply::json(&serde_json::json!({
                "candidate": candidate, "status": "synced", "commits": 0, "prs": 0
            }))
        });

    let api = api_stats.or(api_candidates).or(api_candidate).or(api_github);
    let web = warp::fs::dir("web/");
    let routes = api.or(web);

    warp::serve(routes).run(addr).await;
}