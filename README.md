# Trabajo Practico Integrador Base de Datos 
El siguiente proyecto fue realizado por los alumnos de la Universidad Tecnológica Nacional sede Resistencia. 

El repositorio en GitHub se encuentra [`aquí`](https://github.com/lauacosta/BaseDeDatos).

## Setup
1. Sigue las instrucciones en la página oficial de [`Rust`](https://www.rust-lang.org/) para instalar Rust y el manager Cargo de acuerdo a tu sistema.

2. Instala la interfaz de línea de comandos de sqlx:
```
$ cargo install sqlx-cli
```
3. Se necesita declarar la URL de la base de datos dentro de un archivo .env en la misma carpeta que Cargo.toml.
```
$ DATABASE_URL=mysql://usuario:contraseña@localhost/base_de_datos
```
4. Crear la base de datos usando sqlx.
```
$ sqlx db create
```
5. Ejecutar las migraciones.
```
$ sqlx migrate run
```
## Build
Para ejecutar el programa:
```
// Podes ejecutarlo sin compilarlo haciendo:
$ cargo run

// Podes compilarlo y ejecutarlo haciendo:
$ cargo build --release
$ ./target/release/carga_datos
```
