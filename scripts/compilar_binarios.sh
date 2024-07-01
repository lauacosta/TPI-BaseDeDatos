#!/usr/bin/env bash
set -x
set -eo pipefail


if ! [ -x "$(command -v rustc)" ]; then
    echo >&2 "Error: Rust no está instalado."
    echo >&2 "Usa:"
    echo >&2 "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo >&2 "para instalarlo."
    exit 1
fi

if ! [ -x "$(command -v cargo)" ]; then
    echo >&2 "Error: cargo no está instalado."
    echo >&2 "Usa:"
    echo >&2 " rustup component add cargo"
    echo >&2 "para instalarlo."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx no está instalado."
    echo >&2 "Usa:"
    echo >&2 " cargo install sqlx-cli"
    echo >&2 "para instalarlo"
    exit 1
fi


if [ ! -d "bin" ]; then
    echo "Directorio 'bin' no existe. Creandolo..."
    mkdir -p "bin"
    mkdir -p "bin/Windows"
    mkdir -p "bin/Linux"
    echo "Directorio 'bin' creado."
else
    echo "Directorio 'bin' ya existe."
fi

sqlx database create
sqlx migrate run

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

cp target/x86_64-unknown-linux-gnu/release/entrega bin/Linux/
cp target/x86_64-pc-windows-gnu/release/entrega.exe bin/Windows/
rm -rf target/x86_64-pc-windows-gnu/
rm -rf target/x86_64-unknown-linux-gnu/
