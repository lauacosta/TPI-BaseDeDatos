# Trabajo Practico Integrador Base de Datos 
El siguiente proyecto fue realizado por los alumnos de la Universidad Tecnológica Nacional sede Resistencia. 

El repositorio en GitHub se encuentra [`aquí`](https://github.com/lauacosta/BaseDeDatos).

La estructura de la aplicación, ignorando la carpeta para los macros, es la siguiente:
```
.
└── carga_datos
   ├── .env
   ├── .gitignore
   ├── bin
   │  ├── Linux
   │  │  └── entrega
   │  └── Windows
   │     └── entrega.exe
   ├── Cargo.lock
   ├── Cargo.toml
   ├── datasets
   │  ├── provincia_localidad_calles.csv
   │  ├── idiomas.csv
   │  └── universidades.csv
   ├── migrations
   │  └── 20240606032226_cargar_tablas.sql
   ├── README.md
   ├── scripts
   │  └── compilar_binarios.sh
   └── src
      ├── datasets.rs
      ├── db_tablas.rs
      ├── lib.rs
      └── main.rs
```


### Ejecución
1. Se necesita declarar la URL de la base de datos dentro de un archivo .env en ./carga_datos/
```
$ // Reemplazar con los datos correspondientes:
$ DATABASE_URL=mysql://<usuario>:<contraseña>@localhost/<base de datos>
```
2. En la carpeta 'bin' se encuentran los binarios para cada plataforma.
```
$ // Si se ejecuta desde Linux:
$ ./bin/Linux/entrega -h

$ // Si se ejecuta desde Windows:
$ ./bin/Windows/entrega -h
```

En caso de que por algún motivo no funcionen, puede seguir los pasos para [compilarlo](#Build).

## Build
> [!IMPORTANT] 
> La versión mínima de rustc para poder compilar el programa es v1.75.0 .

1. Sigue las instrucciones en la página oficial de [`Rust`](https://www.rust-lang.org/) para instalar Rust y Cargo de acuerdo a tu sistema.
2. Instala la interfaz de línea de comandos de sqlx:
```
$ cargo install sqlx-cli
```
3. Crear la base de datos usando sqlx.
```
$ sqlx db create
```
4. Ejecutar las migraciones.
```
$ sqlx migrate run
```
5. Para ejecutar el programa:
```
// Para ejecutarlo sin compilarlo:
$ cargo run 

// Compilarlo y ejecutarlo:
$ cargo build --release
$ ./target/release/entrega -c 1000  
```
