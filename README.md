# Trabajo Practico Integrador Base de Datos 
El siguiente trabajo fue realizado por los alumnos de la Universidad Tecnológica Nacional sede Resistencia.

## Setup
1. Sigue las instrucciones en la página oficial de [`Rust`](https://www.rust-lang.org/) para instalar Rust y el manager Cargo de acuerdo a tu sistema.
2. Instala la utilidad de comandos de sqlx:
cargo install sqlx-cli
1. Se necesita declarar la URL de la base de datos en un archivo .env en la misma carpeta que Cargo.toml.
DATABASE_URL=mysql://usuario:contraseña@localhost/base_de_datos
2. Crear la base de datos usando sqlx.
sqlx db create
3. Ejecutar las migraciones.
sqlx migrate run

## Ejecucion
Terminado los pasos del setup, puedes ejecutar el programa utilizando el comando cargo run o compilandolo
