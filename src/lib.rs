// Autor: Acosta Quintana, Lautaro
use colored::Colorize;
use dbdata::DBData;
use fake::{Fake, Faker};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub mod datasets;
pub mod db_tablas;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::Mutex;

/// Establece una conexión con la base de datos utilizando el URL definido en las variables del
/// ambiente.
pub async fn conectar_con_bd() -> anyhow::Result<Pool<MySql>> {
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
pub async fn cargar_tabla<T>(muestras: usize, pool: &Pool<MySql>) -> anyhow::Result<Vec<T>>
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
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
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

    eprintln!("[{}] {}", chrono::Local::now().format("%H:%M:%S"), msg)
}

pub static CONTADOR: Lazy<Mutex<ContadorGlobal>> = Lazy::new(|| Mutex::new(ContadorGlobal::new()));

pub struct ContadorGlobal {
    total: usize,
    subtotals: HashMap<Notificacion, usize>,
}

impl ContadorGlobal {
    pub fn new() -> Self {
        Self {
            total: 0,
            subtotals: HashMap::new(),
        }
    }
    pub fn increment(&mut self, notif: Notificacion) {
        self.total += 1;
        *self.subtotals.entry(notif).or_insert(0) += 1;
    }
    pub fn get_total(&self) -> usize {
        self.total
    }

    pub fn get_subtotals(&self) -> HashMap<Notificacion, usize> {
        self.subtotals.clone()
    }
}

pub async fn incrementar_contador(category: Notificacion) {
    let mut counter = CONTADOR.lock().await;
    counter.increment(category);
}

pub async fn generar_reporte() {
    let counter = CONTADOR.lock().await;
    let total = counter.total;
    let subtotales = counter.subtotals.clone();
    let mut info = 0;
    let mut warn = 0;
    for (&k, &v) in subtotales.iter() {
        match k {
            Notificacion::INFO => info = v,
            Notificacion::WARN => warn = v,
            _ => (),
        }
    }
    let msg = format!(
        "\nLa cantidad de registros que se intentaron cargar fueron: {total}
    - {:<6} Se cargaron exitosamente. ({:>6.4}%)
    - {:<6} Fueron rechazados.        ({:>6.4}%)",
        info,
        ((info as f64 / total as f64) * 100.0)
            .to_string()
            .bright_green()
            .bold(),
        warn,
        ((warn as f64 / total as f64) * 100.0)
            .to_string()
            .bright_yellow()
            .bold()
    );
    eprintln!("{msg}");
}
