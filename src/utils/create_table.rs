use sqlx::{Pool, Postgres};

pub async fn create_table(pool: &Pool<Postgres>) -> Result<String, Box<dyn std::error::Error>> {
    let check_table: &str = "SELECT EXISTS (
    SELECT 1
    FROM pg_tables
    WHERE schemaname = 'public'
    AND tablename = 'client_list'
);";

    let row: (bool,) = sqlx::query_as(&check_table).fetch_one(pool).await?;
    let table_exists: bool = row.0;

    if table_exists {
        return Ok("Table already exists".to_string());
    }

    if !table_exists {
        let client_list: &str = "
            CREATE TABLE public.client_list (
            id bigserial NOT NULL,
            archived varchar NULL,
            client_id int8 NOT NULL,
            created_at varchar NOT NULL,
            login varchar NOT NULL,
            client_info varchar NOT NULL,
            CONSTRAINT client_list_client_id_key UNIQUE (client_id),
            CONSTRAINT client_list_pkey PRIMARY KEY (id));";

        sqlx::query(&client_list)
            .execute(pool)
            .await
            .expect("Error creating table");

        let bonuses: &str = "
            CREATE TABLE public.bonuses (
            id bigserial NOT NULL,
            awaiting_bonus int8 NULL,
            awaiting_bonus_without_nds int8 NULL,
            fk_client_list_client_id bigserial NOT NULL,
            CONSTRAINT bonuses_fk_client_list_client_id_key UNIQUE (fk_client_list_client_id),
            CONSTRAINT bonuses_id_key UNIQUE (id),
            CONSTRAINT bonuses_pkey PRIMARY KEY (id, fk_client_list_client_id),
            CONSTRAINT bonuses_fk_client_list_client_id_fkey FOREIGN KEY (fk_client_list_client_id) REFERENCES public.client_list(client_id));";

        sqlx::query(&bonuses)
            .execute(pool)
            .await
            .expect("Error creating table");

        let campaign_data: &str = "
            CREATE TABLE public.campaign_data (
            id bigserial NOT NULL,
            date date NOT NULL,
            clicks int8 NOT NULL,
            cost float8 NOT NULL,
            ctr float8 NOT NULL,
            ad_network_type varchar(255) NOT NULL,
            avg_impression_position float8 NOT NULL,
            avg_cpc float8 NOT NULL,
            avg_pageviews float8 NOT NULL,
            bounce_rate float8 NOT NULL,
            campaign_id int8 NOT NULL,
            client_login varchar(255) NOT NULL,
            uniq_key_day varchar NOT NULL,
            CONSTRAINT campaign_data_pkey PRIMARY KEY (id),
            CONSTRAINT campaign_data_uniq_key_day_key UNIQUE (uniq_key_day));";

        sqlx::query(&campaign_data)
            .execute(pool)
            .await
            .expect("Error creating table");
    }

    Ok("Table created successfully!".to_string())
}
