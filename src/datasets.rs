use std::{error::Error, path::Path};

use serde::Deserialize;

pub fn cargar_universidades<P: AsRef<Path>>(archivo: P) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b',')
        .from_path(archivo)?;

    let mut resultado = vec![];
    for r in reader.records() {
        let valor = r?;
        for v in valor.iter() {
            resultado.push(v.to_string());
        }
    }
    Ok(resultado)
}

#[derive(Debug, Deserialize)]
pub struct Localidad {
    pub nombre: String,
    pub calles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Provincia {
    pub nombre: String,
    pub localidades: Vec<Localidad>,
}

pub fn cargar_provincias<P: AsRef<Path>>(archivo: P) -> Result<Vec<Provincia>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .delimiter(b',')
        .from_path(archivo)?;

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
