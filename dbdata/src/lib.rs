use sqlx::{MySql, Pool};
use std::error::Error;

#[allow(async_fn_in_trait)]
/// Representa un dato insertable en la base de datos.
pub trait DBData {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>>;
}
