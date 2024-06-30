use crate::{notificar_carga, Notificacion::*};
use colored::Colorize;
use dbdata::DBData;
use dbdata_derive::DBData;
use fake::{
    faker::{
        address::en::*,
        company::en::CompanyName,
        internet::en::SafeEmail,
        job::en::{Field, Position},
        lorem::en::*,
        name::en::*,
        phone_number::en::{CellNumber, PhoneNumber},
        time::en::{Date, Time},
    },
    Dummy, Fake, Faker,
};
use once_cell::sync::Lazy;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use sqlx::{
    types::{time::Date, BigDecimal, Type},
    MySql, Pool,
};
use std::{error::Error, sync::Mutex};
use time::Duration;

static GLOBAL_RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::from_entropy()));

#[derive(Debug, DBData)]
pub struct Instituciones {
    pub nombre: String,
    pub codigo_postal: u32, // WARN: FK Direcciones
    pub calle: String,
    pub numero: u32,
}

impl Instituciones {
    pub fn new(direccion: &Direcciones, nombre: &str) -> Self {
        let codigo_postal = direccion.codigo_postal;
        let calle = direccion.calle.clone();
        let numero = direccion.numero;

        Self {
            nombre: nombre.to_string(),
            codigo_postal,
            calle,
            numero,
        }
    }
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
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
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
        .choose(&mut *rng)
        .unwrap()
        .to_string();
        let sexo = ['M', 'F'].choose(&mut *rng).unwrap().to_string();

        let fecha_nacimiento =
            Date::from_ordinal_date(rng.gen_range(1960..=1980), rng.gen_range(1..=30))
                .expect("Fallo en generar una fecha de nacimiento");

        let nacionalidad = CountryName().fake();
        let cuil = Cuil::new(&dni);
        let cuit = if rng.gen::<bool>() {
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
    pub tipo: String,      // ('Personal', 'Empresarial', 'Otro')
    pub medio: String,     // ('Celular', 'Telefono', 'Email')
    pub direccion: Option<String>,
    pub numero: Option<String>,
}

impl Contactos {
    pub fn new(profesor: &Profesores) -> Self {
        let dni_profesor = profesor.dni.clone();
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let tipo = ["Personal", "Empresarial", "Otro"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();

        let medio = ["Celular", "Telefono", "Email"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();

        let direccion = match medio.as_str() {
            "Email" => Some(SafeEmail().fake()),
            _ => None,
        };
        let numero = match medio.as_str() {
            "Telefono" => Some(PhoneNumber().fake()),
            "Celular" => Some(CellNumber().fake()),
            _ => None,
        };

        match medio.as_str() {
            "Email" => assert_eq!(None, numero),
            "Telefono" | "Celular" => assert_eq!(None, direccion),
            _ => (),
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

#[derive(Debug, DBData, Clone)]
pub struct Titulos {
    pub nivel: String,
    pub titulo: String,
}

impl Dummy<Faker> for Titulos {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let nivel = ["Terciario", "Maestria", "Doctorado", "Tecnicatura"]
            .choose(rng)
            .unwrap()
            .to_string();

        Self {
            nivel,
            titulo: Words(4..5).fake::<Vec<String>>().join(" "),
        }
    }
}

#[derive(Debug, DBData)]
pub struct CursosConferencias {
    pub nombre_inst: String,
    pub nombre_curso: String,
    pub descripcion: Option<String>,
    pub tipo: String, // ('Curso', 'Conferencia')
}

impl CursosConferencias {
    pub fn new(institucion: &Instituciones) -> Self {
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let nombre_inst = institucion.nombre.clone();
        let nombre_curso: String = Name().fake();
        let descripcion = if rng.gen::<bool>() {
            Some(Words(1..20).fake::<Vec<String>>().join(" "))
        } else {
            None
        };
        let tipo = ["Curso", "Conferencia"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();

        Self {
            nombre_inst,
            nombre_curso,
            descripcion,
            tipo,
        }
    }
}

#[derive(Debug, DBData)]
pub struct AntecedentesDocentes {
    pub nombre_inst: String,
    pub unidad_academica: String,
    pub id_declaracion: u32,
    pub dni_profesor: Dni, // WARN: FK de Profesores
    pub desde: Date,
    pub hasta: Option<Date>,
    pub dedicacion: u32,
}

impl AntecedentesDocentes {
    pub fn new(
        profesor: &Profesores,
        institucion: &Instituciones,
        declaracion: &DeclaracionesDeCargo,
    ) -> Self {
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let nombre_inst = institucion.nombre.clone();
        let id_declaracion = declaracion.id_declaracion;

        let unidad_academica = Word().fake();
        let dni_profesor = profesor.dni.clone();
        let desde = profesor.fecha_nacimiento + Duration::days(rng.gen_range(30..50) * 365);
        let hasta = if rng.gen::<bool>() {
            Some(desde + Duration::days(rng.gen_range(1..30) * 365))
        } else {
            None
        };
        let dedicacion = rng.gen_range(1..8);

        Self {
            nombre_inst,
            unidad_academica,
            id_declaracion,
            dni_profesor,
            desde,
            hasta,
            dedicacion,
        }
    }
}

#[derive(Debug, Dummy, DBData)]
pub struct ActividadesInvestigacion {
    pub id_investigacion: u32,
    pub nombre_inst: String,
    pub categoria: String,
    pub area_ppal: String,
}

impl ActividadesInvestigacion {
    pub fn new(institucion: &Instituciones) -> Self {
        let id_investigacion = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex")
            .gen();
        let nombre_inst = institucion.nombre.clone();
        let categoria = Word().fake();
        let area_ppal: String = Field().fake();
        Self {
            id_investigacion,
            nombre_inst,
            categoria,
            area_ppal,
        }
    }
}

#[derive(Debug, DBData)]
pub struct ActividadesExtensionUniversitaria {
    pub id_actividad: u32,
    pub nombre_inst: String,
    pub cargo: String,
    pub categoria: String,
}

impl ActividadesExtensionUniversitaria {
    pub fn new(institucion: &Instituciones) -> Self {
        let id_actividad = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex")
            .gen();
        let nombre_inst = institucion.nombre.clone();
        let cargo: String = Position().fake();
        let categoria = Word().fake();
        Self {
            id_actividad,
            nombre_inst,
            cargo,
            categoria,
        }
    }
}

#[derive(Debug, DBData)]
pub struct AntecedentesProfesionales {
    pub dni_profesor: Dni, //WARN: FK de Profesores
    pub id_declaracion: u32,
    pub tipo_actividad: String,
    pub desde: Date,
    pub hasta: Date,
}

impl AntecedentesProfesionales {
    pub fn new(profesor: &Profesores, declaracion: &DeclaracionesDeCargo) -> Self {
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let dni_profesor = profesor.dni.clone();
        let id_declaracion = declaracion.id_declaracion;
        let tipo_actividad = Word().fake();
        let desde = profesor.fecha_nacimiento + Duration::days(rng.gen_range(30..50) * 365);
        let hasta = desde + Duration::days(365 * rng.gen_range(1..5));
        Self {
            dni_profesor,
            id_declaracion,
            tipo_actividad,
            desde,
            hasta,
        }
    }
}

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

#[derive(Debug, DBData)]
pub struct DependenciasEmpresas {
    pub dni_profesor: Dni, //WARN: FK de Profesores
    pub nombre: String,
    pub tipo_actividad: String, // ('Autonomo', 'Dependencia')
    pub observacion: String,
    pub naturaleza_juridica: String, // ('Privado', 'Publico')
    pub codigo_postal: u32,
    pub calle: String,
    pub numero: u32,
    pub id_obrasocial: u32,
}

impl DependenciasEmpresas {
    pub fn new(profesor: &Profesores, direccion: &Direcciones, obra: &ObrasSociales) -> Self {
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let dni_profesor = profesor.dni.clone();
        let nombre = CompanyName().fake();
        let tipo_actividad = ["Autonomo", "Dependencia"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();
        let observacion = Words(1..20).fake::<Vec<String>>().join(" ");
        let naturaleza_juridica = ["Privado", "Publico"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();

        Self {
            dni_profesor,
            nombre,
            tipo_actividad,
            observacion,
            naturaleza_juridica,
            codigo_postal: direccion.codigo_postal,
            calle: direccion.calle.clone(),
            numero: direccion.numero,
            id_obrasocial: obra.id_obrasocial,
        }
    }
}

#[derive(Debug, Dummy, DBData)]
pub struct ObrasSociales {
    #[dummy(faker = "CompanyName()")]
    pub nombre_obra: String,
    #[dummy(faker = "..")]
    pub id_obrasocial: u32,
}

#[derive(Debug, DBData)]
pub struct DocObraSocial {
    pub id_doc: u32,
    pub id_obra_social: u32,
    pub dni_profesor: Dni,     //WARN: FK de Profesores
    pub tipo_personal: String, // ('No Docente', 'Docente', 'Contratado', 'Becario')
    pub tipo_caracter: String, // ('Titular', 'Suplente', 'Graduado', 'Estudiante', 'Interino')
    pub presta_servicios: bool,
    pub dependencia: String,
}

impl DocObraSocial {
    pub fn new(profesor: &Profesores, obra: &ObrasSociales) -> Self {
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let id_doc = rng.gen();
        let id_obra_social = obra.id_obrasocial;
        let dni_profesor = profesor.dni.clone();
        let tipo_personal = ["No Docente", "Docente", "Contratado", "Becario"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();
        let tipo_caracter = ["Titular", "Suplente", "Graduado", "Estudiante", "Interino"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();
        let presta_servicios = rng.gen::<bool>();
        let dependencia: String = Word().fake();

        Self {
            id_doc,
            id_obra_social,
            dni_profesor,
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
        let id_declaracion = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex")
            .gen();
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
    pub localidad: String,
    pub provincia: String,
}

impl Direcciones {
    pub fn new(provincia: &str, localidad: &str, calle: &str) -> Self {
        let mut rng = GLOBAL_RNG.lock().unwrap();
        let numero = BuildingNumber()
            .fake::<String>()
            .parse()
            .expect("Fallo en transformar String a u32 'BuildingNumber()'");

        let codigo_postal = rng.gen_range(1000..10000);

        Self {
            codigo_postal,
            calle: calle.to_string(),
            numero,
            localidad: localidad.to_string(),
            provincia: provincia.to_string(),
        }
    }
}

#[derive(Debug, DBData)]
pub struct DeclaracionesDeCargo {
    pub dni_profesor: Dni,
    pub nombre_dep: String,
    pub id_declaracion: u32,
    pub cumple_horario: String,
    pub reparticion: String,
    pub dependencia: String,
}

impl DeclaracionesDeCargo {
    pub fn new(dep: &DependenciasEmpresas) -> Self {
        let id_declaracion = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex")
            .gen();
        let dni_profesor = dep.dni_profesor.clone();
        let nombre_dep = dep.nombre.clone();
        let cumple_horario = Word().fake();
        let reparticion = Word().fake();
        let dependencia = Word().fake();
        Self {
            dni_profesor,
            nombre_dep,
            id_declaracion,
            cumple_horario,
            reparticion,
            dependencia,
        }
    }
}

#[derive(Debug, DBData)]
pub struct Horarios {
    pub id_declaracion: u32, //WARN: FK de DeclaracionesDeCargo
    pub dia: String,         // ('Lunes','Martes','Miercoles','Jueves','Viernes')
    pub hora_inicio: time::Time,
    pub hora_fin: time::Time,
    pub nombre_catedra: String,
}

impl Horarios {
    pub fn new(declaracion: &DeclaracionesDeCargo) -> Self {
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let id_declaracion = declaracion.id_declaracion;
        let dia = ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();

        let hora_inicio = Time().fake();
        let hora_fin = hora_inicio + Duration::hours(rng.gen_range(1..=8));
        let nombre_catedra = Words(1..5).fake::<Vec<String>>().join(" ");
        Self {
            id_declaracion,
            dia,
            hora_inicio,
            hora_fin,
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
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
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
pub struct Familiares {
    pub dni_profesor: Dni,
    pub dni_familiar: Dni,
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

impl Familiares {
    pub fn new(direccion: &Direcciones, profesor: &Profesores) -> Self {
        let mut rng = GLOBAL_RNG
            .lock()
            .expect("Fallo en adquirir el rng dentro del Mutex");
        let dni_familiar = Faker.fake();
        let dni_profesor = profesor.dni.clone();
        let nombre = FirstName().fake();
        let apellido = LastName().fake();
        let parentesco = ["Cónyuge", "Hijo", "Padre", "Pareja", "Hermano"]
            .choose(&mut *rng)
            .unwrap()
            .to_string();

        let fecha_nacimiento =
            Date::from_ordinal_date(rng.gen_range(1990..=2015), rng.gen_range(1..=30))
                .expect("Fallo en generar una fecha de nacimiento");

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
            dni_profesor,
            dni_familiar,
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

// https://servicioscf.afip.gob.ar/publico/abc/ABCpaso2.aspx?id_nivel1=3036&id_nivel2=3040&p=Conceptos%20b%C3%A1sicos
#[derive(Debug, Clone, Type)]
#[sqlx(transparent)]
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
