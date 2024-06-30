-- Add migration script here
CREATE TABLE Direcciones (
	CodigoPostal int unsigned,
	Calle varchar(100),
	Numero int unsigned,
	Localidad varchar(100) NOT NULL,
	Provincia varchar(100) NOT NULL,
	PRIMARY KEY (CodigoPostal, Calle, Numero)
);

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
    CHECK (CUIT REGEXP '^[0-9]{11}$'),
	CUIL char(11) NOT NULL,
    CHECK (CUIL REGEXP '^[0-9]{11}$'),
	CUITEmpleador char(11) NOT NULL,
	PRIMARY KEY (DNI)
);

CREATE TABLE Instituciones (
    Nombre varchar(255),
	CodigoPostal int unsigned,
	Calle varchar(100),
	Numero int unsigned,
    PRIMARY KEY (Nombre),
    FOREIGN KEY (CodigoPostal, Calle, Numero) REFERENCES Direcciones(CodigoPostal, Calle, Numero)
);

CREATE TABLE Contactos (
	DNIProfesor char(8),
	Medio enum('Celular', 'Telefono', 'Email'),
	Direccion varchar(100),
	Tipo enum('Personal', 'Empresarial', 'Otro'),
	Numero varchar(30),
	PRIMARY KEY (DNIProfesor, Tipo, Medio),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) 
    ON DELETE CASCADE ON UPDATE CASCADE,
    CHECK (
        (Medio IN ('Celular', 'Telefono') AND Numero IS NOT NULL) OR 
        (Medio = 'Email' AND Direccion IS NOT NULL)
    )
);

CREATE TABLE Idiomas (
	Nombre varchar(50),
	PRIMARY KEY (Nombre)
);

CREATE TABLE SeDaIdioma (
    NombreIdioma varchar(50),
    NombreInst varchar(255),
    PRIMARY KEY(NombreIdioma, NombreInst),
    FOREIGN KEY(NombreIdioma) REFERENCES Idiomas(Nombre),
    FOREIGN KEY(NombreInst) REFERENCES Instituciones(Nombre)
);

CREATE TABLE ConoceIdioma (
	DNIProfesor char(8),
	NombreIdioma varchar(50),
	Certificacion varchar(50) NOT NULL,
	Nivel varchar(50) NOT NULL,
	PRIMARY KEY (DNIProfesor, NombreIdioma),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
	FOREIGN KEY (NombreIdioma) REFERENCES Idiomas(Nombre)
);

CREATE TABLE Titulos (
	Nivel varchar(50),
	Titulo varchar(100),
	PRIMARY KEY (Nivel, Titulo)
);

CREATE TABLE SeDaTitulo (
    Titulo varchar(100),
    NombreInst varchar(255),
    Nivel varchar(50),
    PRIMARY KEY (Nivel, Titulo, NombreInst),
    FOREIGN KEY (Nivel, Titulo) REFERENCES Titulos (Nivel, Titulo),
    FOREIGN KEY (NombreInst) REFERENCES Instituciones (Nombre)
);


CREATE TABLE PoseeTitulo (
	DNI char(8),
	Nivel varchar(50),
	Titulo varchar(100),
	Desde date NOT NULL,
	Hasta date NOT NULL,
	PRIMARY KEY (DNI, Nivel, Titulo),
	FOREIGN KEY (Nivel, Titulo) REFERENCES Titulos (Nivel, Titulo),
	FOREIGN KEY (DNI) REFERENCES Profesores(DNI)
);

CREATE TABLE CursosConferencias (
	NombreCurso varchar(100),
	NombreInst varchar(255),
	Descripcion varchar(255),
	Tipo enum('Curso', 'Conferencia') NOT NULL,
	PRIMARY KEY (NombreCurso),
    FOREIGN KEY (NombreInst) REFERENCES Instituciones(Nombre)
);

CREATE TABLE AtendioA (
	NombreCurso varchar(100),
	DNIProfesor char(8),
	Desde date NOT NULL,
	Hasta date,
	PRIMARY KEY (NombreCurso, DNIProfesor),
	FOREIGN KEY (NombreCurso) REFERENCES CursosConferencias (NombreCurso),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI)
);


CREATE TABLE ActividadesInvestigacion (
	IDInvestigacion int unsigned,
	NombreInst varchar(255) NOT NULL,
	Categoria varchar(50) NOT NULL,
	AreaPPAL varchar(50) NOT NULL,
	PRIMARY KEY (IDInvestigacion),
    FOREIGN KEY (NombreInst) REFERENCES Instituciones(Nombre)
);

CREATE TABLE RealizaInves (
	IDInvestigacion int unsigned,
	DNIProfesor char(8),
	Desde date NOT NULL,
	Hasta date,
	Dedicacion int unsigned NOT NULL,
	PRIMARY KEY (IDInvestigacion, DNIProfesor),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
	FOREIGN KEY (IDInvestigacion) REFERENCES ActividadesInvestigacion(IDInvestigacion)
);

CREATE TABLE ActividadesExtensionUniversitaria (
	IDActividad int unsigned,
	NombreInst varchar(255) NOT NULL,
	Cargo varchar(50) NOT NULL,
	Categoria varchar(50) NOT NULL,
	PRIMARY KEY (IDActividad),
    FOREIGN KEY (NombreInst) REFERENCES Instituciones(Nombre)
);

CREATE TABLE RealizoAct(
	IDActividad int unsigned,
	DNIProfesor char(8),
	Acciones varchar(50) NOT NULL,
	Dedicacion int unsigned NOT NULL,
	Hasta date NOT NULL,
	Desde date NOT NULL,
	PRIMARY KEY (DNIProfesor, IDActividad),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI),
	FOREIGN KEY (IDActividad) REFERENCES ActividadesExtensionUniversitaria(IDActividad)
);

CREATE TABLE ObrasSociales (
    NombreObra varchar(100),
    IDObraSocial int unsigned,
    PRIMARY KEY (IDObraSocial)
);

CREATE TABLE DependenciasEmpresas (
	DNIProfesor char(8),
	Nombre varchar(100),
	TipoActividad enum('Autónomo', 'Dependencia') NOT NULL,
	Observacion varchar(250) NOT NULL,
	NaturalezaJuridica enum('Privado', 'Publico'),
	CodigoPostal int unsigned,
	Calle varchar(100),
	Numero int unsigned,
    IDObraSocial int unsigned,
    PRIMARY KEY (DNIProfesor, Nombre), 
    FOREIGN KEY (IDObraSocial) REFERENCES ObrasSociales(IDObraSocial),
	FOREIGN KEY (CodigoPostal, Calle, Numero) REFERENCES Direcciones(CodigoPostal, Calle, Numero),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI)
    ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE DeclaracionesDeCargo (
    DNIProfesor char(8),
    NombreDep varchar(100),
	IDDeclaracion int unsigned,
	CumpleHorario varchar(100) NOT NULL,
	Reparticion varchar(100) NOT NULL,
	Dependencia varchar(100) NOT NULL,
	PRIMARY KEY (IDDeclaracion),
    FOREIGN KEY (DNIProfesor, NombreDep) REFERENCES DependenciasEmpresas(DNIProfesor, Nombre)
);

CREATE TABLE AntecedentesProfesionales (
	DNIProfesor char(8),
    IDDeclaracion int unsigned,
	TipoActividad varchar(50),
	Desde date not null,
	Hasta date not null,
	PRIMARY KEY (DNIProfesor, TipoActividad),
    FOREIGN KEY(IDDeclaracion) REFERENCES DeclaracionesDeCargo(IDDeclaracion),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI)
    ON DELETE CASCADE ON UPDATE	CASCADE
);

CREATE TABLE Publicaciones (
	IDPublicacion int unsigned,
	Autores varchar(200) NOT NULL,
	Anio YEAR NOT NULL,
	Titulo varchar(50) NOT NULL,
	PRIMARY KEY (IDPublicacion)
);

CREATE TABLE ReferenciaBibliografica (
	IDFuente int unsigned,
	IDCitador int unsigned,
	PRIMARY KEY (IDFuente, IDCitador),
	FOREIGN KEY (IDFuente) REFERENCES Publicaciones(IDPublicacion),
	FOREIGN KEY (IDCitador) REFERENCES Publicaciones(IDPublicacion)
);

CREATE TABLE Publico(
	IDPublicacion int unsigned,
	DNIProfesor char(8),
	PRIMARY KEY (IDPublicacion, DNIProfesor),
	FOREIGN KEY (IDPublicacion) REFERENCES Publicaciones(IDPublicacion),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI)
);

CREATE TABLE ReunionesCientificas (
	Titulo varchar(50),
	Fecha date,
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



CREATE TABLE Familiares (
    DNIProfesor char(8),
	DNIFamiliar char(8),
    CHECK (DNIFamiliar REGEXP '^[0-9]{8}$'),
	Nombre varchar(50) NOT NULL,
	Apellido varchar(50) NOT NULL,
	Parentesco enum ('Cónyuge', 'Hijo', 'Padre', 'Pareja', 'Hermano') NOT NULL,
	FechaNacimiento date NOT NULL,
	TipoDocumento varchar(50) NOT NULL,
	Porcentaje Numeric NOT NULL,
	NumeroDir int unsigned NOT NULL,
	CodigoPostal int unsigned NOT NULL,
	Calle varchar(100) NOT NULL,
	Piso int unsigned,
	Departamento tinyint,
	PRIMARY KEY (DNIFamiliar, DNIProfesor),
	FOREIGN KEY (CodigoPostal, Calle, NumeroDir) REFERENCES Direcciones (CodigoPostal, Calle, Numero),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) 
    ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE Beneficia (
    DNIFamiliar char(8),
    DNIProfesor char(8),
    IDObraSocial int unsigned,
    PRIMARY KEY(IDObraSocial, DNIFamiliar, DNIProfesor),
    FOREIGN KEY (IDObraSocial) REFERENCES ObrasSociales(IDObraSocial),
    FOREIGN KEY (DNIFamiliar, DNIProfesor) REFERENCES Familiares(DNIFamiliar, DNIProfesor)
);

-- FIXME: DEFINIR CLAVE DE OBRA SOCIAL
CREATE TABLE DocObraSocial (
    IDDoc int unsigned,
	IDObraSocial int unsigned,
	DNIProfesor decimal(8),
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
	PRIMARY KEY (IDObraSocial, IDDoc),
    FOREIGN KEY (IDObraSocial) REFERENCES ObrasSociales(IDObraSocial)
    ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE Percepciones (
	InstitucionCaja varchar(100),
	Tipo varchar(50),
	Regimen varchar(50) NOT NULL,
	Causa varchar(50) NOT NULL,
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
	IDDeclaracion int unsigned,
	DNIProfesor char(8),
	Fecha date not null,
	Lugar varchar(100) not null,
	PRIMARY KEY (DNIProfesor, IDDeclaracion),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) ON
	UPDATE CASCADE ON DELETE CASCADE
);



CREATE TABLE AntecedentesDocentes (
	NombreInst varchar(255),
	UnidadAcademica varchar(50),
    IDDeclaracion int unsigned,
    DNIProfesor char(8),
	Desde date NOT NULL,
	Hasta date,
	Dedicacion int unsigned NOT NULL,
	PRIMARY KEY (DNIProfesor, UnidadAcademica),
    FOREIGN KEY (NombreInst) REFERENCES Instituciones(Nombre),
    FOREIGN KEY (IDDeclaracion) REFERENCES DeclaracionesDeCargo(IDDeclaracion),
	FOREIGN KEY (DNIProfesor) REFERENCES Profesores(DNI) 
    ON DELETE CASCADE ON UPDATE	CASCADE
);

CREATE TABLE Horarios (
	IDDeclaracion int unsigned,
	Dia enum(
    	'Lunes',
    	'Martes',
    	'Miercoles',
    	'Jueves',
    	'Viernes'
	),
	HoraInicio time,
	HoraFin time,
	NombreCatedra varchar(50),
	PRIMARY KEY (IDDeclaracion, Dia, HoraInicio, HoraFin),
	FOREIGN KEY (IDDeclaracion) REFERENCES DeclaracionesDeCargo(IDDeclaracion) 
    ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE Empleadores (
	CUIT char(11),
    CHECK (CUIT REGEXP '^[0-9]{11}$'),
	RazonSocial varchar(100),
	CodigoPostal int unsigned NOT NULL,
	Calle varchar(100) NOT NULL,
	Numero int unsigned NOT NULL,
	Piso int unsigned,
	Departamento tinyint,
	PRIMARY KEY (CUIT),
	FOREIGN KEY (CodigoPostal, Calle, Numero) REFERENCES Direcciones (CodigoPostal, Calle, Numero)
);

CREATE TABLE ResideEn (
	DNIProfesor char(8),
	CodigoPostal int unsigned,
	Calle varchar(100),
	Numero int unsigned,
	Piso int unsigned,
	Departamento tinyint,
	PRIMARY KEY (DNIProfesor, CodigoPostal, Calle, Numero),
	FOREIGN KEY(CodigoPostal, Calle, Numero) REFERENCES Direcciones (CodigoPostal, Calle, Numero),
	FOREIGN KEY(DNIProfesor) REFERENCES Profesores(DNI)
);

CREATE TABLE Seguros (
	CodigoCompania int unsigned,
	CompaniaAseguradora varchar(100),
	LugarEmision varchar(100),
	FechaEmision date,
	PRIMARY KEY(CodigoCompania)
);


CREATE TABLE AseguraA (
	DNIProfesor char (8),
	DNIFamiliar char (8),
	CodigoCompania int unsigned,
	CapitalAsegurado Numeric,
	FechaIngreso date,
	PRIMARY KEY (DNIProfesor, DNIFamiliar, CodigoCompania),
	FOREIGN KEY (CodigoCompania) REFERENCES Seguros (CodigoCompania),
	FOREIGN KEY (DNIFamiliar, DNIProfesor) REFERENCES Familiares (DNIFamiliar, DNIProfesor)
);

ALTER TABLE
	Profesores
ADD
	CONSTRAINT RefCuit2 FOREIGN KEY (CUITEmpleador) REFERENCES Empleadores (CUIT);
