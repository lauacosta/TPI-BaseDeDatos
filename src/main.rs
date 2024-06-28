use carga_datos::{db_cargasfk::*, db_tablas::*, *};
use clap::Parser;
use colored::Colorize;
use dbdata::DBData;
use rand::seq::SliceRandom;
use rand::thread_rng;
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
    let pool = conectar_con_bd().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let muestras = Args::parse().cantidad;
    let start = std::time::Instant::now();

    // Primero aquellas tablas que no tienen FK.
    // FIXME: Corregir las colisiones contra la bd?
    let direcciones = cargar_tabla::<Direcciones>(muestras, &pool).await?;
    let titulos = cargar_tabla::<Titulos>(muestras, &pool).await?;
    let publicaciones = cargar_tabla::<Publicaciones>(muestras, &pool).await?;
    let reuniones = cargar_tabla::<ReunionesCientificas>(muestras, &pool).await?;
    let percepciones = cargar_tabla::<Percepciones>(muestras, &pool).await?;
    let seguros = cargar_tabla::<Seguros>(muestras, &pool).await?;
    let obras_sociales = cargar_tabla::<ObrasSociales>(muestras, &pool).await?;
    let mut cont = 7;

    let idiomas = [
        "Inglés",
        "Español",
        "Portugues",
        "Mandarín",
        "Japones",
        "Italiano",
    ];
    cargar_idiomas(&idiomas, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Idiomas".bright_green()
    );

    let mut empleadores = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let direccion = direcciones.choose(&mut thread_rng()).unwrap();
        let fila = Empleadores::new(direccion);
        fila.insertar_en_db(&pool).await?;
        empleadores.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Empleadores".bright_green()
    );

    let mut instituciones = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let direccion = direcciones.choose(&mut thread_rng()).unwrap();
        let fila = Instituciones::new(direccion);
        fila.insertar_en_db(&pool).await?;
        instituciones.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Instituciones".bright_green()
    );

    let mut cur_conf = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut thread_rng()).unwrap();
        let fila = CursosConferencias::new(institucion);
        fila.insertar_en_db(&pool).await?;
        cur_conf.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "CursosConferencias".bright_green()
    );

    //FIXME: Tiene sentido cargar tantas actividades como muestras?
    let mut act_uni = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut thread_rng()).unwrap();
        let fila = ActividadesExtensionUniversitaria::new(institucion);
        fila.insertar_en_db(&pool).await?;
        act_uni.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "ActividadesExtensionUniversitaria".bright_green()
    );

    //FIXME: Tiene sentido cargar tantas actividades como muestras?
    let mut act_inv = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut thread_rng()).unwrap();
        let fila = ActividadesInvestigacion::new(institucion);
        fila.insertar_en_db(&pool).await?;
        act_inv.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "ActividadesInvestigacion".bright_green()
    );

    let mut profesores = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let empleador = empleadores.choose(&mut thread_rng()).unwrap();
        let fila = Profesores::new(empleador);
        fila.insertar_en_db(&pool).await?;
        profesores.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Profesores".bright_green()
    );

    let mut contactos = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let profesor = profesores.choose(&mut thread_rng()).unwrap();
        let fila = Contactos::new(profesor);
        fila.insertar_en_db(&pool).await?;
        contactos.push(fila)
    }
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Contactos".bright_green()
    );

    let mut dep_emp = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let obra = obras_sociales.choose(&mut thread_rng()).unwrap();
        let direccion = direcciones.choose(&mut thread_rng()).unwrap();
        let profesor = profesores.choose(&mut thread_rng()).unwrap();
        let fila = DependenciasEmpresas::new(profesor, direccion, obra);
        fila.insertar_en_db(&pool).await?;
        dep_emp.push(fila);
    }
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "DependenciasEmpresas".bright_green()
    );

    let mut familiares = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let direccion = direcciones.choose(&mut thread_rng()).unwrap();
        let profesor = profesores.choose(&mut thread_rng()).unwrap();
        let fila = Familiares::new(direccion, profesor);
        fila.insertar_en_db(&pool).await?;
        familiares.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Familiares".bright_green()
    );

    let mut doc_obras = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let obra = obras_sociales.choose(&mut thread_rng()).unwrap();
        let profesor = profesores.choose(&mut thread_rng()).unwrap();
        let fila = DocObraSocial::new(profesor, obra);
        fila.insertar_en_db(&pool).await?;
        doc_obras.push(fila);
    }
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "DocObraSocial".bright_green()
    );

    let mut dec_jur = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let profesor = profesores.choose(&mut thread_rng()).unwrap();
        let fila = DeclaracionesJuradas::new(profesor);
        fila.insertar_en_db(&pool).await?;
        dec_jur.push(fila);
    }
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "DeclaracionesJuradas".bright_green()
    );

    let mut dec_car = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let dep = dep_emp.choose(&mut thread_rng()).unwrap();
        let fila = DeclaracionesDeCargo::new(dep);
        fila.insertar_en_db(&pool).await?;
        dec_car.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "DeclaracionesDeCargo".bright_green()
    );

    let mut ant_pro = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let profesor = profesores.choose(&mut thread_rng()).unwrap();
        let declaracion = dec_car.choose(&mut thread_rng()).unwrap();
        let fila = AntecedentesProfesionales::new(profesor, declaracion);
        fila.insertar_en_db(&pool).await?;
        ant_pro.push(fila)
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "AntecedentesProfesionales".bright_green()
    );

    let mut ant_doc = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let institucion = instituciones.choose(&mut thread_rng()).unwrap();
        let profesor = profesores.choose(&mut thread_rng()).unwrap();
        let declaracion = dec_car.choose(&mut thread_rng()).unwrap();
        let fila = AntecedentesDocentes::new(profesor, institucion, declaracion);
        fila.insertar_en_db(&pool).await?;
        ant_doc.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "AntecedentesDocentes".bright_green()
    );

    let mut horarios = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let declaraciones = dec_car.choose(&mut thread_rng()).unwrap();
        let fila = Horarios::new(declaraciones);

        fila.insertar_en_db(&pool).await?;
        horarios.push(fila);
    }

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Horarios".bright_green()
    );

    cargar_atendio_a(&cur_conf, &profesores, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "AtendioA".bright_green()
    );

    cargar_se_da_idiomas(&idiomas, &instituciones, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "SeDaIdioma".bright_green()
    );

    cargar_conoce_idiomas(&idiomas, &profesores, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "ConoceIdiomas".bright_green()
    );

    cargar_beneficia(&obras_sociales, &familiares, muestras, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "Beneficia".bright_green()
    );

    cargar_posee_titulo(&titulos, &profesores, muestras, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "PoseeTitulos".bright_green()
    );

    cargar_se_da_titulo(&titulos, &instituciones, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "SeDaTitulos".bright_green()
    );

    cargar_realiza_investigacion(&act_inv, &profesores, muestras, &pool).await?;

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "RealizaInves".bright_green()
    );

    cargar_realizo_actividad(&act_uni, &profesores, muestras, &pool).await?;

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "RealizoAct".bright_green()
    );

    cargar_referencias_bibliograficas(&publicaciones, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "ReferenciasBibliograficas".bright_green()
    );

    cargar_publico(&publicaciones, &profesores, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "PublicoPublicacion".bright_green()
    );

    cargar_participo_en_reunion(&reuniones, &profesores, &pool).await?;
    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "ParticipoEnReunion".bright_green()
    );

    cargar_percibe_en(&percepciones, &profesores, &pool).await?;

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "PercibeEn".bright_green()
    );

    cargar_reside_en(&profesores, &direcciones, &pool).await?;

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "ResideEn".bright_green()
    );

    cargar_asegura_a(&seguros, &familiares, &pool).await?;

    cont += 1;
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".bright_green(),
        "AseguraA".bright_green()
    );

    cont += 1;
    eprintln!(
        "{} Se han cargado aproximadamente {} registros en {} segundos.",
        "[INFO]".bright_green(),
        (muestras * cont).to_string().bright_green(),
        start.elapsed().as_secs().to_string().bright_green()
    );

    Ok(())
}
