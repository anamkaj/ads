use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]

pub struct List {
    pub archived: String,
    pub client_id: i64,
    pub created_at: String,
    pub login: String,
    pub client_info: String,
    pub awaiting_bonus: i64,
    pub awaiting_bonus_without_nds: i64,
}

impl List {
    pub async fn get_client_list(
        pool: Pool<Postgres>,
    ) -> Result<Vec<List>, Box<dyn std::error::Error>> {
        let gel_client: &str = "SELECT 
            c.client_id,
            c.login,
            c.created_at,
            c.client_info,
            c.archived,
            b.awaiting_bonus,
            b.awaiting_bonus_without_nds
            FROM client_list c
            INNER JOIN 
            bonuses b
            ON 
            c.client_id = b.fk_client_list_client_id;";

        let clients: Vec<List> = sqlx::query_as(&gel_client).fetch_all(&pool).await?;

        Ok(clients)
    }
}
