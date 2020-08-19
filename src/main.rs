//
// RaphPdf
//
// @copyright  Copyright (c) 2020-2020 Grégory Muller
// @license    https://www.apache.org/licenses/LICENSE-2.0
// @link       https://github.com/debitux/raphpdf
// @since      0.1.0
//

extern crate pdf_form_ids;

pub mod iris;
pub mod symag;

use iris::{fill_pdf_iris, read_file_iris, Iris};
use symag::{fill_pdf_sygma, read_file_sygma, Symag};

use pdf_form_ids::Form;
use std::error::Error;
use std::fs;
use std::io::{stderr, Result, Write};
use std::path::{Path, PathBuf};

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "erreur : {}", err);
    while let Some(cause) = err.source() {
        let _ = writeln!(stderr(), "cause : {}", cause);
        err = cause;
    }
}

fn _affiche_champ(rep: &str) {
    let path = Path::new(&rep);
    let form = Form::load(&path).expect("error don't load the gabarit");

    let field_names = form.get_all_names();

    let mut i = 0;
    for ele in field_names {
        println!("{}: {:?}", i, ele);
        i = i + 1;
    }
}

fn recup(symag_liste: &mut Vec<PathBuf>, iris_liste: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir("original/")? {
        let dir = entry?;
        let en = dir.path();
        let ne = &en.as_path().display().to_string();
        if ne.contains("symag") {
            symag_liste.push(en);
        } else if ne.contains("iris") {
            iris_liste.push(en);
        }
    }
    Ok(())
}

fn main() {
    //_affiche_champ("GabaritSymag.pdf");
    //_affiche_champ("GabaritIris.pdf");

    let mut sym = Symag::new();
    let mut iri = Iris::new();

    let mut symag_liste = Vec::new();
    let mut iris_liste = Vec::new();

    if let Err(err) = recup(&mut symag_liste, &mut iris_liste) {
        print_error(&err);
        std::process::exit(1);
    }
    for entry in symag_liste {
        if let Err(err) = read_file_sygma(&mut sym, &entry) {
            print_error(&err);
            std::process::exit(1);
        }

        if let Err(err) = fill_pdf_sygma(&mut sym) {
            print_error(&err);
            std::process::exit(1);
        }
        symag::clear(&mut sym);
    }

    for entry in iris_liste {
        if let Err(err) = read_file_iris(&mut iri, &entry) {
            print_error(&err);
            std::process::exit(1);
        }

        if let Err(err) = fill_pdf_iris(&mut iri) {
            print_error(&err);
            std::process::exit(1);
        }
        iris::clear(&mut iri);
    }
}
