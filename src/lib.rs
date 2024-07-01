// Autor: Acosta Quintana, Lautaro
use colored::Colorize;
use dbdata::DBData;
use fake::{Fake, Faker};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::error::Error;

pub mod datasets;
pub mod db_cargasfk;
pub mod db_tablas;

pub const BIND_LIMIT: usize = 65543;

/// Establece una conexión con la base de datos utilizando el URL definido en las variables del
/// ambiente.
pub async fn conectar_con_bd() -> Result<Pool<MySql>, Box<dyn Error>> {
    dotenvy::dotenv().expect("Archivo .env no pudo se encontrado.");
    let db_url =
        std::env::var("DATABASE_URL").expect("No se pudo encontrar la variable 'DATABASE_URL'.");
    Ok(MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(4))
        .connect(&db_url)
        .await?)
}

/// Genera e inserta dentro de la base de datos los datos generados completamente de manera
/// pseudoaleatoria.
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
    notificar_carga(Notificacion::INFO, nombre_tabla);
    Ok(tablas)
}

/// Modela los distintos tipos de notificación dentro del programa.
pub enum Notificacion {
    INFO,
    WARN,
    ERROR,
}

/// Muestra un mensaje a traves de STDERR.
pub fn notificar_carga(tipo: Notificacion, data: &str) {
    let msg = match tipo {
        Notificacion::INFO => format!(
            "{} Se ha cargado {} correctamente!",
            "[INFO]".bright_green().bold(),
            data.bright_green().bold()
        ),
        Notificacion::WARN => format!("{} {}", "[WARN]".bright_yellow().bold(), data),
        Notificacion::ERROR => format!("{} {}", "[ERROR]".bright_red().bold(), data),
    };

    eprintln!(
        "[{}] {}",
        chrono::Local::now().format("%H:%M:%S").to_string(),
        msg
    )
}
