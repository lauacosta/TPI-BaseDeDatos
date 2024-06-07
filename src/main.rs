use carga_datos::{
    ActividadesExtensionUniversitaria, ActividadesInvestigacion, AntecedentesDocentes,
    AntecedentesProfesionales, Contactos, CursoOConferencia, Direcciones, Empleadores, Idiomas,
    Profesores, Publicaciones, Titulos,
};
use dotenv::dotenv;
use fake::faker::lorem::en::*;
use fake::faker::time::en::Date;
use fake::Fake;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::time::Date;
use sqlx::{MySql, Pool};
use std::error::Error;
use time::Duration;

/// Orden de carga hasta ahora:
/// 1. Direcciones
/// 2. Empleadores
/// 3. Profesores
/// 4. Contactos
/// 5. ConoceIdiomas
/// 6. Titulos
/// 7. PoseeTitulo
/// 8. CursosOConferencias
/// 9. AtendioA
/// 10. AntecedentesDocentes
/// 11. ActividadesInvestigacion
/// 12. ParticipaEnInvestigacion
/// 13. ActividadesExtensionUniversitaria
/// 14. RealizoActividad

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_url =
        std::env::var("DATABASE_URL").expect("No se pudo encontrar la variable 'DATABASE_URL'");
    let pool = MySqlPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

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

    let contactos: Vec<Contactos> = (1..=10)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            Contactos::new(profesor)
        })
        .collect();

    cargar_contactos(&contactos, &pool).await?;
    println!("Se han cargado todos los contactos correctamente!");

    let idiomas: Vec<Idiomas> = (1..=10).map(|_| Idiomas::new()).collect();
    cargar_idiomas(&idiomas, &pool).await?;
    println!("Se han cargado todos los idiomas correctamente!");

    cargar_conoce_idiomas(&idiomas, &profesores, &pool).await?;
    println!("Se han cargado todos los conoce_idiomas correctamente!");

    let titulos: Vec<Titulos> = (1..=10).map(|_| Titulos::new()).collect();
    cargar_titulos(&titulos, &pool).await?;
    println!("Se han cargado todos los titulos correctamente!");

    cargar_posee_titulo(&titulos, &profesores, &pool).await?;
    println!("Se han cargado todos los posee_titulos correctamente!");

    let cursos_conferencias: Vec<CursoOConferencia> =
        (1..=10).map(|_| CursoOConferencia::new()).collect();
    cargar_cur_conf(&cursos_conferencias, &pool).await?;
    println!("Se han cargado todos los cursos o conferencias correctamente!");

    cargar_atendio_a(&cursos_conferencias, &profesores, &pool).await?;
    println!("Se han cargado todos los atendio_a correctamente!");

    let antecedentes_docentes: Vec<AntecedentesDocentes> = (1..=10)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            AntecedentesDocentes::new(profesor)
        })
        .collect();
    cargar_antecedentes_docentes(&antecedentes_docentes, &pool).await?;
    println!("Se han cargado todos los antecedentes docentes correctamente!");

    let actividades_investigacion: Vec<ActividadesInvestigacion> =
        (1..=10).map(|_| ActividadesInvestigacion::new()).collect();
    cargar_actividades_investigacion(&actividades_investigacion, &pool).await?;
    println!("Se han cargado todas las actividades de investigacion correctamente!");

    cargar_participa_en_investigacion(&actividades_investigacion, &profesores, &pool).await?;
    println!("Se han cargado todas las actividades de investigacion correctamente!");

    let actividad_universitaria: Vec<ActividadesExtensionUniversitaria> = (1..=10)
        .map(|_| ActividadesExtensionUniversitaria::new())
        .collect();
    cargar_actividad_universitaria(&actividad_universitaria, &pool).await?;
    println!("Se han cargado todas las actividades universitarias correctamente!");

    cargar_realizo_actividad(&actividad_universitaria, &profesores, &pool).await?;
    println!("Se han cargado todas las realizo actividad correctamente!");

    let antecedentes_profesonales: Vec<AntecedentesProfesionales> = (1..=10)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            AntecedentesProfesionales::new(profesor)
        })
        .collect();
    cargar_antecedentes_profesionales(&antecedentes_profesonales, &pool).await?;
    println!("Se han cargado todos los antecedentes profesionales correctamente!");

    let publicaciones: Vec<Publicaciones> = (1..=10).map(|_| Publicaciones::new()).collect();
    cargar_publicaciones(&publicaciones, &pool).await?;
    println!("Se han cargado todas las publicaciones correctamente!");
    cargar_referencias_bibliograficas(&publicaciones, &pool).await?;
    println!("Se han cargado todas las referencias bibliograficas correctamente!");

    Ok(())
}

// FIXME: Ver como usar macros para reducir el codigo duplicado.

async fn cargar_referencias_bibliograficas(
    publicaciones: &[Publicaciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for _ in (1..thread_rng().gen_range(1..publicaciones.len())).into_iter() {
        let p1 = publicaciones.choose(&mut thread_rng()).unwrap();
        let p2 = publicaciones.choose(&mut thread_rng()).unwrap();
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
async fn cargar_publicaciones(
    publicaciones: &[Publicaciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for p in publicaciones {
        match sqlx::query!(
            r#"
            insert into Publicaciones (IDPublicacion, Autores, Anio, Titulo)
            values (?,?,?,?)
            "#,
            p.id_publicacion,
            p.autores,
            p.anio,
            p.titulo
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error: {} ANIO: {}", err, p.anio);
                continue;
            }
        };
    }
    Ok(())
}

async fn cargar_antecedentes_profesionales(
    antecedentes: &[AntecedentesProfesionales],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for a in antecedentes {
        match sqlx::query!(
            r#"
            insert into AntecedentesProfesionales (DNIProfesor, Cargo, Empresa, TipoActividad, Desde, Hasta)
            values (?,?,?,?,?,?)
            "#,
            a.dni_profesor,
            a.cargo,
            a.empresa,
            a.tipo_actividad,
            a.desde,
            a.hasta,
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

async fn cargar_realizo_actividad(
    actividades: &[ActividadesExtensionUniversitaria],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let mut rng = thread_rng();
        let act = actividades.choose(&mut thread_rng()).unwrap();

        let acciones: String = Word().fake();
        // FIXME: Esto obviamente es muy ingenuo.
        let desde: Date = Date().fake();
        let hasta = if rng.gen::<bool>() {
            Some(desde + Duration::days(365))
        } else {
            None
        };
        let dedicacion = rng.gen_range(1..8);

        //FIXME: Revisar por qué considera que Dedicacion no es parte de la tabla.
        match sqlx::query!(
            r#"
            insert into RealizoActividad (IDActividad, DNIProfesor, Acciones, Hasta, Desde)
            values (?,?,?,?,?)
            "#,
            act.id_actividad,
            prof.dni,
            acciones,
            //dedicacion,
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

async fn cargar_actividad_universitaria(
    actividades: &[ActividadesExtensionUniversitaria],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for a in actividades {
        match sqlx::query!(
            r#"
            insert into ActividadesExtensionUniversitaria (IDActividad, Institucion, Cargo, Categoria)
            values (?,?,?,?)
            "#,
            a.id_actividad,
            a.institucion,
            a.cargo,
            a.categoria,
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

async fn cargar_participa_en_investigacion(
    actividades: &[ActividadesInvestigacion],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let mut rng = thread_rng();
        let act = actividades.choose(&mut thread_rng()).unwrap();
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

async fn cargar_actividades_investigacion(
    actividades: &[ActividadesInvestigacion],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for a in actividades {
        match sqlx::query!(
            r#"
            insert into ActividadesInvestigacion (IDInvestigacion, Institucion, Categoria, AreaPPAL)
            values (?,?,?,?)
            "#,
            a.id_investigacion,
            a.institucion,
            a.categoria,
            a.area_ppal
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

async fn cargar_antecedentes_docentes(
    antecedentes: &[AntecedentesDocentes],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for a in antecedentes {
        match sqlx::query!(
            r#"
            insert into AntecedentesDocentes (Institucion, UnidadAcademica, Cargo, Desde, Hasta, Dedicacion, DNIProfesor)
            values (?,?,?,?, ?, ?, ?)
            "#,
            a.institucion,
            a.unidad_academica,
            a.cargo,
            a.desde,
            a.hasta,
            a.dedicacion,
            a.dni_profesor
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

async fn cargar_atendio_a(
    curso_conferencia: &[CursoOConferencia],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let c = curso_conferencia.choose(&mut thread_rng()).unwrap();
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

async fn cargar_cur_conf(
    curso_conferencia: &[CursoOConferencia],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for c in curso_conferencia {
        match sqlx::query!(
            r#"
            insert into CursosOConferencias (Nombre, Institucion, Descripcion, Tipo)
            values (?,?,?,?)
            "#,
            c.nombre,
            c.institucion,
            c.descripcion,
            c.tipo
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

async fn cargar_posee_titulo(
    titulos: &[Titulos],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let t = titulos.choose(&mut thread_rng()).unwrap();

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

async fn cargar_titulos(titulos: &[Titulos], pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
    for t in titulos {
        match sqlx::query!(
            r#"
            
            insert into Titulos
            (Institucion, Nivel, Titulo) 
            values (?,?,?)

            "#,
            t.institucion,
            t.nivel,
            t.titulo
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

async fn cargar_conoce_idiomas(
    idiomas: &[Idiomas],
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        let idioma = idiomas.choose(&mut thread_rng()).unwrap().0.clone();
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
    Ok(())
}

async fn cargar_idiomas(idiomas: &[Idiomas], pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
    let row_count: i64 = sqlx::query_scalar("select count(*) FROM Idiomas")
        .fetch_one(pool)
        .await?;

    if row_count == 0 {
        for i in idiomas {
            match sqlx::query!(
                r#"
            insert into Idiomas (Nombre) values (?)
            "#,
                i.0
            )
            .execute(pool)
            .await
            {
                Ok(_) => continue,
                Err(err) => {
                    println!("Error: {}", err);
                    continue;
                }
            };
        }
    }
    Ok(())
}

async fn cargar_contactos(
    contactos: &[Contactos],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for c in contactos {
        match sqlx::query!(
            r#"
            
            insert into Contactos
            (DNIProfesor, Tipo, Direccion, Medio, Numero) 
            values (?,?,?,?,?)

            "#,
            c.dni_profesor,
            c.tipo,
            c.direccion,
            c.medio,
            c.numero
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

async fn cargar_profesores(
    profesores: &[Profesores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for prof in profesores {
        match sqlx::query!(
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
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}
async fn cargar_empleadores(
    empleadores: &[Empleadores],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for emp in empleadores {
        match sqlx::query!(
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
        .await
            {
                Ok(_) => continue,
                Err(err) => {
                    println!("Error: {}", err);
                    continue;
                }
            };
    }
    Ok(())
}
async fn cargar_direcciones(
    direcciones: &[Direcciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for dir in direcciones {
        match sqlx::query!(
            r#"insert into Direcciones (CodigoPostal, Calle, Numero, Localidad, Provincia) 
values (?,?,?,?,?)"#,
            dir.codigo_postal,
            dir.calle,
            dir.numero,
            dir.localidad,
            dir.provincia,
        )
        .execute(pool)
        .await
        {
            Ok(_) => continue,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}
