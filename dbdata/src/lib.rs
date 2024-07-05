use sqlx::{MySql, Pool};

#[allow(async_fn_in_trait)]
/// Trait que representa a un dato que sabe cómo insertarse en una base de datos MySQL.
pub trait DBData {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), anyhow::Error>;
}
