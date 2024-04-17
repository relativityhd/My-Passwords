use crate::types::Industry;

pub fn gen_pw(institution: &str, industry: &Industry, secret: &str, account_name: &str) -> String {
    let greek_letter = match institution.len() {
        0..=5 => "Alpha",
        6..=7 => "Beta",
        8..=10 => "Gamma",
        11..=16 => "Delta",
        _ => "Phi",
    };

    let special_char = industry.parse();

    let mut key_number = 0;
    for c in secret.chars() {
        key_number += c as usize;
    }

    // Sum ascii-values of all chars of the account name
    let account_char_sum: usize = account_name.as_bytes().iter().map(|&b| b as usize).sum();
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
