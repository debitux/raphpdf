//
// Fill pdf with Iris options.
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
use std::io::{BufRead, BufReader, Result, Seek};
use std::path::Path;
use unidecode::unidecode;

#[cfg(target_os = "linux")]
static RET: &str = "\n";

#[cfg(target_os = "windows")]
static RET: &str = "\r\n";

pub struct Iris {
    pub date_inter: String,
    pub dsc: String,
    pub date_prevu: String,
    pub client: String,
    pub site: String,
    pub code_postal: String,
    pub ville: String,
    pub contact: String,
    pub tel: String,
    pub titre: String,
    pub snnumber: String,
    pub model: String,
    pub typ: String,
    pub num_iris: String,
    pub commentaire: String,
}

impl Iris {
    /// Constructor
    pub fn new() -> Iris {
        Iris {
            date_inter: "".to_string(),
            dsc: "".to_string(),
            date_prevu: "".to_string(),
            client: "".to_string(),
            site: "".to_string(),
            code_postal: "".to_string(),
            ville: "".to_string(),
            contact: "".to_string(),
            tel: "".to_string(),
            titre: "".to_string(),
            snnumber: "".to_string(),
            model: "".to_string(),
            typ: "".to_string(),
            num_iris: "".to_string(),
            commentaire: "".to_string(),
        }
    }
}

pub fn clear(obj: &mut Iris) {
    obj.date_inter = "".to_string();
    obj.dsc = "".to_string();
    obj.date_prevu = "".to_string();
    obj.client = "".to_string();
    obj.site = "".to_string();
    obj.code_postal = "".to_string();
    obj.ville = "".to_string();
    obj.contact = "".to_string();
    obj.tel = "".to_string();
    obj.titre = "".to_string();
    obj.snnumber = "".to_string();
    obj.model = "".to_string();
    obj.typ = "".to_string();
    obj.num_iris = "".to_string();
    obj.commentaire = "".to_string();
}

pub fn read_dsc(obj: &mut String, filep: &Path) -> Result<()> {
    let file = File::open(&filep)?;
    let reader = BufReader::new(&file);
    let mut i = 0;
    let mut j = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line?;
        i += 1;
        if line.contains("Description :") {
            j = _index + 1;
        }
    }
    // println!("i : {}", i);
    let f2 = File::open(&filep)?;
    let r2 = BufReader::new(&f2);
    for (_index, line) in r2.lines().enumerate() {
        let line = line?;
        if _index < j {
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let st = line.to_string() + RET;
        obj.push_str(&st);
        // println!("{}", &line.to_string());
    }
    Ok(())
}

pub fn read_file_iris(obj: &mut Iris, filep: &Path) -> Result<()> {
    let file = File::open(&filep)?;
    let reader = BufReader::new(&file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains("Date prévue: ") {
            obj.date_prevu = line.trim_start_matches("Date prévue: ").to_string();
            obj.date_prevu = unidecode(&obj.date_prevu);
        } else if line.contains("Client : ") {
            obj.client = line.trim_start_matches("Client : ").to_string();
        } else if line.contains("Site d'intervention : ") {
            obj.site = line
                .trim_start_matches("Site d'intervention : ")
                .to_string();
        } else if line.contains("CP d'intervention : ") {
            obj.code_postal = line.trim_start_matches("CP d'intervention : ").to_string();
        } else if line.contains("Ville d'intervention : ") {
            obj.ville = line
                .trim_start_matches("Ville d'intervention : ")
                .to_string();
        } else if line.contains("Contact : ") {
            obj.contact = line.trim_start_matches("Contact : ").to_string();
        } else if line.contains("Téléphone : ") {
            obj.tel = line.trim_start_matches("Téléphone : ").to_string();
        } else if line.contains("Titre : ") {
            obj.titre = line.trim_start_matches("Titre : ").to_string();
            obj.titre = unidecode(&obj.titre);
        } else if line.contains("Modèle du matériel : ") {
            obj.model = line.trim_start_matches("Modèle du matériel : ").to_string();
        } else if line.contains("Type : ") {
            obj.typ = line.trim_start_matches("Type : ").to_string();
        } else if line.contains("N° : ") {
            obj.num_iris = line.trim_start_matches("N° : ").to_string();
        }
        //  println!("{}", line);
    }
    let mut s = String::new();
    read_dsc(&mut s, &filep);
    obj.dsc = unidecode(&s);
    Ok(())
}

pub fn fill_pdf_iris(obj: &mut Iris) -> Result<()> {
    let path = Path::new("GabaritIris.pdf");
    let mut form = Form::load(&path).expect("error don't load the gabarit");

    /*form.set_text(0, "Commentaire".to_string())
        .expect("error message");
    form.set_text(5, "Date d intervention".to_string())
        .expect("error message");*/
    form.set_text(12, (*obj.dsc).to_string())
        .expect("error message");
    form.set_text(13, (*obj.date_prevu).to_string())
        .expect("error message");
    form.set_text(14, (*obj.client).to_string())
        .expect("error message");
    form.set_text(15, (*obj.site).to_string())
        .expect("error message");
    form.set_text(16, (*obj.code_postal).to_string())
        .expect("error message");
    form.set_text(17, (*obj.ville).to_string())
        .expect("error message");
    form.set_text(18, (*obj.contact).to_string())
        .expect("error message");
    form.set_text(19, (*obj.tel).to_string())
        .expect("error message");
    form.set_text(20, (*obj.titre).to_string())
        .expect("error message");
    form.set_text(21, (*obj.snnumber).to_string())
        .expect("error message");
    form.set_text(22, (*obj.model).to_string())
        .expect("error message");
    form.set_text(23, (*obj.typ).to_string())
        .expect("error message");
    form.set_text(24, (*obj.num_iris).to_string())
        .expect("error message");

    let name_file = "pdf/".to_string() + &obj.num_iris + ".pdf";

    let path = Path::new(&name_file);
    form.save(&path)?;
    Ok(())
}
