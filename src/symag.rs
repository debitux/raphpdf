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

static RET: &str = "\n";
//static RET: &str = "\r\n";

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
        }
    }
}



pub fn fill_pdf_sygma(obj: &mut Symag) -> Result<()> {
    let desc = unidecode(&obj.dscourte) + RET + &(unidecode(&obj.symptome));

    let info = (*obj.enseigne).to_string()
        + RET
        + &obj.groupe
        + RET
        + &obj.idsite
        + RET
        + &obj.site
        + RET
        + &obj.adresse
        + RET
        + &obj.ville
        + RET
        + &obj.code_postal
        + RET
        + &obj.tel
        + RET;

    let dec_info = unidecode(&info);

    let inter =
        (*obj.codeact).to_string() + RET + &obj.typedi + RET + &obj.dscactivite + RET + &obj.date;

    let dec_inter = unidecode(&inter);

    let path = Path::new("GabaritSymag.pdf");
    let mut form = Form::load(&path).expect("error don't load the gabarit");

    form.set_text(1, (*obj.dcmateriel).to_string())
        .expect("error message");
    form.set_text(3, (*obj.sninstalle).to_string())
        .expect("error message");
    form.set_text(4, (*obj.numsv).to_string())
        .expect("error message");
    form.set_text(8, desc).expect("error message");
    form.set_text(15, (*obj.trackingaller).to_string())
        .expect("error message");
    form.set_text(16, dec_info).expect("error message");
    form.set_text(21, dec_inter).expect("error message");

    let name_file = "pdf/".to_string() + &obj.numsv + ".pdf";

    let path = Path::new(&name_file);
    form.save(&path)?;
    Ok(())
}

pub fn read_file_sygma(obj: &mut Symag, filep: &Path) -> Result<()> {
    // let path = Path::new("original/file1.txt");
    let file = File::open(&filep)?;
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains("Numero de DI :") {
            obj.numsv = line.trim_start_matches("Numero de DI :").to_string();
        } else if line.contains("Enseigne :") {
            obj.enseigne = (*line).to_string();
        } else if line.contains("Groupe Client :") {
            obj.groupe = (*line).to_string();
        } else if line.contains("Type DI :") {
            obj.typedi = (*line).to_string();
        } else if line.contains("Code Activité :") {
            obj.codeact = (*line).to_string();
        } else if line.contains("Description Activité:") {
            obj.dscactivite = (*line).to_string();
        } else if line.contains("Date et heure de DPA :") {
            obj.date = line
                .trim_start_matches("Date et heure de DPA :")
                .to_string();
        } else if line.contains("ID Site :") {
            obj.idsite = (*line).to_string();
        } else if line.contains("Site :") {
            obj.site = (*line).to_string();
        } else if line.contains("Adresse du site :") {
            obj.adresse = (*line).to_string();
        } else if line.contains("Ville :") {
            obj.ville = (*line).to_string();
        } else if line.contains("Code Postal :") {
            obj.code_postal = (*line).to_string();
        } else if line.contains("Téléphone du Site :") {
            obj.tel = (*line).to_string();
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
        // println!("{}", line);
    }
    Ok(())
}
