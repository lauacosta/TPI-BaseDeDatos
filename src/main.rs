use carga_datos::{Direcciones, Empleadores, Profesores};
use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::thread_rng;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_url =
        std::env::var("DATABASE_URL").expect("No se pudo encontrar la variable 'DATABASE_URL'");
    let pool = MySqlPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    //let _ = sqlx::query!("INSERT INTO Idiomas (Nombre) values(?)", "Ingles")
    //    .execute(&pool)
    //    .await?;

    let direcciones: Vec<Direcciones> = (1..=10).map(|_| rand::random()).collect();
    cargar_direcciones(&direcciones, &pool).await?;
    println!("Se han cargado todas las direcciones correctamente!");

    let empleadores: Vec<Empleadores> = (1..=10)
        .map(|_| {
            let direccion = direcciones.choose(&mut thread_rng()).unwrap();
            Empleadores::new(direccion)
        })
        .collect();
    cargar_empleadores(&empleadores, &pool).await?;
    println!("Se han cargado todos los empleadores correctamente!");

    let profesores: Vec<Profesores> = (1..=10)
        .map(|_| {
            let empleador = empleadores.choose(&mut thread_rng()).unwrap();
            Profesores::new(empleador)
        })
        .collect();
    cargar_profesores(&profesores, &pool).await?;
    println!("Se han cargado todos los profesores correctamente!");

    Ok(())
}

async fn cargar_profesores(
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let str = sqlx::query!(
            r#"
            
            insert into Profesores 
            (DNI, Nombre, Apellido, FechaNacimiento, Nacionalidad, EstadoCivil, Sexo, CUIT, CUIL, CUITEmpleador)
            values (?,?,?,?,?,?,'M',?,?,?)

            "#,
            prof.dni,
            prof.nombre,
            prof.apellido,
            prof.fecha_nacimiento,
            prof.nacionalidad,
            prof.estado_civil,
            // FIXME:: MySQL Error 0100 Data truncated in 'Sexo'
            //prof.sexo,
            prof.cuit,
            prof.cuil,
            prof.cuit_empleador
        )
        .execute(pool)
        .await?;

        println!("> Rows affected = {}", str.rows_affected());
    }
    Ok(())
}
async fn cargar_empleadores(
    empleadores: &[Empleadores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for emp in empleadores {
        let str = sqlx::query!(
            r#"insert into Empleadores (CUIT_CUIL, RazonSocial, CodigoPostal, Calle, Numero, Piso, Departamento) 
values (?,?,?,?,?,?,?)"#,
            emp.cuit_cuil,
            emp.razon_social,
            emp.codigo_postal,
            emp.calle,
            emp.numero,
            emp.piso,
            emp.departamento
        )
        .execute(pool)
        .await?;

        println!("> Rows affected = {}", str.rows_affected());
    }
    Ok(())
}
async fn cargar_direcciones(
    direcciones: &[Direcciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for dir in direcciones {
        let str = sqlx::query!(
            r#"insert into Direcciones (CodigoPostal, Calle, Numero, Localidad, Provincia) 
values (?,?,?,?,?)"#,
            dir.codigo_postal,
            dir.calle,
            dir.numero,
            dir.localidad,
            dir.provincia,
        )
        .execute(pool)
        .await?;

        println!("> Rows affected = {}", str.rows_affected());
    }
    Ok(())
}
