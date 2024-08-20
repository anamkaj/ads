use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::Pool;
use sqlx::Postgres;
use crate::direct::db::get_client_list::List;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientList {
    pub result: ClientResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientResult {
    #[serde(rename = "Clients")]
    pub clients: Vec<Client>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    #[serde(rename = "Archived")]
    pub archived: String,
    #[serde(rename = "ClientId")]
    pub client_id: i64,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "Bonuses")]
    pub bonuses: Bonuses,
    #[serde(rename = "Login")]
    pub login: String,
    #[serde(rename = "ClientInfo")]
    pub client_info: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Bonuses {
    #[serde(rename = "AwaitingBonus")]
    pub awaiting_bonus: i64,
    #[serde(rename = "AwaitingBonusWithoutNds")]
    pub awaiting_bonus_without_nds: i64,
}

impl ClientList {
    pub async fn get_client_list(
        pool: Pool<Postgres>,
    ) -> Result<Vec<List>, Box<dyn std::error::Error>> {
        dotenv().ok();
        let token: String = std::env::var("ACCESS_TOKEN").unwrap();

        let client: reqwest::Client = reqwest::Client::builder().build()?;

        let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Authorization", token.parse()?);

        let json: serde_json::Value = serde_json::json!({
            "method": "get",
            "params": {
                "SelectionCriteria": {
                    "Archived": "NO"
                },
                "FieldNames": [
                    "ClientId",
                    "ClientInfo",
                    "Login",
                    "Archived",
                    "CreatedAt",
                    "Bonuses"
                ]
            }
        });

        let request = client
            .request(
                reqwest::Method::POST,
                "https://api.direct.yandex.com/json/v501/agencyclients",
            )
            .headers(headers)
            .body(json.to_string());

        let response: reqwest::Response = request.send().await?;
        let body: String = response.text().await?;

        let json_des: ClientList = serde_json::from_str(&body).expect("Ошибка серелизации JSON");

        let insert_client_list = "INSERT INTO client_list 
        (client_id, login, created_at, client_info, archived)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (client_id) 
        DO UPDATE SET
        login = EXCLUDED.login,
        created_at = EXCLUDED.created_at,
        client_info = EXCLUDED.client_info,
        archived = EXCLUDED.archived;
        ";

        let insert_client_list_bonus = "
        INSERT INTO bonuses 
            ( awaiting_bonus, 
            awaiting_bonus_without_nds, 
            fk_client_list_client_id
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (fk_client_list_client_id) 
            DO UPDATE SET
            awaiting_bonus = EXCLUDED.awaiting_bonus,
            awaiting_bonus_without_nds = EXCLUDED.awaiting_bonus_without_nds;
            ";

        for client in &json_des.result.clients {
            let _ = sqlx::query(&insert_client_list)
                .bind(&client.client_id)
                .bind(&client.login)
                .bind(&client.created_at)
                .bind(&client.client_info)
                .bind(&client.archived)
                .execute(&pool)
                .await?;

            let _ = sqlx::query(&insert_client_list_bonus)
                .bind(&client.bonuses.awaiting_bonus)
                .bind(&client.bonuses.awaiting_bonus_without_nds)
                .bind(&client.client_id)
                .execute(&pool)
                .await?;
        }

        let data: Vec<List> = List::get_client_list(pool).await?;

        Ok(data)
    }
}
