-- Add migration script here
CREATE TABLE Profesores (
    DNI char(8),
    CHECK (DNI REGEXP '^[0-9]{8}$'),
    Nombre varchar(100) NOT NULL,
    Apellido varchar(100) NOT NULL,
    FechaNacimiento date NOT NULL,
    Nacionalidad varchar(100) NOT NULL,
    EstadoCivil enum(
        'Soltero/a',
        'Casado/a',
        'Divorciado/a',
        'Viudo/a',
        'Conviviente'
    ) NOT NULL,
    Sexo enum('M', 'F') NOT NULL,
    CUIT char(11),
    CUIL char(11) NOT NULL,
    CUITEmpleador char(11) NOT NULL,
    modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (DNI)
);

CREATE TABLE Contactos (
    DNIProfesor char(8),
    Tipo enum('Celular', 'Telefono', 'Email'),
    Direccion varchar(100),
    Medio enum('Personal', 'Empresarial', 'Otro'),
    Numero varchar(30),
    PRIMARY KEY (DNIProfesor, Tipo, Medio),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) ON DELETE CASCADE ON
    UPDATE
        CASCADE
);

CREATE TABLE Idiomas (Nombre varchar(50), PRIMARY KEY (Nombre));

CREATE TABLE ConoceIdioma (
    DNIProfesor char(8),
    NombreIdioma varchar(50),
    Certificacion varchar(50) NOT NULL,
    Institucion varchar(50) NOT NULL,
    Nivel varchar(50) NOT NULL,
    PRIMARY KEY (DNIProfesor, NombreIdioma),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
    FOREIGN KEY (NombreIdioma) REFERENCES Idiomas(Nombre)
);

CREATE TABLE Titulos (
    Institucion varchar(100),
    Nivel varchar(50),
    Titulo varchar(100),
    -- FIXME: Temporal, podriamos implementar en la carga que sea un set determinado de titulos y no haría falta esta timestamp
    modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (Institucion, Nivel, Titulo)
);

CREATE TABLE PoseeTitulo (
    DNI char(8),
    Institucion varchar(100),
    Nivel varchar(50),
    Titulo varchar(100),
    Desde date NOT NULL,
    Hasta date NOT NULL,
    PRIMARY KEY (DNI, Institucion, Nivel, Titulo),
    FOREIGN KEY (Institucion, Nivel, Titulo) REFERENCES Titulos (Institucion, Nivel, Titulo),
    FOREIGN KEY (DNI) REFERENCES Profesores(DNI)
);

CREATE TABLE CursosOConferencias (
    Nombre varchar(100),
    Institucion varchar(100),
    Descripcion varchar(255),
    Tipo enum('Curso', 'Conferencia') NOT NULL,
    modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (Nombre, Institucion)
);

CREATE TABLE AtendioA (
    Nombre varchar(50),
    Institucion varchar(100),
    DNI char(8),
    Desde date NOT NULL,
    Hasta date NOT NULL,
    PRIMARY KEY (DNI, Nombre, Institucion),
    FOREIGN KEY (Nombre, Institucion) REFERENCES CursosOConferencias (Nombre, Institucion),
    FOREIGN KEY (DNI) REFERENCES Profesores(DNI)
);

CREATE TABLE AntecedentesDocentes (
    Institucion varchar(50),
    UnidadAcademica varchar(50),
    Cargo varchar(50),
    Desde date NOT NULL,
    Hasta date,
    Dedicacion int UNSIGNED NOT NULL,
    DNIProfesor char(8),
    PRIMARY KEY (DNIProfesor, Institucion, Cargo, UnidadAcademica),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) ON DELETE CASCADE ON
    UPDATE
        CASCADE
);

CREATE TABLE ActividadesInvestigacion (
    IDInvestigacion int UNSIGNED,
    Institucion varchar(50) NOT NULL,
    Categoria varchar(50) NOT NULL,
    AreaPPAL varchar(50) NOT NULL,
    modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (IDInvestigacion)
);

CREATE TABLE ParticipaEnInvestigacion (
    IDInvestigacion int UNSIGNED,
    DNIProfesor char(8),
    Desde date NOT NULL,
    Hasta date,
    Dedicacion int UNSIGNED NOT NULL,
    PRIMARY KEY (IDInvestigacion, DNIProfesor),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
    FOREIGN KEY (IDInvestigacion) REFERENCES ActividadesInvestigacion(IDInvestigacion)
);

CREATE TABLE ActividadesExtensionUniversitaria (
    IDActividad int UNSIGNED,
    Institucion varchar(50) NOT NULL,
    Cargo varchar(50) NOT NULL,
    Categoria varchar(50) NOT NULL,
    modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (IDActividad)
);

CREATE TABLE RealizoActividad(
    IDActividad int UNSIGNED,
    DNIProfesor char(8),
    Acciones varchar(50) NOT NULL,
    Dedicacion int UNSIGNED NOT NULL,
    Hasta date NOT NULL,
    Desde date NOT NULL,
    PRIMARY KEY (DNIProfesor, IDActividad),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
    FOREIGN KEY (IDActividad) REFERENCES ActividadesExtensionUniversitaria(IDActividad)
);

CREATE TABLE AntecedentesProfesionales (
    DNIProfesor char(8),
    Cargo varchar(50),
    Empresa varchar(50),
    TipoActividad varchar(50),
    -- No tendrian que ser no nulos?
    Desde date NOT NULL,
    Hasta date NOT NULL,
    PRIMARY KEY (DNIProfesor, Empresa, TipoActividad, Cargo),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) ON DELETE CASCADE ON
    UPDATE
        CASCADE
);

CREATE TABLE Publicaciones (
    IDPublicacion int UNSIGNED,
    Autores varchar(200) NOT NULL,
    Anio YEAR NOT NULL,
    Titulo varchar(50) NOT NULL,
    modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (IDPublicacion)
);

CREATE TABLE ReferenciaBibliografica (
    IDFuente int UNSIGNED,
    IDCitador int UNSIGNED,
    PRIMARY KEY (IDFuente, IDCitador),
    FOREIGN KEY (IDFuente) REFERENCES Publicaciones(IDPublicacion),
    FOREIGN KEY (IDCitador) REFERENCES Publicaciones(IDPublicacion)
);

CREATE TABLE PublicoPublicacion(
    IDPublicacion int UNSIGNED,
    DNIProfesor char(8),
    PRIMARY KEY (IDPublicacion, DNIProfesor),
    FOREIGN KEY (IDPublicacion) REFERENCES Publicaciones(IDPublicacion),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI)
);

CREATE TABLE ReunionesCientificas (
    Titulo varchar(50),
    Fecha date,
    modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (Titulo, Fecha)
);

CREATE TABLE ParticipoEnReunion (
    DNIProfesor char(8),
    Titulo varchar(50),
    Fecha date,
    Participacion varchar(50),
    PRIMARY KEY (DNIProfesor, Titulo, Fecha),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
    FOREIGN KEY (Titulo, Fecha) REFERENCES ReunionesCientificas(Titulo, Fecha)
);

CREATE TABLE DependenciasOEmpresas (
    DNIProfesor char(8),
    Nombre varchar(50),
    FechaIngreso date,
    Cargo varchar(50),
    Lugar varchar(100),
    TipoActividad enum('Autónomo', 'Dependencia') NOT NULL,
    ObraSocial varchar(50) NOT NULL,
    Observacion varchar(250) NOT NULL,
    NaturalezaJuridica enum('Privado', 'Publico'),
    PRIMARY KEY (DNIProfesor, Nombre, FechaIngreso, Cargo),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) ON DELETE CASCADE ON
    UPDATE
        CASCADE
);

CREATE TABLE ObrasSociales (
    IDObraSocial int UNSIGNED,
    DNIBeneficiario char(8),
    DNIProfesor char(8),
    -- FIXME: REVISAR SI ESTE ATRIBUTO HACE FALTA
    TipoPersonal enum('No Docente', 'Docente', 'Contratado', 'Becario') NOT NULL,
    TipoCaracter enum(
        'Titular',
        'Suplente',
        'Graduado',
        'Estudiante',
        'Interino'
    ) NOT NULL,
    PrestaServicios bool NOT NULL,
    Dependencia varchar(100) NOT NULL,
    PRIMARY KEY (IDObraSocial, DNIProfesor),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE TABLE Percepciones (
    InstitucionCaja varchar(100),
    Tipo varchar(50),
    Regimen varchar(50) NOT NULL,
    Causa varchar(50) NOT NULL,
    Modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (Tipo, InstitucionCaja)
);

CREATE TABLE PercibeEn (
    DNI char(8),
    InstitucionCaja varchar(100),
    Tipo varchar(50),
    EstadoPercepcion enum('Percibiendo', 'Suspendido') NOT NULL,
    Desde date NOT NULL,
    PRIMARY KEY (DNI, Tipo, InstitucionCaja),
    FOREIGN KEY (DNI) REFERENCES Profesores (DNI),
    FOREIGN KEY (Tipo, InstitucionCaja) REFERENCES Percepciones (Tipo, InstitucionCaja)
);

CREATE TABLE DeclaracionesJuradas (
    IDDeclaracion int UNSIGNED,
    DNIProfesor char(8),
    Fecha date NOT NULL,
    Lugar varchar(100) NOT NULL,
    PRIMARY KEY (DNIProfesor, IDDeclaracion),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE TABLE Direcciones (
    CodigoPostal int UNSIGNED,
    Calle varchar(100),
    Numero int UNSIGNED,
    Localidad varchar(100),
    Provincia varchar(100),
    Modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (CodigoPostal, Calle, Numero)
);

CREATE TABLE DeclaracionesDeCargo (
    IDDeclaracion int UNSIGNED,
    CumpleHorario varchar(100) NOT NULL,
    Reparticion varchar(100) NOT NULL,
    Dependencia varchar(100) NOT NULL,
    CodigoPostal int UNSIGNED NOT NULL,
    Calle varchar(100) NOT NULL,
    Numero int UNSIGNED NOT NULL,
    Modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (IDDeclaracion),
        FOREIGN KEY (CodigoPostal, Calle, Numero) REFERENCES Direcciones(CodigoPostal, Calle, Numero)
);

CREATE TABLE Horarios (
    IDDeclaracion int UNSIGNED,
    Dia enum(
        'Lunes',
        'Martes',
        'Miercoles',
        'Jueves',
        'Viernes'
    ),
    RangoHorario varchar(25),
    NombreCatedra varchar(50),
    PRIMARY KEY (IDDeclaracion, Dia, RangoHorario),
    FOREIGN KEY (IDDeclaracion) REFERENCES DeclaracionesDeCargo(IDDeclaracion) ON DELETE CASCADE ON
    UPDATE
        CASCADE
);

CREATE TABLE CumpleCargo (
    DNIProfesor char(8),
    IDDeclaracion int UNSIGNED,
    PRIMARY KEY (DNIProfesor, IDDeclaracion),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
    FOREIGN KEY (IDDeclaracion) REFERENCES DeclaracionesDeCargo(IDDeclaracion)
);

CREATE TABLE Empleadores (
    CUIT char(11),
    RazonSocial varchar(100),
    CodigoPostal int UNSIGNED NOT NULL,
    Calle varchar(100) NOT NULL,
    Numero int UNSIGNED NOT NULL,
    Piso int UNSIGNED,
    Departamento tinyint,
    PRIMARY KEY (CUIT),
    FOREIGN KEY (CodigoPostal, Calle, Numero) REFERENCES Direcciones (CodigoPostal, Calle, Numero)
);

CREATE TABLE ResideEn (
    DNIProfesor char(8),
    CodigoPostal int UNSIGNED,
    Calle varchar(100),
    Numero int UNSIGNED,
    Piso int UNSIGNED,
    Departamento tinyint,
    PRIMARY KEY (DNIProfesor, CodigoPostal, Calle, Numero),
    FOREIGN KEY(CodigoPostal, Calle, Numero) REFERENCES Direcciones (CodigoPostal, Calle, Numero),
    FOREIGN KEY(DNIProfesor) REFERENCES Profesores(DNI)
);

CREATE TABLE Seguros (
    CodigoCompania int UNSIGNED,
    CompaniaAseguradora varchar(100),
    LugarEmision varchar(100),
    FechaEmision date,
    Modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY(CodigoCompania)
);

CREATE TABLE Beneficiarios (
    DNI char(8),
    CHECK (DNI REGEXP '^[0-9]{8}$'),
    Nombre varchar(50) NOT NULL,
    Apellido varchar(50) NOT NULL,
    Parentesco varchar (25) NOT NULL,
    FechaNacimiento date NOT NULL,
    -- FIXME: Que tipos de documento pueden ser?
    TipoDocumento varchar(50) NOT NULL,
    Porcentaje Numeric NOT NULL,
    NumeroDir int UNSIGNED NOT NULL,
    CodigoPostal int UNSIGNED NOT NULL,
    Calle varchar(100) NOT NULL,
    Piso int UNSIGNED,
    Departamento tinyint,
    Modificada TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON
    UPDATE
        CURRENT_TIMESTAMP,
        PRIMARY KEY (DNI),
        FOREIGN KEY (CodigoPostal, Calle, NumeroDir) REFERENCES Direcciones (CodigoPostal, Calle, Numero)
);

CREATE TABLE AseguraA (
    DNIProfesor char(8),
    DNIBeneficiario char(8),
    CodigoCompania int UNSIGNED,
    CapitalAsegurado decimal(10, 3),
    FechaIngreso date,
    PRIMARY KEY (DNIProfesor, CodigoCompania),
    FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
    FOREIGN KEY (CodigoCompania) REFERENCES Seguros (CodigoCompania),
    FOREIGN KEY (DNIBeneficiario) REFERENCES Beneficiarios (DNI)
);

ALTER TABLE
    Profesores
ADD
    CONSTRAINT RefCuit2 FOREIGN KEY (CUITEmpleador) REFERENCES Empleadores (CUIT);

ALTER TABLE
    ObrasSociales
ADD
    CONSTRAINT RefDNIBenef2 FOREIGN KEY (DNIBeneficiario) REFERENCES Beneficiarios(DNI);
