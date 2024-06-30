use crate::{db_tablas::*, notificar_carga, Notificacion::INFO, Notificacion::WARN};
use fake::{
    faker::{lorem::en::*, time::en::Date},
    Fake,
};
use once_cell::sync::Lazy;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use sqlx::{types::time::Date, MySql, Pool};
use std::error::Error;
use time::Duration;
use tokio::sync::Mutex;

pub static GLOBAL_RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::from_entropy()));

// FIXME: Ver como usar macros para reducir el codigo duplicado.
pub async fn cargar_asegura_a(
    seguros: &[Seguros],
    familiares: &[Familiares],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for s in seguros {
        let mut rng = GLOBAL_RNG.lock().await;
        let familiar = familiares.choose(&mut *rng).unwrap();
        let capital_asegurado = rng.gen_range(100000.0..1000000.0);
        let fecha_ingreso: Date = Date().fake();
        match sqlx::query!(
            r#"
            insert into AseguraA(
                DNIProfesor, DNIFamiliar , CodigoCompania, CapitalAsegurado, FechaIngreso
            )
            values (?,?,?,?,?)
            "#,
            familiar.dni_profesor,
            familiar.dni_familiar,
            s.codigo_compania,
            capital_asegurado,
            fecha_ingreso
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
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
        let mut rng = GLOBAL_RNG.lock().await;
        let dir = direcciones.choose(&mut *rng).unwrap();
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
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
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
        let mut rng = GLOBAL_RNG.lock().await;
        let prof = profesores.choose(&mut *rng).unwrap();
        let desde: Date = Date().fake();
        let estado_percepcion = ["Suspendido", "Percibiendo"]
            .choose(&mut *rng)
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
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
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
        let prof = profesores.choose(&mut *GLOBAL_RNG.lock().await).unwrap();
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
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }

    notificar_carga(INFO, "ParticipoEnReunion");
    Ok(())
}

pub async fn cargar_publico(
    publicaciones: &[Publicaciones],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for p in publicaciones {
        let prof = profesores.choose(&mut *GLOBAL_RNG.lock().await).unwrap();
        match sqlx::query!(
            r#"
            insert into Publico(IDPublicacion, DNIProfesor)
            values (?,?)
            "#,
            p.id_publicacion,
            prof.dni
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }
    Ok(())
}

pub async fn cargar_referencias_bibliograficas(
    publicaciones: &[Publicaciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    let mut rng = GLOBAL_RNG.lock().await;
    for _ in 1..rng.gen_range(1..publicaciones.len()) {
        let p1 = publicaciones.choose(&mut *rng).unwrap();
        let p2 = publicaciones.choose(&mut *rng).unwrap();
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
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }
    Ok(())
}

pub async fn cargar_realizo_actividad(
    actividades: &[ActividadesExtensionUniversitaria],
    profesores: &[Profesores],
    muestras: usize,
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    let mut rng = GLOBAL_RNG.lock().await;
    let m = rng.gen_range(0..muestras);
    for _ in 1..=m {
        let act = actividades.choose(&mut *rng).unwrap();
        let prof = profesores.choose(&mut *rng).unwrap();

        let acciones: String = Word().fake();
        // FIXME: Esto obviamente es muy ingenuo.
        let desde: Date = Date().fake();
        let hasta = desde + Duration::days(365);
        let dedicacion = rng.gen_range(1..8);

        match sqlx::query!(
            r#"
            insert into RealizoAct (IDActividad, DNIProfesor, Acciones, Dedicacion, Hasta, Desde)
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
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }
    Ok(())
}

pub async fn cargar_realiza_investigacion(
    actividades: &[ActividadesInvestigacion],
    profesores: &[Profesores],
    muestras: usize,
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    let mut rng = GLOBAL_RNG.lock().await;
    let m = rng.gen_range(0..muestras);
    for _ in 1..=m {
        let prof = profesores.choose(&mut *rng).unwrap();
        let act = actividades.choose(&mut *rng).unwrap();
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
            insert into RealizaInves (IDInvestigacion, DNIProfesor, Desde, Hasta, Dedicacion)
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
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }
    Ok(())
}

pub async fn cargar_atendio_a(
    curso_conferencia: &[CursosConferencias],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let c = curso_conferencia
            .choose(&mut *GLOBAL_RNG.lock().await)
            .unwrap();
        let desde: Date = Date().fake();
        let hasta = match c.tipo.as_str() {
            "Curso" => Some(desde + Duration::days(30)),
            "Conferencia" => Some(desde + Duration::days(1)),
            _ => None,
        };

        match sqlx::query!(
            r#"
            insert into AtendioA (NombreCurso, DNIProfesor, Desde, Hasta)
            values (?,?,?,?)
            "#,
            c.nombre_curso,
            prof.dni,
            desde,
            hasta
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => notificar_carga(WARN, &err.to_string()),
        };
    }
    Ok(())
}

//    let n_items = profesores.len();
//
//    let mut cursos = Vec::with_capacity(n_items);
//    for _ in 0..n_items {
//        cursos.push(
//            curso_conferencia
//                .choose(&mut *GLOBAL_RNG.lock().await)
//                .unwrap(),
//        )
//    }
//
//    let mut f_desde: Vec<Date> = Vec::with_capacity(n_items);
//    f_desde.extend((0..n_items).map(|_| Date().fake::<Date>()));
//
//    let mut f_hasta = Vec::with_capacity(n_items);
//    f_hasta.extend((0..n_items).map(|idx| match cursos[idx].tipo.as_str() {
//        "Curso" => Some(f_desde[idx] + Duration::days(30)),
//        "Conferencia" => Some(f_desde[idx] + Duration::days(1)),
//        _ => None,
//    }));
//
//    let max_capacity = BIND_LIMIT / 5;
//    if n_items <= max_capacity {
//        let mut query_builder: QueryBuilder<MySql> =
//            QueryBuilder::new("insert into AtendioA (NombreCurso, DNIProfesor, Desde, Hasta)");
//        query_builder.push_values(0..n_items, |mut b, idx| {
//            let profesor = &profesores[idx];
//            let desde = f_desde[idx];
//            let hasta = f_hasta[idx];
//            let curso = cursos[idx];
//
//            b.push_bind(curso.nombre_curso.clone())
//                .push_bind(profesor.dni.clone())
//                .push_bind(desde)
//                .push_bind(hasta);
//        });
//
//        if let Err(err) = query_builder.build().execute(pool).await {
//            eprintln!("{} {}", "[WARN]".bright_yellow().bold(), err);
//        }
//    } else {
//        for chunk in (0..n_items).step_by(max_capacity) {
//            let mut query_builder: QueryBuilder<MySql> =
//                QueryBuilder::new("insert into AtendioA (NombreCurso, DNIProfesor, Desde, Hasta)");
//            let end = std::cmp::min(chunk + max_capacity, n_items);
//            query_builder.push_values(chunk..end, |mut b, idx| {
//                let profesor = &profesores[idx];
//                let desde = f_desde[idx];
//                let hasta = f_hasta[idx];
//                let curso = cursos[idx];
//
//                b.push_bind(curso.nombre_curso.clone())
//                    .push_bind(profesor.dni.clone())
//                    .push_bind(desde)
//                    .push_bind(hasta);
//            });
//            if let Err(err) = query_builder.build().execute(pool).await {
//                eprintln!("{} {}", "[WARN]".bright_yellow().bold(), err);
//            }
//        }
//    }
//    Ok(())
//}

pub async fn cargar_se_da_titulo(
    titulos: &[Titulos],
    instituciones: &[Instituciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for inst in instituciones {
        // FIXME: Encontrar una mejor manera de que cada instituciones emita varios titulos.
        let mut rng = GLOBAL_RNG.lock().await;
        for _ in 1..=rng.gen_range(1..5) {
            let titulo = titulos.choose(&mut *rng).unwrap();
            match sqlx::query!(
                r#"
            insert ignore into SeDaTitulo (Titulo, NombreInst, Nivel)
            values (?,?,?)
            "#,
                titulo.titulo,
                inst.nombre,
                titulo.nivel,
            )
            .execute(pool)
            .await
            {
                Ok(_) => (),
                Err(err) => {
                    notificar_carga(WARN, &err.to_string());
                }
            };
        }
    }
    Ok(())
}

pub async fn cargar_posee_titulo(
    titulos: &[Titulos],
    profesores: &[Profesores],
    muestras: usize,
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    let mut rng = GLOBAL_RNG.lock().await;
    let (terciarios, otros): (Vec<Titulos>, Vec<Titulos>) = titulos
        .iter()
        .cloned()
        .partition(|x| x.nivel == "Terciario");

    for prof in profesores {
        let t = terciarios
            .choose(&mut *rng)
            .expect("No hay titulos terciarios en la tabla Titulos.");

        // FIXME: Esto obviamente es muy ingenuo.
        let desde: Date = Date().fake();
        let hasta = desde + Duration::days(365 * 5);
        match sqlx::query!(
            r#"
            insert into PoseeTitulo (DNI, Nivel, Titulo, Desde, Hasta)
            values (?,?,?,?,?)
            "#,
            prof.dni,
            t.nivel,
            t.titulo,
            desde,
            hasta
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }

    let m = rng.gen_range(0..muestras);
    for _ in 0..=m {
        let t = otros
            .choose(&mut *rng)
            .expect("No hay titulos no terciarios en la tabla Titulos.");
        let prof = profesores.choose(&mut *rng).unwrap();

        // FIXME: Esto obviamente es muy ingenuo.
        let desde: Date = Date().fake();
        let hasta = desde + Duration::days(365 * 5);
        match sqlx::query!(
            r#"
            insert into PoseeTitulo (DNI, Nivel, Titulo, Desde, Hasta)
            values (?,?,?,?,?)
            "#,
            prof.dni,
            t.nivel,
            t.titulo,
            desde,
            hasta
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }
    Ok(())
}

pub async fn cargar_beneficia(
    obras: &[ObrasSociales],
    familiares: &[Familiares],
    muestras: usize,
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    let m = GLOBAL_RNG.lock().await.gen_range(0..muestras);
    for _ in 1..=m {
        let familiar = familiares.choose(&mut *GLOBAL_RNG.lock().await).unwrap();
        let obra = obras.choose(&mut *GLOBAL_RNG.lock().await).unwrap();
        match sqlx::query!(
            r#"
            insert into Beneficia (DNIFamiliar, DNIProfesor, IDObraSocial)
            values (?,?,?)
            "#,
            familiar.dni_familiar,
            familiar.dni_profesor,
            obra.id_obrasocial
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                notificar_carga(WARN, &err.to_string());
            }
        };
    }
    Ok(())
}

pub async fn cargar_se_da_idiomas(
    idiomas: &[&str],
    instituciones: &[Instituciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for inst in instituciones {
        let mut rng = GLOBAL_RNG.lock().await;
        for _ in 1..=rng.gen_range(1..3) {
            let idioma = idiomas.choose(&mut *rng).unwrap();
            match sqlx::query!(
                r#"
            insert ignore into SeDaIdioma (NombreIdioma, NombreInst)
            values (?,?)
            "#,
                inst.nombre,
                idioma,
            )
            .execute(pool)
            .await
            {
                Ok(_) => (),
                Err(err) => {
                    notificar_carga(WARN, &err.to_string());
                }
            };
        }
    }
    Ok(())
}

pub async fn cargar_conoce_idiomas(
    idiomas: &[&str],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let mut rng = GLOBAL_RNG.lock().await;
        for _ in 1..=rng.gen_range(1..3) {
            let idioma = idiomas.choose(&mut *rng).unwrap();
            let certificacion: String = Word().fake();
            let nivel: String = Word().fake();
            match sqlx::query!(
                r#"
            insert ignore into ConoceIdioma (DNIProfesor, NombreIdioma, Certificacion, Nivel)
            values (?,?,?,?)
            "#,
                prof.dni,
                idioma,
                certificacion,
                nivel
            )
            .execute(pool)
            .await
            {
                Ok(_) => (),
                Err(err) => {
                    notificar_carga(WARN, &err.to_string());
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
                Ok(_) => (),
                Err(err) => {
                    notificar_carga(WARN, &err.to_string());
                }
            };
        }
    }
    Ok(())
}
