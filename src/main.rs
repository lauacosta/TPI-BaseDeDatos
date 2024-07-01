// Autor: Acosta Quintana, Lautaro

use carga_datos::{datasets::*, db_cargasfk::*, db_tablas::*, Notificacion::INFO, *};
use clap::Parser;
use dbdata::DBData;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::error::Error;

/* Orden de carga hasta ahora:
- Primero aquellas tablas que no tienen FKs.
    01. Direcciones
    02. Titulos
    03. Publicaciones
    04. Reuniones
    05. Percepciones
    06. Seguros
    07. ObrasSociales
    08. Idiomas

- Segundo, aquellas tablas que contienen FKs.

    09. Empleadores
    10. Instituciones
    11. CursosConferencias
    12. ActividadesExtensionUniversitaria
    13. ActividadesInvestigacion
    14. Profesores
    15. Contactos
    16. DependenciasEmpresas
    17. Familiares
    18. DocObraSocial
    19. DeclaracionesJuradas
    20. DeclaracionesDeCargo
    21. AntecedentesProfesionales
    22. AntecedentesDocentes
    23. Horarios
    24. AtendioA
    25. SeDaIdiomas
    26. ConoceIdiomas
    27. Beneficia
    28. PoseeTitulo
    29. SeDaTitulo
    30. RealizaInvestigacion
    31. RealizoActividad
    32. ReferenciasBibliograficas
    33. Publico
    34. ParticipoEnReunion
    35. PercibeEn
    36. ResideEn
    37. AseguraA
*/

#[derive(Parser, Debug)]
#[command(version = "0.1.1")]
/// Programa auxiliar para la generación de datos en el Trabajo Practico Integrador de la Materia
/// Base de Datos en la Universidad Nacional Regional Resistencia.
struct Args {
    /// Cantidad de registros a generar en cada tabla.
    #[arg(short, long, default_value_t = 1000)]
    cantidad: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = conectar_con_bd().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let muestras = Args::parse().cantidad;
    let nombre_universidades = cargar_de_csv("./datasets/universidades.csv")?;
    let provincias = cargar_provincias("./datasets/provincia_localidad_calles.csv")?;
    let idiomas = cargar_de_csv("./datasets/idiomas.csv")?;
    let mut rng = StdRng::from_entropy();

    // Primero aquellas tablas que no tienen FK.
    let mut direcciones = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let provincia = provincias.choose(&mut rng).unwrap();
        let localidad = provincia.localidades.choose(&mut rng).unwrap();
        let calle = localidad.calles.choose(&mut rng).unwrap();
        let fila = Direcciones::new(&provincia.nombre, &localidad.nombre, calle);
        fila.insertar_en_db(&pool).await?;
        direcciones.push(fila);
    }
    notificar_carga(INFO, "Direcciones");

    let titulos = cargar_tabla::<Titulos>(muestras, &pool).await?;
    let publicaciones = cargar_tabla::<Publicaciones>(muestras, &pool).await?;
    let reuniones = cargar_tabla::<ReunionesCientificas>(muestras, &pool).await?;
    let percepciones = cargar_tabla::<Percepciones>(muestras, &pool).await?;
    let seguros = cargar_tabla::<Seguros>(muestras, &pool).await?;

    let mut obras_sociales = cargar_tabla::<ObrasSociales>(muestras, &pool).await?;
    let dasuten = ObrasSociales::new("D.A.S.U.Te.N", rng.gen());
    dasuten.insertar_en_db(&pool).await?;
    obras_sociales.push(dasuten);

    cargar_idiomas(&idiomas, &pool).await?;

    let mut empleadores = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let direccion = direcciones.choose(&mut rng).unwrap();
        let fila = Empleadores::new(direccion);
        fila.insertar_en_db(&pool).await?;
        empleadores.push(fila);
    }

    notificar_carga(INFO, "Empleadores");

    // WARN: Como toma los primeros n universidades, no podrá insertarse después de ejecutarse por
    // primera vez sobre la misma base de datos.
    let mut instituciones = Vec::with_capacity(muestras);
    for nombre in nombre_universidades.iter().take(muestras) {
        let direccion = direcciones.choose(&mut rng).unwrap();
        let fila = Instituciones::new(direccion, &nombre);
        match fila.insertar_en_db(&pool).await {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{err}");
                dbg!("{:?}", direccion, &fila);
            }
        };
        instituciones.push(fila);
    }

    notificar_carga(INFO, "Instituciones");

    let mut cur_conf = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut rng).unwrap();
        let fila = CursosConferencias::new(institucion);
        fila.insertar_en_db(&pool).await?;
        cur_conf.push(fila);
    }

    notificar_carga(INFO, "CursosConferencias");

    //FIXME: Tiene sentido cargar tantas actividades como muestras?
    let mut act_uni = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut rng).unwrap();
        let fila = ActividadesExtensionUniversitaria::new(institucion);
        fila.insertar_en_db(&pool).await?;
        act_uni.push(fila);
    }

    notificar_carga(INFO, "ActividadesExtensionUniversitaria");

    //FIXME: Tiene sentido cargar tantas actividades como muestras?
    let mut act_inv = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut rng).unwrap();
        let fila = ActividadesInvestigacion::new(institucion);
        fila.insertar_en_db(&pool).await?;
        act_inv.push(fila);
    }

    notificar_carga(INFO, "ActividadesInvestigacion");

    let mut profesores = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let empleador = empleadores.choose(&mut rng).unwrap();
        let fila = Profesores::new(empleador);
        fila.insertar_en_db(&pool).await?;
        profesores.push(fila);
    }

    notificar_carga(INFO, "Profesores");

    let mut contactos = Vec::with_capacity(muestras);
    for prof in profesores.iter() {
        let fila = Contactos::new(prof);
        fila.insertar_en_db(&pool).await?;
        contactos.push(fila)
    }
    notificar_carga(INFO, "Contactos");

    let mut dep_emp = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let obra = obras_sociales.choose(&mut rng).unwrap();
        let direccion = direcciones.choose(&mut rng).unwrap();
        let profesor = profesores.choose(&mut rng).unwrap();
        let fila = DependenciasEmpresas::new(profesor, direccion, obra);
        fila.insertar_en_db(&pool).await?;
        dep_emp.push(fila);
    }
    notificar_carga(INFO, "DependenciasEmpresas");

    let mut familiares = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let direccion = direcciones.choose(&mut rng).unwrap();
        let profesor = profesores.choose(&mut rng).unwrap();
        let fila = Familiares::new(direccion, profesor);
        fila.insertar_en_db(&pool).await?;
        familiares.push(fila);
    }

    notificar_carga(INFO, "Familiares");

    let mut doc_obras = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let obra = obras_sociales.choose(&mut rng).unwrap();
        let profesor = profesores.choose(&mut rng).unwrap();
        let fila = DocObraSocial::new(profesor, obra);
        fila.insertar_en_db(&pool).await?;
        doc_obras.push(fila);
    }
    notificar_carga(INFO, "DocObraSocial");

    let mut dec_jur = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let profesor = profesores.choose(&mut rng).unwrap();
        let fila = DeclaracionesJuradas::new(profesor);
        fila.insertar_en_db(&pool).await?;
        dec_jur.push(fila);
    }
    notificar_carga(INFO, "DeclaracionesJuradas");

    let mut dec_car = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let dep = dep_emp.choose(&mut rng).unwrap();
        let fila = DeclaracionesDeCargo::new(dep);
        fila.insertar_en_db(&pool).await?;
        dec_car.push(fila);
    }

    notificar_carga(INFO, "DeclaracionesDeCargo");

    let mut ant_pro = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let profesor = profesores.choose(&mut rng).unwrap();
        let declaracion = dec_car.choose(&mut rng).unwrap();
        let fila = AntecedentesProfesionales::new(profesor, declaracion);
        fila.insertar_en_db(&pool).await?;
        ant_pro.push(fila)
    }

    notificar_carga(INFO, "AntecedentesProfesionales");

    let mut ant_doc = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut rng).unwrap();
        let profesor = profesores.choose(&mut rng).unwrap();
        let declaracion = dec_car.choose(&mut rng).unwrap();
        let fila = AntecedentesDocentes::new(profesor, institucion, declaracion);
        fila.insertar_en_db(&pool).await?;
        ant_doc.push(fila);
    }

    notificar_carga(INFO, "AntecedentesDocentes");

    let mut horarios = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let declaraciones = dec_car.choose(&mut rng).unwrap();
        let fila = Horarios::new(declaraciones);

        fila.insertar_en_db(&pool).await?;
        horarios.push(fila);
    }

    notificar_carga(INFO, "Horarios");

    cargar_atendio_a(&cur_conf, &profesores, &pool).await?;

    cargar_se_da_idiomas(&idiomas, &instituciones, &pool).await?;

    cargar_conoce_idiomas(&idiomas, &profesores, &pool).await?;

    cargar_beneficia(&obras_sociales, &familiares, muestras, &pool).await?;

    cargar_posee_titulo(&titulos, &profesores, muestras, &pool).await?;

    cargar_se_da_titulo(&titulos, &instituciones, &pool).await?;

    cargar_realiza_investigacion(&act_inv, &profesores, muestras, &pool).await?;

    cargar_realizo_actividad(&act_uni, &profesores, muestras, &pool).await?;

    cargar_referencias_bibliograficas(&publicaciones, &pool).await?;

    cargar_publico(&publicaciones, &profesores, &pool).await?;

    cargar_participo_en_reunion(&reuniones, &profesores, &pool).await?;

    cargar_percibe_en(&percepciones, &profesores, &pool).await?;
    notificar_carga(INFO, "PercibeEn");

    cargar_reside_en(&profesores, &direcciones, &pool).await?;

    cargar_asegura_a(&seguros, &familiares, &pool).await?;

    Ok(())
}
