use carga_datos::{funciones_de_carga::*, tablas::*};
use colored::Colorize;
use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use sqlx::mysql::MySqlPoolOptions;
use std::error::Error;

/* Orden de carga hasta ahora:
Primero aquellas tablas que no tienen FKs.
    1. Idiomas
    2. Direcciones
    3. Titulos
    4. CursosOConferencias
    5. ActividadesInvestigacion
    6. ActividadesExtensionUniversitaria
    7. Publicaciones
    8. ReunionesCientificas
    9. Percepciones
    10. Seguros
*/

/* Segundo, aquellas tablas que contienen FKs.
    11. Empleadores
    12. Profesores
    13. Contactos
    14. ConoceIdiomas
    15. PoseeTitulo
    16. AtendioA
    17. AntecedentesDocentes
    18. ParticipaEnInvestigacion
    19. RealizoActividad
    20. AntecedentesProfesionales
    21. ReferenciaBibliografica
    22. PublicoPublicacion
    23. ParticipoEnReunion
    24. DependenciasOEmpresas
    25. Beneficiarios
    26. ObrasSociales
    27. PercibeEn
    28. DeclaracionesJuradas
    29. DeclaracionesDeCargo
    30. Horarios
    31. CumpleCargo
    32. ResideEn
    33. AseguraA
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_url =
        std::env::var("DATABASE_URL").expect("No se pudo encontrar la variable 'DATABASE_URL'");
    let pool = MySqlPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let muestras = 10;

    // Primero aquellas tablas que no tienen FK.
    let idiomas = [
        "Inglés",
        "Español",
        "Portugues",
        "Mandarín",
        "Japones",
        "Italiano",
    ];
    cargar_idiomas(&idiomas, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Idiomas".green());

    let direcciones: Vec<Direcciones> = gen_tablas(10);
    cargar_direcciones(&direcciones, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Direcciones".green());

    let titulos: Vec<Titulos> = gen_tablas(muestras);
    cargar_titulos(&titulos, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Titulos".green());

    let cur_conf: Vec<CursoOConferencia> = gen_tablas(muestras);
    cargar_cur_conf(&cur_conf, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "CursosOConferencia".green()
    );

    let act_inv: Vec<ActividadesInvestigacion> = gen_tablas(muestras);
    cargar_actividades_investigacion(&act_inv, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "ActividadesInvestigacion".green()
    );

    let act_uni: Vec<ActividadesExtensionUniversitaria> = gen_tablas(muestras);
    cargar_actividad_universitaria(&act_uni, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "ActividadesExtensionUniversitaria".green()
    );

    let publicaciones: Vec<Publicaciones> = gen_tablas(muestras);
    cargar_publicaciones(&publicaciones, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Publicaciones".green());

    let reuniones: Vec<ReunionesCientificas> = gen_tablas(muestras);
    cargar_reuniones_cientificas(&reuniones, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "ReunionesCientificas".green()
    );

    let percepciones: Vec<Percepciones> = gen_tablas(muestras);
    cargar_percepciones(&percepciones, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Percepciones".green());

    let seguros: Vec<Seguros> = gen_tablas(muestras);
    cargar_seguros(&seguros, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Seguros".green());

    // Segundo, aquellas tablas que contienen FKs.
    let empleadores: Vec<Empleadores> = (1..=muestras)
        .map(|_| {
            let direccion = direcciones.choose(&mut thread_rng()).unwrap();
            Empleadores::new(direccion)
        })
        .collect();
    cargar_empleadores(&empleadores, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Empleadores".green());

    let profesores: Vec<Profesores> = (1..=muestras)
        .map(|_| {
            let empleador = empleadores.choose(&mut thread_rng()).unwrap();
            Profesores::new(empleador)
        })
        .collect();
    cargar_profesores(&profesores, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Profesores".green());

    let contactos: Vec<Contactos> = (1..=muestras)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            Contactos::new(profesor)
        })
        .collect();
    cargar_contactos(&contactos, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Contactos".green());

    cargar_conoce_idiomas(&idiomas, &profesores, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "ConoceIdiomas".green());

    cargar_posee_titulo(&titulos, &profesores, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "PoseeTitulos".green());

    cargar_atendio_a(&cur_conf, &profesores, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "AtendioA".green());

    let ant_doc: Vec<AntecedentesDocentes> = (1..=muestras)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            AntecedentesDocentes::new(profesor)
        })
        .collect();
    cargar_antecedentes_docentes(&ant_doc, &pool).await?;
    eprintln!(
        "Se ha cargado {} docentes correctamente!",
        "antecedentes ".green()
    );

    cargar_participa_en_investigacion(&act_inv, &profesores, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "ParticipaEnInvestigacion".green()
    );

    cargar_realizo_actividad(&act_uni, &profesores, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "RealizoActividad ".green()
    );

    let ant_pro: Vec<AntecedentesProfesionales> = (1..=muestras)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            AntecedentesProfesionales::new(profesor)
        })
        .collect();
    cargar_antecedentes_profesionales(&ant_pro, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "AntecedentesProfesionales".green()
    );

    cargar_referencias_bibliograficas(&publicaciones, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "ReferenciasBibliograficas".green()
    );

    cargar_publico_publicaciones(&publicaciones, &profesores, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "PublicoPublicacion".green()
    );

    cargar_participo_en_reunion(&reuniones, &profesores, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "ParticipoEnReunion".green()
    );

    let dep_emp: Vec<DependenciasOEmpresas> = (1..=muestras)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            DependenciasOEmpresas::new(profesor)
        })
        .collect();
    cargar_dependencias_o_empresas(&dep_emp, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "DependenciasOEmpresas".green()
    );

    let beneficiarios: Vec<Beneficiarios> = (1..=muestras)
        .map(|_| {
            let direccion = direcciones.choose(&mut thread_rng()).unwrap();
            Beneficiarios::new(direccion)
        })
        .collect();
    cargar_beneficiarios(&beneficiarios, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Beneficiarios".green());

    let ob_social: Vec<ObrasSociales> = (1..=muestras)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            let beneficiario = if thread_rng().gen::<bool>() {
                Some(beneficiarios.choose(&mut thread_rng()).unwrap())
            } else {
                None
            };
            ObrasSociales::new(profesor, beneficiario)
        })
        .collect();
    cargar_obras_sociales(&ob_social, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "ObrasSociales".green());

    cargar_percibe_en(&percepciones, &profesores, &pool).await?;
    eprintln!("Se ha cargado {} en correctamente!", "Percibe".green());

    let dec_jur: Vec<DeclaracionesJuradas> = (1..=muestras)
        .map(|_| {
            let profesor = profesores.choose(&mut thread_rng()).unwrap();
            DeclaracionesJuradas::new(profesor)
        })
        .collect();
    cargar_declaraciones_juradas(&dec_jur, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "DeclaracionesJuradas".green()
    );

    let dec_car: Vec<DeclaracionesDeCargo> = (1..=muestras)
        .map(|_| {
            let direccion = direcciones.choose(&mut thread_rng()).unwrap();
            DeclaracionesDeCargo::new(direccion)
        })
        .collect();
    cargar_declaraciones_cargo(&dec_car, &pool).await?;
    eprintln!(
        "Se ha cargado {} correctamente!",
        "DeclaracionesDeCargo".green()
    );

    let horarios: Vec<Horarios> = (1..=muestras)
        .map(|_| {
            let declaraciones = dec_car.choose(&mut thread_rng()).unwrap();
            Horarios::new(&declaraciones)
        })
        .collect();
    cargar_horarios(&horarios, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "Horarios".green());

    cargar_cumple_cargo(&profesores, &dec_car, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "CumpleCargo".green());

    cargar_reside_en(&profesores, &direcciones, &pool).await?;
    eprintln!("Se ha cargado {} correctamente!", "ResideEn".green());

    cargar_asegura_a(&profesores, &seguros, &beneficiarios, &pool).await?;
    eprintln!("Se ha cargado {} a correctamente!", "AseguraA".green());
    Ok(())
}
