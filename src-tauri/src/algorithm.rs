/**
 * Password-Generator algorithm
 *
 * Inputs:
 * account_name: str
 * secret: str
 * institution: str
 * industry: str
 *
 *
 * DB-Password-Struct:
 * - Account Name: str
 * - PIN / Secret: str
 * - Institution Name: str
 * - Industry of Institution: enum
 * - Work / Private: bool
 * - LegacyPW: bool
 * - Date added: chrono::DateTime
 */
use super::types::Industry;

pub fn gen_pw(institution: &str, industry: &Industry, secret: &str, account_name: &str) -> String {
    let greek_letter = match institution.len() {
        0..=5 => "Alpha",
        6..=8 => "Beta",
        9..=12 => "Gamma",
        13..=20 => "Delta",
        _ => "Omega",
    };

    let special_char = industry.parse();

    let mut key_number = 0;
    for c in secret.chars() {
        key_number += c as u32;
    }

    // Sum ascii-values of all chars of the account name
    let account_char_sum: u32 = account_name.as_bytes().iter().map(|&b| b as u32).sum();
    let account_god = match account_char_sum % 5 {
        0 => "Apollo",
        1 => "Thor",
        2 => "Anubis",
        3 => "Ra",
        4 => "Zeus",
        _ => unreachable!(),
    };

    format!(
        "{}_{}{}{}",
        greek_letter, special_char, key_number, account_god
    )
}

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
        key_number += c as u32;
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
