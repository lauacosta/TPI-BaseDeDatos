use std::error::Error;
use colored::Colorize;
use dbdata::DBData;
use fake::{Fake, Faker};
use mysql_async::Pool;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool as SqlxPool};

pub mod db_cargasfk;
pub mod db_tablas;

pub const BIND_LIMIT: usize = 65543;

pub async fn conectar_con_bd() -> Result<SqlxPool<MySql>, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_url =
        std::env::var("DATABASE_URL").expect("No se pudo encontrar la variable 'DATABASE_URL'");
    Ok(MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(4))
        .connect(&db_url)
        .await?)
}

pub async fn cargar_tabla<T>(muestras: usize, pool: &SqlxPool<MySql>) -> Result<Vec<T>, Box<dyn Error>>
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

pub async fn generar_tabla<T>(
    muestras: usize,
    pool: &Pool,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: fake::Dummy<fake::Faker>,
{
    let mut tablas: Vec<T> = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        tablas.push(Faker.fake());
    }

    //let nombre_tabla = std::any::type_name::<T>().rsplit("::").next().unwrap();
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".to_string().bright_green(),
    //    nombre_tabla.bright_green()
    //);
    Ok(tablas)
}
