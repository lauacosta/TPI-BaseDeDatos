use carga_datos::{db_cargasfk::*, db_tablas::*, *};
use clap::Parser;
use colored::Colorize;
use dbdata::DBData;
use mysql_async::prelude::*;
use mysql_async::Opts;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    cantidad: usize,
}

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
    //let pool = conectar_con_bd().await?;
    //sqlx::migrate!("./migrations").run(&pool).await?;
    //
    dotenvy::dotenv()?;
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = mysql_async::Pool::new(Opts::from_url(&database_url)?);
    let mut conn = pool.get_conn().await?;

    let muestras = Args::parse().cantidad;

    let start = std::time::Instant::now();

    // Primero aquellas tablas que no tienen FK.
    // FIXME: Corregir las colisiones contra la bd?
    let idiomas = [
        "Inglés",
        "Español",
        "Portugues",
        "Mandarín",
        "Japones",
        "Italiano",
    ];
    //cargar_idiomas(&idiomas, &pool).await?;
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "Idiomas".bright_green()
    //);

    let direcciones: Vec<Direcciones> = generar_tabla(muestras, &pool).await?.insertar_en_bd().await?;

    r"INSERT INTO Direcciones (codigopostal,calle,numero,localidad,provincia) 
      VALUES (:codigopostal,:calle,:numero,:localidad,:provincia)".
        with(direcciones.iter().map(|dir| params! {
           "codigopostal" => dir.codigo_postal,
           "calle" => dir.calle.clone(),
           "numero" => dir.numero,
           "localidad" => dir.localidad.clone(),
           "provincia" => dir.provincia.clone()
     }))
    .batch(&mut conn)
    .await?;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Direcciones".bright_green()
    );

    drop(conn);
    pool.disconnect().await?;
    
    //let titulos = cargar_tabla::<Titulos>(muestras, &pool).await?;
    //let cur_conf = cargar_tabla::<CursosOConferencias>(muestras, &pool).await?;
    //let act_inv = cargar_tabla::<ActividadesInvestigacion>(muestras, &pool).await?;
    //let act_uni = cargar_tabla::<ActividadesExtensionUniversitaria>(muestras, &pool).await?;
    //let publicaciones = cargar_tabla::<Publicaciones>(muestras, &pool).await?;
    //let reuniones = cargar_tabla::<ReunionesCientificas>(muestras, &pool).await?;
    //let percepciones = cargar_tabla::<Percepciones>(muestras, &pool).await?;
    //let seguros = cargar_tabla::<Seguros>(muestras, &pool).await?;
    //
    //let mut empleadores = Vec::with_capacity(muestras);
    //for _ in 1..=muestras {
    //    let direccion = direcciones.choose(&mut thread_rng()).unwrap();
    //    let fila = Empleadores::new(direccion);
    //    fila.insertar_en_db(&pool).await?;
    //    empleadores.push(fila);
    //}
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "Empleadores".bright_green()
    //);
    //
    //let mut profesores = Vec::with_capacity(muestras);
    //for _ in 1..=muestras {
    //    let empleador = empleadores.choose(&mut thread_rng()).unwrap();
    //    let fila = Profesores::new(empleador);
    //    fila.insertar_en_db(&pool).await?;
    //    profesores.push(fila);
    //}
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "Profesores".bright_green()
    //);
    //
    //let mut contactos = Vec::with_capacity(muestras);
    //for _ in 1..=muestras {
    //    let profesor = profesores.choose(&mut thread_rng()).unwrap();
    //    let fila = Contactos::new(profesor);
    //    fila.insertar_en_db(&pool).await?;
    //    contactos.push(fila)
    //}
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "Contactos".bright_green()
    //);
    //
    //cargar_conoce_idiomas(&idiomas, &profesores, &pool).await?;
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "ConoceIdiomas".bright_green()
    //);
    //
    //cargar_posee_titulo(&titulos, &profesores, &pool).await?;
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "PoseeTitulos".bright_green()
    //);
    //
    //cargar_atendio_a(&cur_conf, &profesores, &pool).await?;
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "AtendioA".bright_green()
    //);
    //
    //let ant_doc: Vec<AntecedentesDocentes> = (1..=muestras)
    //    .map(|_| {
    //        let profesor = profesores.choose(&mut thread_rng()).unwrap();
    //        AntecedentesDocentes::new(profesor)
    //    })
    //    .collect();
    //for i in ant_doc.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "AntecedentesDocentes".bright_green()
    //);
    //
    //cargar_participa_en_investigacion(&act_inv, &profesores, &pool).await?;
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "ParticipaEnInvestigacion".bright_green()
    //);
    //
    //cargar_realizo_actividad(&act_uni, &profesores, &pool).await?;
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "RealizoActividad".bright_green()
    //);
    //
    //let ant_pro: Vec<AntecedentesProfesionales> = (1..=muestras)
    //    .map(|_| {
    //        let profesor = profesores.choose(&mut thread_rng()).unwrap();
    //        AntecedentesProfesionales::new(profesor)
    //    })
    //    .collect();
    //for i in ant_pro.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "AntecedentesProfesionales".bright_green()
    //);
    //
    //cargar_referencias_bibliograficas(&publicaciones, &pool).await?;
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "ReferenciasBibliograficas".bright_green()
    //);
    //
    //cargar_publico_publicaciones(&publicaciones, &profesores, &pool).await?;
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "PublicoPublicacion".bright_green()
    //);
    //
    //cargar_participo_en_reunion(&reuniones, &profesores, &pool).await?;
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "ParticipoEnReunion".bright_green()
    //);
    //
    //let dep_emp: Vec<DependenciasOEmpresas> = (1..=muestras)
    //    .map(|_| {
    //        let profesor = profesores.choose(&mut thread_rng()).unwrap();
    //        DependenciasOEmpresas::new(profesor)
    //    })
    //    .collect();
    //for i in dep_emp.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "DependenciasOEmpresas".bright_green()
    //);
    //
    //let beneficiarios: Vec<Beneficiarios> = (1..=muestras)
    //    .map(|_| {
    //        let direccion = direcciones.choose(&mut thread_rng()).unwrap();
    //        Beneficiarios::new(direccion)
    //    })
    //    .collect();
    //for i in beneficiarios.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "Beneficiarios".bright_green()
    //);
    //
    //let ob_social: Vec<ObrasSociales> = (1..=muestras)
    //    .map(|_| {
    //        let profesor = profesores.choose(&mut thread_rng()).unwrap();
    //        let beneficiario = if thread_rng().gen::<bool>() {
    //            Some(beneficiarios.choose(&mut thread_rng()).unwrap())
    //        } else {
    //            None
    //        };
    //        ObrasSociales::new(profesor, beneficiario)
    //    })
    //    .collect();
    //for i in ob_social.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "ObrasSociales".bright_green()
    //);
    //
    //cargar_percibe_en(&percepciones, &profesores, &pool).await?;
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "PercibeEn".bright_green()
    //);
    //
    //let dec_jur: Vec<DeclaracionesJuradas> = (1..=muestras)
    //    .map(|_| {
    //        let profesor = profesores.choose(&mut thread_rng()).unwrap();
    //        DeclaracionesJuradas::new(profesor)
    //    })
    //    .collect();
    //for i in dec_jur.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "DeclaracionesJuradas".bright_green()
    //);
    //
    //let dec_car: Vec<DeclaracionesDeCargo> = (1..=muestras)
    //    .map(|_| {
    //        let direccion = direcciones.choose(&mut thread_rng()).unwrap();
    //        DeclaracionesDeCargo::new(direccion)
    //    })
    //    .collect();
    //for i in dec_car.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "DeclaracionesDeCargo".bright_green()
    //);
    //
    //let horarios: Vec<Horarios> = (1..=muestras)
    //    .map(|_| {
    //        let declaraciones = dec_car.choose(&mut thread_rng()).unwrap();
    //        Horarios::new(declaraciones)
    //    })
    //    .collect();
    //for i in horarios.iter() {
    //    i.insertar_en_db(&pool).await?;
    //}
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "Horarios".bright_green()
    //);
    //
    //cargar_cumple_cargo(&profesores, &dec_car, &pool).await?;
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "CumpleCargo".bright_green()
    //);
    //
    //cargar_reside_en(&profesores, &direcciones, &pool).await?;
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "ResideEn".bright_green()
    //);
    //
    //cargar_asegura_a(&profesores, &seguros, &beneficiarios, &pool).await?;
    //
    //eprintln!(
    //    "{} Se ha cargado {} correctamente!",
    //    "[INFO]".bright_green(),
    //    "AseguraA".bright_green()
    //);
    //
    eprintln!(
        "{} Se han cargado {} registros en {} segundos.",
        "[INFO]".bright_green(),
        (muestras * 33).to_string().bright_green(),
        start.elapsed().as_secs().to_string().bright_green()
    );
    //
    Ok(())
}
