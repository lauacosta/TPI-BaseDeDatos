use colored::Colorize;
use dbdata::DBData;
use dbdata_derive::DBData;
use fake::{
    faker::{
        address::en::*,
        company::en::CompanyName,
        internet::en::SafeEmail,
        lorem::en::*,
        name::en::*,
        phone_number::en::{CellNumber, PhoneNumber},
        time::en::{Date, Time},
    },
    Dummy, Fake, Faker,
};
use once_cell::sync::Lazy;
use rand::{rngs::StdRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};
use sqlx::{
    mysql::MySqlPoolOptions, types::{time::Date, BigDecimal, Type}, MySql, Pool
};
use std::{error::Error, sync::Mutex};
use time::Duration;

static GLOBAL_RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::from_entropy()));

pub async fn conectar_con_bd() -> Result<Pool<MySql>, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_url =
        std::env::var("DATABASE_URL").expect("No se pudo encontrar la variable 'DATABASE_URL'");
    Ok(MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(4))
        .connect(&db_url)
        .await?)
}

pub async fn cargar_tabla<T>(muestras: usize, pool: &Pool<MySql>) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DBData + fake::Dummy<fake::Faker>,
{
    let mut tablas: Vec<T> = Vec::with_capacity(muestras);
    for _ in 1..=muestras {
        let ejemplar: T = Faker.fake();
        ejemplar.insertar_en_db(pool).await?;
        tablas.push(ejemplar);
    }

    let nombre_tabla = std::any::type_name::<T>().rsplit("::").next().unwrap();
    eprintln!(
        "{} Se ha cargado {} correctamente!",
        "[INFO]".to_string().bright_green(),
        nombre_tabla.bright_green()
    );
    Ok(tablas)
}

#[derive(Debug, DBData)]
pub struct Profesores {
    pub dni: Dni,
    pub nombre: String,
    pub apellido: String,
    pub fecha_nacimiento: Date,
    pub nacionalidad: String,
    pub estado_civil: String, // ('Soltero/a', 'Casado/a', 'Divorciado/a', 'Viudo/a', 'Conviviente')
    pub sexo: String,         // ('M', 'F')
    pub cuit: Option<Cuil>,
    pub cuil: Cuil,
    pub cuit_empleador: Cuil, //WARN: FK de Empleador
}

impl Profesores {
    pub fn new(empleador: &Empleadores) -> Self {
        let dni = Faker.fake();
        let nombre = FirstName().fake();
        let apellido = LastName().fake();
        let estado_civil = [
            "Soltero/a",
            "Casado/a",
            "Divorciado/a",
            "Viudo/a",
            "Conviviente",
        ]
        .choose(&mut *GLOBAL_RNG.lock().unwrap())
        .unwrap()
        .to_string();
        let sexo = ['M', 'F']
            .choose(&mut *GLOBAL_RNG.lock().unwrap())
            .unwrap()
            .to_string();
        let fecha_nacimiento = Date().fake();
        let nacionalidad = CountryName().fake();
        let cuil = Cuil::new(&dni);
        let cuit = if GLOBAL_RNG.lock().unwrap().gen::<bool>() {
            Some(Cuil::new(&dni))
        } else {
            None
        };
        let cuit_empleador = empleador.cuit.clone();

        Self {
            dni,
            nombre,
            apellido,
            fecha_nacimiento,
            nacionalidad,
            estado_civil,
            sexo,
            cuit,
            cuil,
            cuit_empleador,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Contactos {
    pub dni_profesor: Dni, //WARN: FK de Profesores
    pub tipo: String,      // ('Celular', 'Telefono', 'Email')
    pub medio: String,     // ('Personal', 'Empresarial', 'Otro')
    pub direccion: Option<String>,
    pub numero: Option<String>,
}

impl Contactos {
    pub fn new(profesor: &Profesores) -> Self {
        let dni_profesor = profesor.dni.clone();
        let tipo = ["Celular", "Telefono", "Email"]
            .choose(&mut *GLOBAL_RNG.lock().unwrap())
            .unwrap()
            .to_string();

        let medio = ["Personal", "Empresarial", "Otro"]
            .choose(&mut *GLOBAL_RNG.lock().unwrap())
            .unwrap()
            .to_string();

        let direccion = match tipo.as_str() {
            "Email" => Some(SafeEmail().fake()),
            _ => None,
        };
        let numero = match tipo.as_str() {
            "Telefono" => Some(PhoneNumber().fake()),
            "Celular" => Some(CellNumber().fake()),
            _ => None,
        };

        Self {
            dni_profesor,
            tipo,
            medio,
            direccion,
            numero,
        }
    }
}

//impl DBData for Contactos {
//    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
//        match sqlx::query!(
//            r#"
//
//            insert into Contactos
//            (DNIProfesor, Tipo, Direccion, Medio, Numero)
//            values (?,?,?,?,?)
//
//            "#,
//            self.dni_profesor,
//            self.tipo,
//            self.direccion,
//            self.medio,
//            self.numero
//        )
//        .execute(pool)
//        .await
//        {
//            Ok(_) => (),
//            Err(err) => {
//                eprintln!("Error: {}", err);
//            }
//        };
//        Ok(())
//    }
//}

#[derive(Debug, DBData)]
pub struct Titulos {
    pub institucion: String,
    pub nivel: String,
    pub titulo: String,
}

impl Dummy<Faker> for Titulos {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Self {
            institucion: Words(1..5).fake::<Vec<String>>().join(" "),
            nivel: Word().fake(),
            titulo: Words(1..5).fake::<Vec<String>>().join(" "),
        }
    }
}

//impl DBData for Titulos {
//    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
//        match sqlx::query!(
//            r#"
//
//            insert into Titulos
//            (Institucion, Nivel, Titulo)
//            values (?,?,?)
//
//            "#,
//            self.institucion,
//            self.nivel,
//            self.titulo
//        )
//        .execute(pool)
//        .await
//        {
//            Ok(_) => (),
//            Err(err) => {
//                eprintln!("Error: {}", err);
//            }
//        };
//        Ok(())
//    }
//}

#[derive(Debug, DBData)]
pub struct CursosOConferencias {
    pub nombre: String,
    pub institucion: String,
    pub descripcion: Option<String>,
    pub tipo: String, // ('Curso', 'Conferencia')
}

impl Dummy<Faker> for CursosOConferencias {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let nombre: String = Name().fake();
        let institucion: String = Word().fake();
        let descripcion = if rng.gen::<bool>() {
            Some(Words(1..20).fake::<Vec<String>>().join(" "))
        } else {
            None
        };
        let tipo = ["Curso", "Conferencia"].choose(rng).unwrap().to_string();
        Self {
            nombre,
            institucion,
            descripcion,
            tipo,
        }
    }
}

//impl DBData for CursoOConferencia {
//    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
//        match sqlx::query!(
//            r#"
//            insert into CursosOConferencias (Nombre, Institucion, Descripcion, Tipo)
//            values (?,?,?,?)
//            "#,
//            self.nombre,
//            self.institucion,
//            self.descripcion,
//            self.tipo
//        )
//        .execute(pool)
//        .await
//        {
//            Ok(_) => (),
//            Err(err) => {
//                eprintln!("Error: {}", err);
//            }
//        };
//        Ok(())
//    }
//}

#[derive(Debug, DBData)]
pub struct AntecedentesDocentes {
    pub institucion: String,
    pub unidad_academica: String,
    pub cargo: String,
    pub desde: Date,
    pub hasta: Option<Date>,
    pub dedicacion: u32,
    pub dni_profesor: Dni, // WARN: FK de Profesores
}

impl AntecedentesDocentes {
    pub fn new(profesor: &Profesores) -> Self {
        let mut rng = GLOBAL_RNG.lock().unwrap();
        let institucion = Word().fake();
        let unidad_academica = Word().fake();
        let cargo = Word().fake();
        let dni_profesor = profesor.dni.clone();
        let desde = Date().fake();
        let hasta = if rng.gen::<bool>() {
            Some(desde + Duration::days(365))
        } else {
            None
        };
        let dedicacion = rng.gen_range(1..8);

        Self {
            institucion,
            unidad_academica,
            cargo,
            desde,
            hasta,
            dedicacion,
            dni_profesor,
        }
    }
}

#[derive(Debug, Dummy, DBData)]
pub struct ActividadesInvestigacion {
    #[dummy(faker = "..")]
    pub id_investigacion: u32,
    #[dummy(faker = "Word()")]
    pub institucion: String,
    #[dummy(faker = "Word()")]
    pub categoria: String,
    #[dummy(faker = "Word()")]
    pub area_ppal: String,
}

#[derive(Debug, Dummy, DBData)]
pub struct ActividadesExtensionUniversitaria {
    #[dummy(faker = "..")]
    pub id_actividad: u32,
    #[dummy(faker = "Word()")]
    pub institucion: String,
    #[dummy(faker = "Word()")]
    pub cargo: String,
    #[dummy(faker = "Word()")]
    pub categoria: String,
}

#[derive(Debug, DBData)]
pub struct AntecedentesProfesionales {
    pub dni_profesor: Dni, //WARN: FK de Profesores
    pub cargo: String,
    pub empresa: String,
    pub tipo_actividad: String,
    pub desde: Date,
    pub hasta: Date,
}

impl AntecedentesProfesionales {
    pub fn new(profesor: &Profesores) -> Self {
        let dni_profesor = profesor.dni.clone();
        let cargo = Word().fake();
        let empresa = CompanyName().fake();
        let tipo_actividad = Word().fake();
        let desde = Date().fake();
        let hasta = desde + Duration::days(365 * GLOBAL_RNG.lock().unwrap().gen_range(1..5));
        Self {
            dni_profesor,
            cargo,
            empresa,
            tipo_actividad,
            desde,
            hasta,
        }
    }
}

//impl DBData for AntecedentesProfesionales {
//    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
//        match sqlx::query!(
//            r#"
//            insert into AntecedentesProfesionales (DNIProfesor, Cargo, Empresa, TipoActividad, Desde, Hasta)
//            values (?,?,?,?,?,?)
//            "#,
//            self.dni_profesor,
//            self.cargo,
//            self.empresa,
//            self.tipo_actividad,
//            self.desde,
//            self.hasta,
//        )
//        .execute(pool)
//        .await
//        {
//            Ok(_) => (),
//            Err(err) => {
//                eprintln!("Error: {}", err);
//            }
//        };
//        Ok(())
//    }
//}
//
#[derive(Debug, DBData)]
pub struct Publicaciones {
    pub id_publicacion: u32,
    pub autores: String,
    pub anio: i32,
    pub titulo: String,
}

impl Dummy<Faker> for Publicaciones {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let id_publicacion = rng.gen();
        let anio = rng.gen_range(1901..2155);
        let titulo: String = Word().fake();
        let autores = (1..rng.gen_range(2..5))
            .map(|_| {
                let nombre: String = FirstName().fake();
                let apellido: String = LastName().fake();
                format!("{apellido}, {nombre}")
            })
            .collect::<Vec<String>>()
            .join("; ");
        Self {
            id_publicacion,
            autores,
            anio,
            titulo,
        }
    }
}

//impl DBData for Publicaciones {
//    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
//        match sqlx::query!(
//            r#"
//            insert into Publicaciones (IDPublicacion, Autores, Anio, Titulo)
//            values (?,?,?,?)
//            "#,
//            self.id_publicacion,
//            self.autores,
//            self.anio,
//            self.titulo
//        )
//        .execute(pool)
//        .await
//        {
//            Ok(_) => (),
//            Err(err) => {
//                eprintln!("Error: {}", err);
//            }
//        };
//        Ok(())
//    }
//}

#[derive(Debug, DBData)]
pub struct ReunionesCientificas {
    pub titulo: String,
    pub fecha: Date,
}

impl Dummy<Faker> for ReunionesCientificas {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        let titulo = Words(1..5).fake::<Vec<String>>().join(" ");
        Self {
            titulo,
            fecha: Date().fake(),
        }
    }
}

//impl DBData for ReunionesCientificas {
//    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
//        match sqlx::query!(
//            r#"
//            insert into ReunionesCientificas(Titulo, Fecha)
//            values (?,?)
//            "#,
//            self.titulo,
//            self.fecha
//        )
//        .execute(pool)
//        .await
//        {
//            Ok(_) => (),
//            Err(err) => {
//                eprintln!("Error: {}", err);
//            }
//        };
//        Ok(())
//    }
//}

#[derive(Debug, DBData)]
pub struct DependenciasOEmpresas {
    pub dni_profesor: Dni, //WARN: FK de Profesores
    pub nombre: String,
    pub fecha_ingreso: Date,
    pub cargo: String,
    pub lugar: Option<String>,
    pub tipo_actividad: String, // ('Autonomo', 'Dependencia')
    pub obra_social: String,
    pub observacion: String,
    pub naturaleza_juridica: String, // ('Privado', 'Publico')
}

impl DependenciasOEmpresas {
    pub fn new(profesor: &Profesores) -> Self {
        let dni_profesor = profesor.dni.clone();
        let nombre = CompanyName().fake();
        let fecha_ingreso = Date().fake();
        let cargo = Word().fake();
        let lugar = Word().fake();
        let tipo_actividad = ["Autonomo", "Dependencia"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        let obra_social = Word().fake();
        let observacion = Words(1..20).fake::<Vec<String>>().join(" ");
        let naturaleza_juridica = ["Privado", "Publico"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        Self {
            dni_profesor,
            nombre,
            fecha_ingreso,
            cargo,
            lugar,
            tipo_actividad,
            obra_social,
            observacion,
            naturaleza_juridica,
        }
    }
}

//impl DBData for DependenciasOEmpresas {
//    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
//        match sqlx::query!(
//            r#"
//            insert into DependenciasOEmpresas (
//                DNIProfesor, Nombre, FechaIngreso, Cargo, Lugar, TipoActividad, ObraSocial, Observacion, NaturalezaJuridica
//            )
//            values (?,?,?,?,?,?,?,?,?)
//            "#,
//            self.dni_profesor,
//            self.nombre,
//            self.fecha_ingreso,
//            self.cargo,
//            self.lugar,
//            self.tipo_actividad,
//            self.obra_social,
//            self.observacion,
//            self.naturaleza_juridica
//        )
//        .execute(pool)
//        .await
//        {
//            Ok(_) => (),
//            Err(err) => {
//                eprintln!("Error: {}", err);
//            }
//        };
//        Ok(())
//    }
//}

#[derive(Debug, DBData)]
pub struct ObrasSociales {
    pub id_obra_social: u32,
    pub dni_profesor: Dni, //WARN: FK de Profesores
    pub dni_beneficiario: Option<Dni>,
    pub tipo_personal: String, // ('No Docente', 'Docente', 'Contratado', 'Becario')
    pub tipo_caracter: String, // ('Titular', 'Suplente', 'Graduado', 'Estudiante', 'Interino')
    pub presta_servicios: bool,
    pub dependencia: String,
}

impl ObrasSociales {
    pub fn new(profesor: &Profesores, beneficiario: Option<&Beneficiarios>) -> Self {
        let mut rng = GLOBAL_RNG.lock().unwrap();
        let id_obra_social = rng.gen();
        let dni_profesor = profesor.dni.clone();
        let dni_beneficiarios = beneficiario.map(|b| b.dni.clone());
        let tipo_personal = ["No Docente", "Docente", "Contratado", "Becario"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        let tipo_caracter = ["Titular", "Suplente", "Graduado", "Estudiante", "Interino"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        let presta_servicios = rng.gen::<bool>();
        let dependencia: String = Word().fake();

        Self {
            id_obra_social,
            dni_profesor,
            dni_beneficiario: dni_beneficiarios,
            tipo_personal,
            tipo_caracter,
            presta_servicios,
            dependencia,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Percepciones {
    pub institucion_caja: String,
    pub tipo: String,
    pub regimen: String,
    pub causa: String,
}

impl Dummy<Faker> for Percepciones {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Self {
            institucion_caja: Words(1..10).fake::<Vec<String>>().join(" "),
            tipo: Words(1..3).fake::<Vec<String>>().join(" "),
            regimen: Word().fake(),
            causa: Words(1..5).fake::<Vec<String>>().join(" "),
        }
    }
}

#[derive(Debug, DBData)]
pub struct DeclaracionesJuradas {
    pub id_declaracion: u32,
    pub dni_profesor: Dni, // WARN: FK de Profesores
    pub fecha: Date,
    pub lugar: String,
}

impl DeclaracionesJuradas {
    pub fn new(profesor: &Profesores) -> Self {
        let id_declaracion = GLOBAL_RNG.lock().unwrap().gen();
        let dni_profesor = profesor.dni.clone();
        let lugar = CityName().fake();
        let fecha = Date().fake();

        Self {
            id_declaracion,
            dni_profesor,
            fecha,
            lugar,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Direcciones {
    pub codigo_postal: u32,
    pub calle: String,
    pub numero: u32,
    pub localidad: Option<String>,
    pub provincia: Option<String>,
}

impl Dummy<Faker> for Direcciones {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let numero = BuildingNumber()
            .fake::<String>()
            .parse()
            .expect("Fallo en transformar  String a u32 'BuildingNumber()'");

        let calle = StreetName().fake();
        let codigo_postal = rng.gen_range(1000..10000);
        let tiene_localidad_y_provincia = rng.gen::<bool>();
        let localidad = if tiene_localidad_y_provincia {
            Some(CityName().fake())
        } else {
            None
        };
        let provincia = if tiene_localidad_y_provincia {
            Some(StateName().fake())
        } else {
            None
        };

        Self {
            codigo_postal,
            calle,
            numero,
            localidad,
            provincia,
        }
    }
}

#[derive(Debug, DBData)]
pub struct DeclaracionesDeCargo {
    pub id_declaracion: u32,
    pub cumple_horario: String,
    pub reparticion: String,
    pub dependencia: String,
    //WARN: FK de Direcciones
    pub codigo_postal: u32,
    pub calle: String,
    pub numero: u32,
}

impl DeclaracionesDeCargo {
    pub fn new(direccion: &Direcciones) -> Self {
        let id_declaracion = GLOBAL_RNG.lock().unwrap().gen();
        let cumple_horario = Word().fake();
        let reparticion = Word().fake();
        let dependencia = Word().fake();
        let codigo_postal = direccion.codigo_postal;
        let calle = direccion.calle.clone();
        let numero = direccion.numero;
        Self {
            id_declaracion,
            cumple_horario,
            reparticion,
            dependencia,
            codigo_postal,
            calle,
            numero,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Horarios {
    pub id_declaracion: u32, //WARN: FK de DeclaracionesDeCargo
    pub dia: String,         // ('Lunes','Martes','Miercoles','Jueves','Viernes')
    pub rango_horario: String,
    pub nombre_catedra: String,
}

impl Horarios {
    pub fn new(declaracion: &DeclaracionesDeCargo) -> Self {
        let id_declaracion = declaracion.id_declaracion;
        let dia = ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        let rango_horario = generar_rangos_horarios();
        let nombre_catedra = Words(1..5).fake::<Vec<String>>().join(" ");
        Self {
            id_declaracion,
            dia,
            rango_horario,
            nombre_catedra,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Empleadores {
    pub cuit: Cuil,
    pub razon_social: String,
    pub piso: Option<u32>,
    pub departamento: Option<u8>,
    //WARN: FK de Direcciones
    pub codigo_postal: u32,
    pub calle: String,
    pub numero: u32,
}

impl Empleadores {
    pub fn new(direccion: &Direcciones) -> Self {
        let mut rng = GLOBAL_RNG.lock().unwrap();
        let razon_social = Name().fake();
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
        let dni: Dni = Faker.fake();
        Self {
            cuit: Cuil::new(&dni),
            razon_social,
            piso,
            departamento,
            codigo_postal: direccion.codigo_postal,
            calle: direccion.calle.clone(),
            numero: direccion.numero,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Seguros {
    pub codigo_compania: u32,
    pub compania_aseguradora: String,
    pub lugar_emision: String,
    pub fecha_emision: Date,
}

impl Dummy<Faker> for Seguros {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let codigo_compania = rng.gen();
        let compania_aseguradora = CompanyName().fake();
        let lugar_emision = CityName().fake();
        let fecha_emision = Date().fake();

        Self {
            codigo_compania,
            compania_aseguradora,
            lugar_emision,
            fecha_emision,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Beneficiarios {
    pub dni: Dni,
    pub nombre: String,
    pub apellido: String,
    pub parentesco: String,
    pub fecha_nacimiento: Date,
    pub tipo_documento: String,
    pub porcentaje: BigDecimal,
    pub piso: Option<u32>,
    pub departamento: Option<u8>,
    //WARN: FK de Direcciones
    pub numero_dir: u32,
    pub codigo_postal: u32,
    pub calle: String,
}

impl Beneficiarios {
    pub fn new(direccion: &Direcciones) -> Self {
        let mut rng = GLOBAL_RNG.lock().unwrap();
        let dni = Faker.fake();
        let nombre = FirstName().fake();
        let apellido = LastName().fake();
        let parentesco = ["Cónyuge", "Hijo", "Padre", "Pareja", "Hermano"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        let fecha_nacimiento = Date().fake();
        //FIXME: Que tipos de documento pueden ser?
        let tipo_documento = Word().fake();
        // FIXME: Va a ocurrir que entre los beneficiarios o no cubriran el 100% o sobrepasaran el
        // 100%, por como está definido esto.
        let porcentaje = BigDecimal::new(rng.gen_range(1..2).into(), rng.gen_range(1..27));
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
        let numero_dir = direccion.numero;
        let codigo_postal = direccion.codigo_postal;
        let calle = direccion.calle.clone();

        Self {
            dni,
            nombre,
            apellido,
            parentesco,
            fecha_nacimiento,
            tipo_documento,
            porcentaje,
            piso,
            departamento,
            numero_dir,
            codigo_postal,
            calle,
        }
    }
}

#[derive(Debug, Clone, Type)]
#[sqlx(transparent)]
// https://servicioscf.afip.gob.ar/publico/abc/ABCpaso2.aspx?id_nivel1=3036&id_nivel2=3040&p=Conceptos%20b%C3%A1sicos
pub struct Cuil(String);
impl Cuil {
    fn new(dni: &Dni) -> Self {
        let dni = dni.0.clone();
        Self(format!("20{dni}8"))
    }
}

#[derive(Debug, Clone, Type)]
#[sqlx(transparent)]
pub struct Dni(String);

impl Dummy<Faker> for Dni {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let dni: String = (0..8).map(|_| rng.gen_range(0..10).to_string()).collect();
        Self(dni)
    }
}

fn generar_rangos_horarios() -> String {
    let tiempo_comienzo: String = Time().fake();
    let tiempo_fin: String = Time().fake();
    format!("{tiempo_comienzo} - {tiempo_fin}")
}
