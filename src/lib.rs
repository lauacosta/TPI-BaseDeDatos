#![allow(dead_code)]

use bigdecimal::num_bigint::BigInt;
use fake::faker::{
    address::en::*,
    company::en::CompanyName,
    internet::en::SafeEmail,
    lorem::en::*,
    name::en::*,
    phone_number::en::{CellNumber, PhoneNumber},
    time::en::Date,
};
use fake::Fake;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};
use sqlx::types::{time::Date, BigDecimal};
use time::Duration;

#[derive(Debug)]
pub struct Profesores {
    pub dni: BigDecimal,
    pub nombre: String,
    pub apellido: String,
    pub fecha_nacimiento: Date,
    pub nacionalidad: String,
    //TODO: Evaluar si utilizo un enum o solamente al generar los valores los genero en base a un set.
    pub estado_civil: String, // ('Soltero/a', 'Casado/a', 'Divorciado/a', 'Viudo/a', 'Conviviente')
    pub sexo: String,         // ('M', 'F')
    pub cuit: BigDecimal,
    pub cuil: BigDecimal,
    pub cuit_empleador: BigDecimal, //FIXME: FK de Empleador
}

impl Profesores {
    pub fn new(empleador: &Empleadores) -> Self {
        let dni = DNI::new();
        let nombre = FirstName().fake();
        let apellido = LastName().fake();
        let estado_civil = [
            "Soltero/a",
            "Casado/a",
            "Divorciado/a",
            "Viudo/a",
            "Conviviente",
        ]
        .choose(&mut thread_rng())
        .unwrap()
        .to_string();
        let sexo = ["M", "F"].choose(&mut thread_rng()).unwrap().to_string();
        let fecha_nacimiento = Date().fake();
        let nacionalidad = CountryName().fake();
        let cuit = CUIL::new();
        let cuil = CUIL::new();
        let cuit_empleador = empleador.cuit_cuil.clone();

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

pub struct Contactos {
    pub dni_profesor: BigDecimal, //FIXME: FK de Profesores
    pub tipo: String,             // ('Celular', 'Telefono', 'Email')
    pub medio: String,            // ('Personal', 'Empresarial', 'Otro')
    pub direccion: Option<String>,
    pub numero: Option<String>,
}

impl Contactos {
    pub fn new(profesor: &Profesores) -> Self {
        let dni_profesor = profesor.dni.clone();
        let tipo = ["Celular", "Telefono", "Email"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();

        let medio = ["Personal", "Empresarial", "Otro"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        // FIXME: Revisar por qué a veces el tipo es Email pero no se le asigna un mail.
        let direccion = match tipo.as_str() {
            "Email" => SafeEmail().fake(),
            _ => None,
        };
        let numero: Option<String> = match tipo.as_str() {
            "Telefono" => PhoneNumber().fake(),
            "Celular" => CellNumber().fake(),
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

pub struct Idiomas(pub String);

impl Idiomas {
    pub fn new() -> Self {
        let idioma = [
            "Inglés",
            "Español",
            "Portugues",
            "Mandarín",
            "Japones",
            "Italiano",
        ]
        .choose(&mut thread_rng())
        .unwrap()
        .to_string();

        Self(idioma)
    }
}

//FIXME: struct ConoceIdioma?

pub struct Titulos {
    pub institucion: String,
    pub nivel: String,
    pub titulo: String,
}
impl Titulos {
    pub fn new() -> Self {
        Self {
            institucion: Word().fake(),
            nivel: Word().fake(),
            titulo: Word().fake(),
        }
    }
}

//FIXME: struct PoseeTitulo?

pub struct CursoOConferencia {
    pub nombre: String,
    pub institucion: String,
    pub descripcion: Option<String>,
    pub tipo: String, // ('Curso', 'Conferencia')
}

impl CursoOConferencia {
    pub fn new() -> Self {
        let nombre: String = Name().fake();
        let institucion: String = Word().fake();
        let mut rng = thread_rng();
        let descripcion = if rng.gen::<bool>() {
            let aux: Vec<String> = Words(1..20).fake();
            Some(aux.join(" "))
        } else {
            None
        };
        let tipo = ["Curso", "Conferencia"]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        Self {
            nombre,
            institucion,
            descripcion,
            tipo,
        }
    }
}

//FIXME: struct AtendioA?

pub struct AntecedentesDocentes {
    pub institucion: String,
    pub unidad_academica: String,
    pub cargo: String,
    pub desde: Date,
    pub hasta: Option<Date>,
    pub dedicacion: u32,
    pub dni_profesor: BigDecimal, // FIXME: FK de Profesores
}

impl AntecedentesDocentes {
    pub fn new(profesor: &Profesores) -> Self {
        let mut rng = thread_rng();
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

pub struct ActividadesInvestigacion {
    pub id_investigacion: u32,
    pub institucion: String,
    pub categoria: String,
    pub area_ppal: String,
}

impl ActividadesInvestigacion {
    pub fn new() -> Self {
        let id_investigacion = thread_rng().gen();
        let institucion = Word().fake();
        let categoria = Word().fake();
        let area_ppal = Word().fake();
        Self {
            id_investigacion,
            institucion,
            categoria,
            area_ppal,
        }
    }
}

//FIXME: struct ParticipaEnInvestigacion?

pub struct ActividadesExtensionUniversitaria {
    pub id_actividad: u32,
    pub institucion: String,
    pub cargo: String,
    pub categoria: String,
}

impl ActividadesExtensionUniversitaria {
    pub fn new() -> Self {
        let id_actividad = thread_rng().gen();
        let institucion = Word().fake();
        let cargo = Word().fake();
        let categoria = Word().fake();
        Self {
            id_actividad,
            institucion,
            cargo,
            categoria,
        }
    }
}

//FIXME: struct RealizoActividad?

pub struct AntecedentesProfesionales {
    pub dni_profesor: BigDecimal, //FIXME: FK de Profesores
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
        let hasta = desde + Duration::days(365 * thread_rng().gen_range(1..5));
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

pub struct Publicaciones {
    pub id_publicacion: u32,
    pub autores: String,
    // FIXME: Tengo que extraer el año de esta fecha
    pub anio: i32,
    pub titulo: String,
}

impl Publicaciones {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let id_publicacion = rng.gen();
        let aux: Vec<String> = (1..rng.gen_range(2..5))
            .map(|_| {
                let nombre: String = FirstName().fake();
                let apellido: String = LastName().fake();
                format!("{apellido}, {nombre}")
            })
            .collect();
        let autores = aux.join("; ");
        let anio = rng.gen_range(1901..2155);
        let titulo: String = Word().fake();

        Self {
            id_publicacion,
            autores,
            anio,
            titulo,
        }
    }
}

//FIXME: struct ReferenciaBibligrafica?
//FIXME: struct PublicoPublicacion?

pub struct ReunionesCientificas {
    titulo: String,
    fecha: Date,
}

//FIXME: struct ParticipoEnReunion?

pub struct DependenciasOEmpresas {
    dni_profesor: BigDecimal, //FIXME: FK de Profesores
    nombre: String,
    fecha_ingreso: Date,
    cargo: String,
    lugar: Option<String>,
    tipo_actividad: Option<String>, // ('Autonomo', 'Dependencia')
    obra_social: Option<String>,
    observacion: Option<String>,
    naturaleza_juridica: Option<String>, // ('Privado', 'Publico')
}

pub struct ObrasSociales {
    id_obra_social: u32,
    dni_profesor: BigDecimal, // FIXME: FK de Profesores
    dni_beneficiarios: Option<BigDecimal>,
    tipo_personal: String, // ('No Docente', 'Docente', 'Contratado', 'Becario')
    tipo_caracter: String, // ('Titular', 'Suplente', 'Graduado', 'Estudiante', 'Interino')
    presta_servicios: bool,
    dependencia: String,
}

pub struct Percepciones {
    institucion_caja: String,
    tipo: String,
    regimen: String,
    causa: String,
}

// FIXME:: struct PercibeEn?

pub struct DeclaracionesJuradas {
    id_declaracion: u32,
    dni_profesor: BigDecimal, // FIXME: FK de Profesores
    fecha: Option<Date>,
    lugar: Option<String>,
}

#[derive(Debug)]
pub struct Direcciones {
    pub codigo_postal: u32,
    pub calle: String,
    pub numero: u32,
    pub localidad: Option<String>,
    pub provincia: Option<String>,
}

impl Distribution<Direcciones> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Direcciones {
        let codigo_postal = rng.gen_range(1000..10000);
        let calle = StreetName().fake();
        let numero = rng.gen_range(1..1000);
        let localidad = if rng.gen::<bool>() {
            Some(CityName().fake())
        } else {
            None
        };
        let provincia = if rng.gen::<bool>() {
            Some(StateName().fake())
        } else {
            None
        };

        return Direcciones {
            codigo_postal,
            calle,
            numero,
            localidad,
            provincia,
        };
    }
}

pub struct DeclaracionesDeCargo {
    id_declaracion: u32,
    cumple_horario: String,
    reparticion: String,
    dependencia: String,
    //FIXME: FK de Direcciones
    codigo_postal: u32,
    calle: String,
    numero: u32,
}

pub struct Horarios {
    id_declaracion: u32, //FIXME: FK de DeclaracionesDeCargo
    dia: String,         // ('Lunes','Martes','Miercoles','Jueves','Viernes')
    rango_horario: String,
    nombre_catedra: String,
}

// FIXME: struct CumpleCargo?

#[derive(Debug)]
pub struct Empleadores {
    pub cuit_cuil: BigDecimal,
    pub razon_social: String,
    pub piso: Option<u32>,
    pub departamento: Option<u8>,
    //FIXME: FK de Direcciones
    pub codigo_postal: u32,
    pub calle: String,
    pub numero: u32,
}

impl Empleadores {
    pub fn new(direccion: &Direcciones) -> Self {
        let mut rng = rand::thread_rng();
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

        Self {
            cuit_cuil: CUIL::new(),
            razon_social,
            piso,
            departamento,
            codigo_postal: direccion.codigo_postal,
            calle: direccion.calle.clone(),
            numero: direccion.numero,
        }
    }
}

// FIXME: struct ResideEn?

pub struct Seguros {
    codigo_compania: u32,
    compania_aseguradora: String,
    lugar_emision: String,
    fecha_emision: Date,
}

pub struct Beneficiarios {
    dni: BigDecimal,
    nombre: String,
    apellido: String,
    parentesco: String,
    fecha_nacimiento: Date,
    tipo_documento: String,
    porcentaje: BigDecimal,
    piso: Option<u32>,
    departamento: Option<char>,
    //FIXME: FK de Direcciones
    numero_dir: u32,
    codigo_postal: u32,
    calle: String,
}

// FIXME: struct AseguraA?

struct CUIL;

impl CUIL {
    fn new() -> BigDecimal {
        let mut rng = rand::thread_rng();
        //FIXME: VER COMO GENERAR NUMEROS de 11 cifras.
        let digits: BigInt = rng.gen_range(100000000..1000000000).into();
        BigDecimal::new(digits.into(), 0)
    }
}

struct DNI;
impl DNI {
    fn new() -> BigDecimal {
        let mut rng = rand::thread_rng();
        let digits: BigInt = rng.gen_range(10000000..100000000).into();
        BigDecimal::new(digits, 0)
    }
}
