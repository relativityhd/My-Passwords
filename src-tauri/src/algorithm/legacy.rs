use crate::types::Industry;

pub fn gen_legacy_pw(institution: &str, industry: &Industry, secret: &str) -> String {
    let greek_letter = match institution.len() {
        0..=5 => "Alpha",
        6 | 7 => "Beta",
        8 | 9 => "Gamma",
        10..=14 => "Delta",
        _ => "Omega",
    };

    let special_char = match industry {
        Industry::Tech => "@",
        Industry::Games => "!",
        Industry::Social => "%",
        Industry::Finance => "$",
        Industry::Other => "&",
        _ => "&",
    };

    let mut key_number = 0;
    for c in secret.chars() {
        key_number += c as usize;
    }

    let key_letter = match secret.len() {
        0..=3 => "Epsilon",
        4 | 5 => "Zeta",
        6..=8 => "Eta",
        9..=11 => "Theta",
        _ => "Psi",
    };

    format!(
        "{}_{}{}{}",
        greek_letter, special_char, key_number, key_letter
    )
}
