//
// Fill pdf with Symag options.
// This file is part of raphpdf
//
// @copyright  Copyright (c) 2020-2020 Grégory Muller
// @license    https://www.apache.org/licenses/LICENSE-2.0
// @link       https://github.com/debitux/raphpdf
// @since      0.1.0
//

extern crate pdf_form_ids;
extern crate unidecode;

use pdf_form_ids::Form;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use unidecode::unidecode;

#[cfg(target_os = "linux")]
static RET: &str = "\n";

#[cfg(target_os = "windows")]
static RET: &str = "\r\n";

pub struct Symag {
    pub numsv: String,
    pub enseigne: String,
    pub groupe: String,
    pub typedi: String,
    pub codeact: String,
    pub dscactivite: String,
    pub date: String,
    pub idsite: String,
    pub site: String,
    pub adresse: String,
    pub ville: String,
    pub code_postal: String,
    pub tel: String,
    pub dscourte: String,
    pub symptome: String,
    pub dcmateriel: String,
    pub sninstalle: String,
    pub trackingaller: String,
    pub contact: String,
}

impl Symag {
    /// Constructor
    pub fn new() -> Symag {
        Symag {
            numsv: "".to_string(),
            enseigne: "".to_string(),
            groupe: "".to_string(),
            typedi: "".to_string(),
            codeact: "".to_string(),
            dscactivite: "".to_string(),
            date: "".to_string(),
            idsite: "".to_string(),
            site: "".to_string(),
            adresse: "".to_string(),
            ville: "".to_string(),
            code_postal: "".to_string(),
            tel: "".to_string(),
            dscourte: "".to_string(),
            symptome: "".to_string(),
            dcmateriel: "".to_string(),
            sninstalle: "".to_string(),
            trackingaller: "".to_string(),
            contact: "None".to_string(),
        }
    }
}

pub fn clear(obj: &mut Symag) {
    obj.numsv = "".to_string();
    obj.enseigne = "".to_string();
    obj.groupe = "".to_string();
    obj.typedi = "".to_string();
    obj.codeact = "".to_string();
    obj.dscactivite = "".to_string();
    obj.date = "".to_string();
    obj.idsite = "".to_string();
    obj.site = "".to_string();
    obj.adresse = "".to_string();
    obj.ville = "".to_string();
    obj.code_postal = "".to_string();
    obj.tel = "".to_string();
    obj.dscourte = "".to_string();
    obj.symptome = "".to_string();
    obj.dcmateriel = "".to_string();
    obj.sninstalle = "".to_string();
    obj.trackingaller = "".to_string();
    obj.contact = "None".to_string();
}

pub fn fill_pdf_sygma(obj: &mut Symag) -> Result<()> {
    let desc = unidecode(&obj.dscourte) + RET + &(unidecode(&obj.symptome));

    let path = Path::new("GabaritSymag.pdf");
    let mut form = Form::load(&path).expect("error don't load the gabarit");

    let contel = unidecode(&obj.contact) + RET + RET + &obj.tel;
    form.set_text(4, desc) // ok
        .expect("error message");
    form.set_text(15, (*obj.dcmateriel).to_string())
        .expect("error message");
    form.set_text(17, (*obj.sninstalle).to_string())
        .expect("error message");
    form.set_text(18, (*obj.trackingaller).to_string())
        .expect("error message");
    form.set_text(19, (*obj.numsv).to_string())
        .expect("error message");
    form.set_text(20, (*obj.idsite).to_string())
        .expect("error message");
    form.set_text(21, (*obj.site).to_string())
        .expect("error message");
    form.set_text(22, (*obj.adresse).to_string())
        .expect("error message");
    form.set_text(23, contel).expect("error message");
    form.set_text(24, (*obj.code_postal).to_string())
        .expect("error message");
    form.set_text(25, (*obj.ville).to_string())
        .expect("error message");
    form.set_text(26, (*obj.date).to_string())
        .expect("error message");
    form.set_text(27, (*obj.enseigne).to_string())
        .expect("error message");
    form.set_text(28, (*obj.typedi).to_string())
        .expect("error message");

    let name_file = "pdf/".to_string() + &obj.numsv + ".pdf";

    let path = Path::new(&name_file);
    form.save(&path)?;
    Ok(())
}

pub fn read_file_sygma(obj: &mut Symag, filep: &Path) -> Result<()> {
    let file = File::open(&filep)?;
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains("Numero de DI :") {
            obj.numsv = line.trim_start_matches("Numero de DI :").to_string();
        } else if line.contains("Enseigne :") {
            obj.enseigne = line.trim_start_matches("Enseigne :").to_string();
        } else if line.contains("Groupe Client :") {
            obj.groupe = (*line).to_string();
        } else if line.contains("Type DI :") {
            obj.typedi = line.trim_start_matches("Type DI :").to_string();
        } else if line.contains("Code Activité :") {
            obj.codeact = (*line).to_string();
        } else if line.contains("Description Activité:") {
            obj.dscactivite = (*line).to_string();
        } else if line.contains("Date et heure de DPA :") {
            obj.date = line
                .trim_start_matches("Date et heure de DPA :")
                .to_string();
        } else if line.contains("ID Site :") {
            obj.idsite = line.trim_start_matches("ID Site :").to_string();
        } else if line.contains("Site :") {
            if line.contains("Site :+") {
                obj.tel = line.trim_start_matches("Téléphone du Site :").to_string();
            } else {
                obj.site = line.trim_start_matches("Site :").to_string();
            }
        } else if line.contains("Adresse du site :") {
            obj.adresse = line.trim_start_matches("Adresse du site :").to_string();
        } else if line.contains("Ville :") {
            obj.ville = line.trim_start_matches("Ville :").to_string();
        } else if line.contains("Code Postal :") {
            obj.code_postal = line.trim_start_matches("Code Postal :").to_string();
        } else if line.contains("Description Courte:") {
            obj.dscourte = (*line).to_string();
        } else if line.contains("Symptôme :") {
            obj.symptome = (*line).to_string();
        } else if line.contains("Description matériel expédié :") {
            obj.dcmateriel = line
                .trim_start_matches("Description matériel expédié :")
                .to_string();
        } else if line.contains("Numéro de série du matériel expédié:") {
            obj.sninstalle = line
                .trim_start_matches("Numéro de série du matériel expédié:")
                .to_string();
        } else if line.contains("Numéro de Tracking :") {
            obj.trackingaller = line.trim_start_matches("Numéro de Tracking :").to_string();
        }
    }
    Ok(())
}
