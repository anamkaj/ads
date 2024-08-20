use crate::server::handler::get_stat_client::ReqParam;
use axum::extract::Query;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres};

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ClientStatistics {
    pub campaign_data: Statistics,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Statistics {
    pub id: i64,
    pub date: NaiveDate,
    pub clicks: i64,
    pub cost: f64,
    pub ctr: f64,
    pub ad_network_type: String,
    pub avg_impression_position: f64,
    pub avg_cpc: f64,
    pub avg_pageviews: f64,
    pub bounce_rate: f64,
    pub campaign_id: i64,
    pub client_login: String,
    pub uniq_key_day: String,
}

pub async fn get_statistics(
    pool: Pool<Postgres>,
    param: &Query<ReqParam>,
) -> Result<Vec<Statistics>, Box<dyn std::error::Error>> {
    let get_stat = "SELECT *
        FROM campaign_data
        WHERE date BETWEEN $1 AND $2
        AND client_login=$3;";

    let data: Vec<Statistics> = sqlx::query_as(&get_stat)
        .bind(&param.date_from)
        .bind(&param.date_to)
        .bind(&param.client_login)
        .fetch_all(&pool)
        .await?;

    Ok(data)
}
