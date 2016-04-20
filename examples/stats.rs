extern crate opencorpora;

use opencorpora::Dict;

fn main() {
    use std::fs::File;
    let file = File::open("dict/dict.xml").unwrap();
    match Dict::read_from_xml(&file) {
        Ok(dict) => {
            println!("Version: {}", dict.version);
            println!("Revision: {}", dict.revision);
            println!("Grammemes count: {}", dict.grammemes.len());
            println!("Restrictions count: {}", dict.restrictions.len());
            println!("Lemmata count: {}", dict.lemmata.len());
            let mut forms_count = 0;
            let mut max_forms_in_lemma = 0;
            let mut max_grammemes_in_form = 0;
            for lemma in &dict.lemmata {
                let forms = lemma.forms.len();
                forms_count += forms;
                if max_forms_in_lemma < forms {
                    max_forms_in_lemma = forms;
                }

                let lemma_grammemes = lemma.grammemes.len();
                for form in &lemma.forms {
                    let form_grammemes = form.grammemes.len();
                    let total_grammemes = lemma_grammemes + form_grammemes;
                    if max_grammemes_in_form < total_grammemes {
                        max_grammemes_in_form = total_grammemes;
                    }
                }
            }
            println!("All forms count: {}", forms_count);
            println!("Max forms in a lemma: {}", max_forms_in_lemma);
            println!("Max grammemes in a form: {}", max_grammemes_in_form);
            println!("Link types count: {}", dict.link_kinds.len());
            println!("Links count: {}", dict.links.len());
        },
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
}
