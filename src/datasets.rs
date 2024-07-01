// Autor: Acosta Quintana, Lautaro
use std::{error::Error, path::Path};

use serde::Deserialize;

/// Extrae los datos del dataset y los almacena en un vector String
pub fn cargar_de_csv<P: AsRef<Path>>(archivo: P) -> Result<Vec<String>, Box<dyn Error>> {
    let archivo = std::fs::File::open(archivo)?;
    let buffer = std::io::BufReader::new(archivo);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b';')
        .trim(csv::Trim::All)
        .from_reader(buffer);

    let mut resultado = vec![];
    for r in reader.records() {
        let valor = r?;
        for v in valor.iter() {
            resultado.push(v.to_string());
        }
    }
    Ok(resultado)
}

/// Modela una Localidad con nombre y las calles que le pertenecen.
#[derive(Debug, Deserialize)]
pub struct Localidad {
    pub nombre: String,
    pub calles: Vec<String>,
}

/// Modela una Provincia con su nombre y sus localidades.
#[derive(Debug, Deserialize)]
pub struct Provincia {
    pub nombre: String,
    pub localidades: Vec<Localidad>,
}

/// Extrae los datos del dataset y los almacena en un vector String
pub fn cargar_provincias<P: AsRef<Path>>(archivo: P) -> Result<Vec<Provincia>, Box<dyn Error>> {
    let archivo = std::fs::File::open(archivo)?;
    let buffer = std::io::BufReader::new(archivo);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .delimiter(b',')
        .trim(csv::Trim::All)
        .from_reader(buffer);

    let mut provincias: Vec<Provincia> = Vec::new();

    for result in reader.deserialize() {
        let record: csv::Result<(String, String, String)> = result;
        if let Ok((localidad_nombre, nombre, provincia_nombre)) = record {
            if let Some(provincia) = provincias.iter_mut().find(|p| p.nombre == provincia_nombre) {
                if let Some(localidad) = provincia
                    .localidades
                    .iter_mut()
                    .find(|l| l.nombre == localidad_nombre)
                {
                    localidad.calles.push(nombre);
                } else {
                    let nueva_localidad = Localidad {
                        nombre: localidad_nombre,
                        calles: vec![nombre],
                    };
                    provincia.localidades.push(nueva_localidad);
                }
            } else {
                let nueva_localidad = Localidad {
                    nombre: localidad_nombre,
                    calles: vec![nombre],
                };
                let nueva_provincia = Provincia {
                    nombre: provincia_nombre,
                    localidades: vec![nueva_localidad],
                };
                provincias.push(nueva_provincia);
            }
        }
    }
    Ok(provincias)
}
