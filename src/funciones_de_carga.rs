#![allow(dead_code)]
use crate::tablas::*;
use fake::faker::lorem::en::*;
use fake::faker::time::en::Date;
use fake::Fake;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use sqlx::types::time::Date;
use sqlx::{MySql, Pool};
use std::error::Error;
use time::Duration;

// FIXME: Ver como usar macros para reducir el codigo duplicado.
pub async fn cargar_asegura_a(
    profesores: &[Profesores],
    seguros: &[Seguros],
    beneficiarios: &[Beneficiarios],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for s in seguros {
        let prof = profesores.choose(&mut thread_rng()).unwrap();
        let beneficiario = beneficiarios.choose(&mut thread_rng()).unwrap();
        let capital_asegurado = thread_rng().gen_range(100000.0..1000000.0);
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

pub async fn cargar_seguros(seguros: &[Seguros], pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
    for s in seguros {
        match sqlx::query!(
            r#"
            insert into Seguros(
                CodigoCompania, CompaniaAseguradora, LugarEmision, FechaEmision
            )
            values (?,?,?,?)
            "#,
            s.codigo_compania,
            s.compania_aseguradora,
            s.lugar_emision,
            s.fecha_emision
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
        let dir = direcciones.choose(&mut thread_rng()).unwrap();
        let mut rng = thread_rng();
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
        let prof = profesores.choose(&mut thread_rng()).unwrap();
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

pub async fn cargar_horarios(
    horarios: &[Horarios],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for h in horarios {
        match sqlx::query!(
            r#"
            insert into Horarios(
                IDDeclaracion, Dia, RangoHorario, NombreCatedra
            )
            values (?,?,?,?)
            "#,
            h.id_declaracion,
            h.dia,
            h.rango_horario,
            h.nombre_catedra
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

pub async fn cargar_declaraciones_cargo(
    declaraciones_cargo: &[DeclaracionesDeCargo],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for d in declaraciones_cargo {
        match sqlx::query!(
            r#"
            insert into DeclaracionesDeCargo(
                IDDeclaracion, CumpleHorario, Reparticion, Dependencia, CodigoPostal, Calle, Numero 
            )
            values (?,?,?,?,?,?,?)
            "#,
            d.id_declaracion,
            d.cumple_horario,
            d.reparticion,
            d.dependencia,
            d.codigo_postal,
            d.calle,
            d.numero
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

pub async fn cargar_declaraciones_juradas(
    declaraciones_juradas: &[DeclaracionesJuradas],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for d in declaraciones_juradas {
        match sqlx::query!(
            r#"
            insert into DeclaracionesJuradas(
                IDDeclaracion, DNIProfesor, Fecha, Lugar 
            )
            values (?,?,?,?)
            "#,
            d.id_declaracion,
            d.dni_profesor,
            d.fecha,
            d.lugar
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
        let prof = profesores.choose(&mut thread_rng()).unwrap();
        let desde: Date = Date().fake();
        let estado_percepcion = ["Suspendido", "Percibiendo"]
            .choose(&mut thread_rng())
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
pub async fn cargar_percepciones(
    percepciones: &[Percepciones],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for p in percepciones {
        match sqlx::query!(
            r#"
            insert into Percepciones(
                InstitucionCaja, Tipo, Regimen, Causa 
            )
            values (?,?,?,?)
            "#,
            p.institucion_caja,
            p.tipo,
            p.regimen,
            p.causa
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

pub async fn cargar_obras_sociales(
    obras: &[ObrasSociales],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for o in obras {
        match sqlx::query!(
            r#"
            insert into ObrasSociales (
                IDObraSocial, DNIBeneficiarios, DNIProfesor, TipoPersonal, TipoCaracter, PrestaServicios, Dependencia
            )
            values (?,?,?,?,?,?,?)
            "#,
            o.id_obra_social,
            o.dni_beneficiarios,
            o.dni_profesor,
            o.tipo_personal,
            o.tipo_caracter,
            o.presta_servicios,
            o.dependencia
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
pub async fn cargar_beneficiarios(
    beneficiarios: &[Beneficiarios],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for b in beneficiarios {
        match sqlx::query!(
            r#"
            insert into Beneficiarios(
                DNI, Nombre, Apellido, Parentesco, FechaNacimiento, TipoDocumento, Porcentaje,
                NumeroDir, CodigoPostal, Calle, Piso, Departamento
            )
            values (?,?,?,?,?,?,?,?,?,?,?,?)
            "#,
            b.dni,
            b.nombre,
            b.apellido,
            b.parentesco,
            b.fecha_nacimiento,
            b.tipo_documento,
            b.porcentaje,
            b.numero_dir,
            b.codigo_postal,
            b.calle,
            b.piso,
            b.departamento
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

pub async fn cargar_dependencias_o_empresas(
    dep_o_emp: &[DependenciasOEmpresas],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for d in dep_o_emp {
        match sqlx::query!(
            r#"
            insert into DependenciasOEmpresas (
                DNIProfesor, Nombre, FechaIngreso, Cargo, Lugar, TipoActividad, ObraSocial, Observacion, NaturalezaJuridica
            )
            values (?,?,?,?,?,?,?,?,?)
            "#,
            d.dni_profesor,
            d.nombre,
            d.fecha_ingreso,
            d.cargo,
            d.lugar,
            d.tipo_actividad,
            d.obra_social,
            d.observacion,
            d.naturaleza_juridica
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
        let prof = profesores.choose(&mut thread_rng()).unwrap();
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

pub async fn cargar_reuniones_cientificas(
    reuniones: &[ReunionesCientificas],
    pool: &Pool<MySql>,
) -> Result<(), Box<dyn Error>> {
    for p in reuniones {
        match sqlx::query!(
            r#"
            insert into ReunionesCientificas(Titulo, Fecha)
            values (?,?)
            "#,
            p.titulo,
            p.fecha
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
        let prof = profesores.choose(&mut thread_rng()).unwrap();
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
pub async fn cargar_publicaciones(
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
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_antecedentes_profesionales(
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

pub async fn cargar_realizo_actividad(
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
        let _dedicacion = rng.gen_range(1..8);

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

pub async fn cargar_actividad_universitaria(
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

pub async fn cargar_participa_en_investigacion(
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

pub async fn cargar_actividades_investigacion(
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

pub async fn cargar_antecedentes_docentes(
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

pub async fn cargar_atendio_a(
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

pub async fn cargar_cur_conf(
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

pub async fn cargar_posee_titulo(
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

pub async fn cargar_titulos(titulos: &[Titulos], pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
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
        for _ in 1..=thread_rng().gen_range(1..3) {
            let idioma = idiomas.choose(&mut thread_rng()).unwrap();
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

pub async fn cargar_contactos(
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
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}

pub async fn cargar_profesores(
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
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }
    Ok(())
}
pub async fn cargar_empleadores(
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
                    eprintln!("Error: {}", err);
                    continue;
                }
            };
    }
    Ok(())
}
pub async fn cargar_direcciones(
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
                eprintln!("Error: {}", err);
                continue;
            }
        };
    }

    Ok(())
}
