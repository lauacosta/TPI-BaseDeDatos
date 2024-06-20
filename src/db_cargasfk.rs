//#![allow(dead_code)]
use crate::db_tablas::*;
use fake::faker::lorem::en::*;
use fake::faker::time::en::Date;
use fake::Fake;
use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use sqlx::types::time::Date;
use sqlx::{MySql, Pool};
use std::error::Error;
use std::sync::Mutex;
use time::Duration;

static GLOBAL_RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| {
    Mutex::new(StdRng::from_entropy())
});

// FIXME: Ver como usar macros para reducir el codigo duplicado.
pub async fn cargar_asegura_a(
    profesores: &[Profesores],
    seguros: &[Seguros],
    beneficiarios: &[Beneficiarios],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for s in seguros {
        let prof = profesores.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let beneficiario = beneficiarios.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let capital_asegurado = GLOBAL_RNG.lock().unwrap().gen_range(100000.0..1000000.0);
        let fecha_ingreso: Date = Date().fake();
        match sqlx::query!(
            r#"
            insert into AseguraA(
                DNIProfesor, DNIBeneficiario, CodigoCompania, CapitalAsegurado, FechaIngreso
            )
            values (?,?,?,?,?)
            "#,
            prof.dni,
            beneficiario.dni,
            s.codigo_compania,
            capital_asegurado,
            fecha_ingreso
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_reside_en(
    profesores: &[Profesores],
    direcciones: &[Direcciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for p in profesores {
        let dir = direcciones.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let mut rng = GLOBAL_RNG.lock()?;
        let vive_en_departamento = rng.gen::<bool>();
        let piso = if vive_en_departamento {
            Some(rng.gen_range(1..1000))
        } else {
            None
        };
        let habitacion: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let departamento = if vive_en_departamento {
            Some(habitacion[rng.gen_range(0..habitacion.len())])
        } else {
            None
        };

        match sqlx::query!(
            r#"
            insert into ResideEn(
                DNIProfesor, CodigoPostal, Calle, Numero, Piso, Departamento
            )
            values (?,?,?,?,?,?)
            "#,
            p.dni,
            dir.codigo_postal,
            dir.calle,
            dir.numero,
            piso,
            departamento
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}
pub async fn cargar_cumple_cargo(
    profesores: &[Profesores],
    declaraciones_cargo: &[DeclaracionesDeCargo],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for d in declaraciones_cargo {
        let prof = profesores.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        match sqlx::query!(
            r#"
            insert into CumpleCargo(
                DNIProfesor, IDDeclaracion
            )
            values (?,?)
            "#,
            prof.dni,
            d.id_declaracion
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_percibe_en(
    percepciones: &[Percepciones],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for p in percepciones {
        let prof = profesores.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let desde: Date = Date().fake();
        let estado_percepcion = ["Suspendido", "Percibiendo"]
            .choose(&mut *GLOBAL_RNG.lock()?)
            .unwrap()
            .to_string();

        match sqlx::query!(
            r#"
            insert into PercibeEn(
                DNI, InstitucionCaja, Tipo, EstadoPercepcion, Desde
            )
            values (?,?,?,?,?)
            "#,
            prof.dni,
            p.institucion_caja,
            p.tipo,
            estado_percepcion,
            desde
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_participo_en_reunion(
    reuniones: &[ReunionesCientificas],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for r in reuniones {
        let prof = profesores.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let participacion: String = Word().fake();
        match sqlx::query!(
            r#"
            insert into ParticipoEnReunion(DNIProfesor, Titulo, Fecha, Participacion)
            values (?,?,?,?)
            "#,
            prof.dni,
            r.titulo,
            r.fecha,
            participacion,
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_publico_publicaciones(
    publicaciones: &[Publicaciones],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for p in publicaciones {
        let prof = profesores.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        match sqlx::query!(
            r#"
            insert into PublicoPublicacion(IDPublicacion, DNIProfesor)
            values (?,?)
            "#,
            p.id_publicacion,
            prof.dni
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_referencias_bibliograficas(
    publicaciones: &[Publicaciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for _ in 1..GLOBAL_RNG.lock()?.gen_range(1..publicaciones.len()) {
        let p1 = publicaciones.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let p2 = publicaciones.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        match sqlx::query!(
            r#"
            insert into ReferenciaBibliografica (IDFuente, IDCitador)
            values (?,?)
            "#,
            p1.id_publicacion,
            p2.id_publicacion,
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_realizo_actividad(
    actividades: &[ActividadesExtensionUniversitaria],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let act = actividades.choose(&mut *GLOBAL_RNG.lock()?).unwrap();

        let acciones: String = Word().fake();
        // FIXME: Esto obviamente es muy ingenuo.
        let desde: Date = Date().fake();
        let hasta = desde + Duration::days(365);
        let dedicacion = GLOBAL_RNG.lock()?.gen_range(1..8);

        match sqlx::query!(
            r#"
            insert into RealizoActividad (IDActividad, DNIProfesor, Acciones, Dedicacion, Hasta, Desde)
            values (?,?,?,?,?,?)
            "#,
            act.id_actividad,
            prof.dni,
            acciones,
            dedicacion,
            hasta,
            desde
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_participa_en_investigacion(
    actividades: &[ActividadesInvestigacion],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let mut rng = GLOBAL_RNG.lock()?;
        let act = actividades.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let desde: Date = Date().fake();

        // FIXME: Esto obviamente es muy ingenuo.
        let hasta = if rng.gen::<bool>() {
            Some(desde + Duration::days(365))
        } else {
            None
        };
        let dedicacion = rng.gen_range(1..8);

        match sqlx::query!(
            r#"
            insert into ParticipaEnInvestigacion (IDInvestigacion, DNIProfesor, Desde, Hasta, Dedicacion)
            values (?,?,?,?,?)
            "#,
            act.id_investigacion,
            prof.dni,
            desde,
            hasta,
            dedicacion
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_atendio_a(
    curso_conferencia: &[CursosOConferencias],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let c = curso_conferencia.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
        let desde: Date = Date().fake();
        let hasta = match c.tipo.as_str() {
            "Curso" => Some(desde + Duration::days(30)),
            "Conferencia" => Some(desde + Duration::days(1)),
            _ => None,
        };

        // FIXME: Esto obviamente es muy ingenuo.
        match sqlx::query!(
            r#"
            insert into AtendioA (Nombre, Institucion, DNI, Desde, Hasta)
            values (?,?,?,?,?)
            "#,
            c.nombre,
            c.institucion,
            prof.dni,
            desde,
            hasta
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_posee_titulo(
    titulos: &[Titulos],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let t = titulos.choose(&mut *GLOBAL_RNG.lock()?).unwrap();

        // FIXME: Esto obviamente es muy ingenuo.
        let desde: Date = Date().fake();
        let hasta = desde + Duration::days(365 * 5);
        match sqlx::query!(
            r#"
            insert into PoseeTitulo (DNI, Institucion, Nivel, Titulo, Desde, Hasta)
            values (?,?,?,?,?,?)
            "#,
            prof.dni,
            t.institucion,
            t.nivel,
            t.titulo,
            desde,
            hasta
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_conoce_idiomas(
    idiomas: &[&str],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        // FIXME: Encontrar una mejor manera de que cada profesor conozca al menos dos idiomas.
        for _ in 1..=GLOBAL_RNG.lock()?.gen_range(1..3) {
            let idioma = idiomas.choose(&mut *GLOBAL_RNG.lock()?).unwrap();
            let certificacion: String = Word().fake();
            let institucion: String = Word().fake();
            let nivel: String = Word().fake();
            match sqlx::query!(
                r#"
            insert into ConoceIdioma (DNIProfesor, NombreIdioma, Certificacion, Institucion, Nivel)
            values (?,?,?,?,?)
            "#,
                prof.dni,
                idioma,
                certificacion,
                institucion,
                nivel
            )
            .execute(pool)
            .await
            {
                Ok(_) => continue,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    continue;
                }
            };
        }
    }
    Ok(())
}

pub async fn cargar_idiomas(idiomas: &[&str], pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
    let row_count: i64 = sqlx::query_scalar("select count(*) FROM Idiomas")
        .fetch_one(pool)
        .await?;

    if row_count == 0 {
        for i in idiomas {
            match sqlx::query!(
                r#"
            insert into Idiomas (Nombre) values (?)
            "#,
                i
            )
            .execute(pool)
            .await
            {
                Ok(_) => continue,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    continue;
                }
            };
        }
    }
    Ok(())
}
