use colored::Colorize;
use dbdata::DBData;
use fake::{Fake, Faker};
use once_cell::sync::Lazy;
use rand::{rngs::StdRng, SeedableRng};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{error::Error, sync::Mutex};

pub mod db_cargasfk;
pub mod db_tablas;

pub const BIND_LIMIT: usize = 65543;

static GLOBAL_RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::from_entropy()));

pub async fn conectar_con_bd() -> Result<Pool<MySql>, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_url =
        std::env::var("DATABASE_URL").expect("No se pudo encontrar la variable 'DATABASE_URL'");
    Ok(MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(4))
        .connect(&db_url)
        .await?)
}

pub async fn cargar_tabla<T>(muestras: usize, pool: &Pool<MySql>) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DBData + fake::Dummy<fake::Faker>,
{
    let mut tablas: Vec<T> = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let registro: T = Faker.fake();
        registro.insertar_en_db(pool).await?;
        tablas.push(registro);
    }

    let nombre_tabla = std::any::type_name::<T>().rsplit("::").next().unwrap();
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".to_string().bright_green(),
        nombre_tabla.bright_green()
    );
    Ok(tablas)
}
