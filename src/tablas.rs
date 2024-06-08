use bigdecimal::num_bigint::BigInt;
use fake::faker::{
    address::en::*,
    company::en::CompanyName,
    internet::en::SafeEmail,
    lorem::en::*,
    name::en::*,
    phone_number::en::{CellNumber, PhoneNumber},
    time::en::{Date, Time},
};
use fake::Faker;
use fake::{Dummy, Fake};
use rand::{seq::SliceRandom, thread_rng, Rng};
use sqlx::{
    types::{time::Date, BigDecimal},
    MySql, Pool,
};
use std::error::Error;
use time::Duration;

pub fn gen_tablas<T: Dummy<Faker>>(muestras: u32) -> Vec<T> {
    (1..=muestras).map(|_| Faker.fake()).collect()
}

#[allow(async_fn_in_trait)]
pub trait DBData {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>>;
}

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
    pub cuit: Option<BigDecimal>,
    pub cuil: BigDecimal,
    pub cuit_empleador: BigDecimal, //WARN: FK de Empleador
}

impl Profesores {
    pub fn new(empleador: &Empleadores) -> Self {
        let dni = Dni::new();
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
        let cuil = Cuil::new();
        let cuit = if thread_rng().gen::<bool>() {
            Some(Cuil::new())
        } else {
            None
        };

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

impl DBData for Profesores {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            
            insert into Profesores 
            (DNI, Nombre, Apellido, FechaNacimiento, Nacionalidad, EstadoCivil, Sexo, CUIT, CUIL, CUITEmpleador)
            values (?,?,?,?,?,?,'M',?,?,?)

            "#,
            self.dni,
            self.nombre,
            self.apellido,
            self.fecha_nacimiento,
            self.nacionalidad,
            self.estado_civil,
            // FIXME:: MySQL Error 0100 Data truncated in 'Sexo'
            //self.sexo,
            self.cuit,
            self.cuil,
            self.cuit_empleador
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };

        Ok(())
    }
}

pub struct Contactos {
    pub dni_profesor: BigDecimal, //WARN: FK de Profesores
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
        let numero = match tipo.as_str() {
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

impl DBData for Contactos {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            
            insert into Contactos
            (DNIProfesor, Tipo, Direccion, Medio, Numero) 
            values (?,?,?,?,?)

            "#,
            self.dni_profesor,
            self.tipo,
            self.direccion,
            self.medio,
            self.numero
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
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

impl DBData for Titulos {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            
            insert into Titulos
            (Institucion, Nivel, Titulo) 
            values (?,?,?)

            "#,
            self.institucion,
            self.nivel,
            self.titulo
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

pub struct CursoOConferencia {
    pub nombre: String,
    pub institucion: String,
    pub descripcion: Option<String>,
    pub tipo: String, // ('Curso', 'Conferencia')
}

impl Dummy<Faker> for CursoOConferencia {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        let nombre: String = Name().fake();
        let institucion: String = Word().fake();
        let mut rng = thread_rng();
        let descripcion = if rng.gen::<bool>() {
            Some(Words(1..20).fake::<Vec<String>>().join(" "))
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

impl DBData for CursoOConferencia {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into CursosOConferencias (Nombre, Institucion, Descripcion, Tipo)
            values (?,?,?,?)
            "#,
            self.nombre,
            self.institucion,
            self.descripcion,
            self.tipo
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

pub struct AntecedentesDocentes {
    pub institucion: String,
    pub unidad_academica: String,
    pub cargo: String,
    pub desde: Date,
    pub hasta: Option<Date>,
    pub dedicacion: u32,
    pub dni_profesor: BigDecimal, // WARN: FK de Profesores
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

impl DBData for AntecedentesDocentes {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into AntecedentesDocentes (Institucion, UnidadAcademica, Cargo, Desde, Hasta, Dedicacion, DNIProfesor)
            values (?,?,?,?, ?, ?, ?)
            "#,
            self.institucion,
            self.unidad_academica,
            self.cargo,
            self.desde,
            self.hasta,
            self.dedicacion,
            self.dni_profesor
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

#[derive(Debug, Dummy)]
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

impl DBData for ActividadesInvestigacion {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into ActividadesInvestigacion (IDInvestigacion, Institucion, Categoria, AreaPPAL)
            values (?,?,?,?)
            "#,
            self.id_investigacion,
            self.institucion,
            self.categoria,
            self.area_ppal
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

#[derive(Debug, Dummy)]
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

impl DBData for ActividadesExtensionUniversitaria {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into ActividadesExtensionUniversitaria (IDActividad, Institucion, Cargo, Categoria)
            values (?,?,?,?)
            "#,
            self.id_actividad,
            self.institucion,
            self.cargo,
            self.categoria,
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

pub struct AntecedentesProfesionales {
    pub dni_profesor: BigDecimal, //WARN: FK de Profesores
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

impl DBData for AntecedentesProfesionales {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into AntecedentesProfesionales (DNIProfesor, Cargo, Empresa, TipoActividad, Desde, Hasta)
            values (?,?,?,?,?,?)
            "#,
            self.dni_profesor,
            self.cargo,
            self.empresa,
            self.tipo_actividad,
            self.desde,
            self.hasta,
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
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

impl DBData for Publicaciones {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into Publicaciones (IDPublicacion, Autores, Anio, Titulo)
            values (?,?,?,?)
            "#,
            self.id_publicacion,
            self.autores,
            self.anio,
            self.titulo
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

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

impl DBData for ReunionesCientificas {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into ReunionesCientificas(Titulo, Fecha)
            values (?,?)
            "#,
            self.titulo,
            self.fecha
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

pub struct DependenciasOEmpresas {
    pub dni_profesor: BigDecimal, //WARN: FK de Profesores
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

impl DBData for DependenciasOEmpresas {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into DependenciasOEmpresas (
                DNIProfesor, Nombre, FechaIngreso, Cargo, Lugar, TipoActividad, ObraSocial, Observacion, NaturalezaJuridica
            )
            values (?,?,?,?,?,?,?,?,?)
            "#,
            self.dni_profesor,
            self.nombre,
            self.fecha_ingreso,
            self.cargo,
            self.lugar,
            self.tipo_actividad,
            self.obra_social,
            self.observacion,
            self.naturaleza_juridica
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

pub struct ObrasSociales {
    pub id_obra_social: u32,
    pub dni_profesor: BigDecimal, //WARN: FK de Profesores
    pub dni_beneficiarios: Option<BigDecimal>,
    pub tipo_personal: String, // ('No Docente', 'Docente', 'Contratado', 'Becario')
    pub tipo_caracter: String, // ('Titular', 'Suplente', 'Graduado', 'Estudiante', 'Interino')
    pub presta_servicios: bool,
    pub dependencia: String,
}

impl ObrasSociales {
    pub fn new(profesor: &Profesores, beneficiario: Option<&Beneficiarios>) -> Self {
        let id_obra_social = thread_rng().gen();
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
        let presta_servicios = thread_rng().gen::<bool>();
        let dependencia: String = Word().fake();

        Self {
            id_obra_social,
            dni_profesor,
            dni_beneficiarios,
            tipo_personal,
            tipo_caracter,
            presta_servicios,
            dependencia,
        }
    }
}

impl DBData for ObrasSociales {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into ObrasSociales (
                IDObraSocial, DNIBeneficiarios, DNIProfesor, TipoPersonal, TipoCaracter, PrestaServicios, Dependencia
            )
            values (?,?,?,?,?,?,?)
            "#,
            self.id_obra_social,
            self.dni_beneficiarios,
            self.dni_profesor,
            self.tipo_personal,
            self.tipo_caracter,
            self.presta_servicios,
            self.dependencia
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

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

impl DBData for Percepciones {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into Percepciones(
                InstitucionCaja, Tipo, Regimen, Causa 
            )
            values (?,?,?,?)
            "#,
            self.institucion_caja,
            self.tipo,
            self.regimen,
            self.causa
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

pub struct DeclaracionesJuradas {
    pub id_declaracion: u32,
    pub dni_profesor: BigDecimal, // WARN: FK de Profesores
    pub fecha: Date,
    pub lugar: String,
}

impl DeclaracionesJuradas {
    pub fn new(profesor: &Profesores) -> Self {
        let id_declaracion = thread_rng().gen();
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

impl DBData for DeclaracionesJuradas {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into DeclaracionesJuradas(
                IDDeclaracion, DNIProfesor, Fecha, Lugar 
            )
            values (?,?,?,?)
            "#,
            self.id_declaracion,
            self.dni_profesor,
            self.fecha,
            self.lugar
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
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

impl DBData for Direcciones {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"insert into Direcciones (CodigoPostal, Calle, Numero, Localidad, Provincia) 
values (?,?,?,?,?)"#,
            self.codigo_postal,
            self.calle,
            self.numero,
            self.localidad,
            self.provincia,
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };

        Ok(())
    }
}

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
        let id_declaracion = thread_rng().gen();
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

impl DBData for DeclaracionesDeCargo {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into DeclaracionesDeCargo(
                IDDeclaracion, CumpleHorario, Reparticion, Dependencia, CodigoPostal, Calle, Numero 
            )
            values (?,?,?,?,?,?,?)
            "#,
            self.id_declaracion,
            self.cumple_horario,
            self.reparticion,
            self.dependencia,
            self.codigo_postal,
            self.calle,
            self.numero
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

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

impl DBData for Horarios {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into Horarios(
                IDDeclaracion, Dia, RangoHorario, NombreCatedra
            )
            values (?,?,?,?)
            "#,
            self.id_declaracion,
            self.dia,
            self.rango_horario,
            self.nombre_catedra
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct Empleadores {
    pub cuit_cuil: BigDecimal,
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
            cuit_cuil: Cuil::new(),
            razon_social,
            piso,
            departamento,
            codigo_postal: direccion.codigo_postal,
            calle: direccion.calle.clone(),
            numero: direccion.numero,
        }
    }
}

impl DBData for Empleadores {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"insert into Empleadores (CUIT_CUIL, RazonSocial, CodigoPostal, Calle, Numero, Piso, Departamento) 
values (?,?,?,?,?,?,?)"#,
            self.cuit_cuil,
            self.razon_social,
            self.codigo_postal,
            self.calle,
            self.numero,
            self.piso,
            self.departamento
        )
        .execute(pool)
        .await
            {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            };
        Ok(())
    }
}

pub struct Seguros {
    pub codigo_compania: u32,
    pub compania_aseguradora: String,
    pub lugar_emision: String,
    pub fecha_emision: Date,
}

impl Dummy<Faker> for Seguros {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        let codigo_compania = thread_rng().gen();
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

impl DBData for Seguros {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into Seguros(
                CodigoCompania, CompaniaAseguradora, LugarEmision, FechaEmision
            )
            values (?,?,?,?)
            "#,
            self.codigo_compania,
            self.compania_aseguradora,
            self.lugar_emision,
            self.fecha_emision
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

pub struct Beneficiarios {
    pub dni: BigDecimal,
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
        let mut rng = thread_rng();
        let dni = Dni::new();
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
        let porcentaje = BigDecimal::new(
            thread_rng().gen_range(1..2).into(),
            thread_rng().gen_range(1..27),
        );
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

impl DBData for Beneficiarios {
    async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
        match sqlx::query!(
            r#"
            insert into Beneficiarios(
                DNI, Nombre, Apellido, Parentesco, FechaNacimiento, TipoDocumento, Porcentaje,
                NumeroDir, CodigoPostal, Calle, Piso, Departamento
            )
            values (?,?,?,?,?,?,?,?,?,?,?,?)
            "#,
            self.dni,
            self.nombre,
            self.apellido,
            self.parentesco,
            self.fecha_nacimiento,
            self.tipo_documento,
            self.porcentaje,
            self.numero_dir,
            self.codigo_postal,
            self.calle,
            self.piso,
            self.departamento
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
        Ok(())
    }
}

struct Cuil;

impl Cuil {
    fn new() -> BigDecimal {
        let mut rng = rand::thread_rng();
        //FIXME: VER COMO GENERAR NUMEROS de 11 cifras.
        let digits: BigInt = rng.gen_range(100000000..1000000000).into();
        BigDecimal::new(digits, 0)
    }
}

struct Dni;

impl Dni {
    fn new() -> BigDecimal {
        let mut rng = rand::thread_rng();
        let digits: BigInt = rng.gen_range(10000000..100000000).into();
        BigDecimal::new(digits, 0)
    }
}

fn generar_rangos_horarios() -> String {
    let tiempo_comienzo: String = Time().fake();
    let tiempo_fin: String = Time().fake();
    format!("{tiempo_comienzo} - {tiempo_fin}")
}
