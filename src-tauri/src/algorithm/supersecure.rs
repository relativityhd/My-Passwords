use crate::{errors::GenerationError, types::Industry};
use itertools::Itertools;

struct WordLengthRange {
    absolute_min: usize,
    absolute_max: usize,
    current_min: usize,
    current_max: usize,
}

#[derive(Copy, Clone)]
enum CurrentRange {
    Greek,
    Egyptian,
    Special,
    Roman,
}

const GREEK_LETTER: [&str; 24] = [
    "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta", "Iota", "Kappa",
    "Lambda", "My", "Ny", "Xi", "Omikron", "Pi", "Rho", "Sigma", "Tau", "Ypsilon", "Phi", "Chi",
    "Psi", "Omega",
];

const EGYPTIAN_GODS: [&str; 17] = [
    "Ra", "Geb", "Nut", "Shu", "Osiris", "Isis", "Set", "Horus", "Bast", "Sobek", "Serqet",
    "Anubis", "Bes", "Khonsu", "Nekhbet", "Babi", "Tawaret",
];

/*
- "Die hängenden Gärten der Semiramis zu Babylon",
- "Der Koloss von Rhodos",
- "Das Grab des Königs Mausolos II. zu Halikarnassos",
- "Der Leuchtturm auf der Insel Pharos vor Alexandria",
- "Die Pyramiden von Gizeh in Ägypten",
- "Der Tempel der Artemis in Ephesos",
- "Die Zeusstatue des Phidias von Olympia"
*/
const WORLD_WONDERS: [&str; 7] = ["HGB", "KVR", "GKM", "LVA", "PGA", "TAE", "ZSO"];

trait CalculatableString {
    fn sum(&self) -> usize;
}

impl CalculatableString for str {
    fn sum(&self) -> usize {
        self.chars().map(|c| c as usize).sum()
    }
}

fn romanize(num: usize) -> String {
    let mut num = num;
    let mut result = String::new();
    let roman_numerals = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    for &(value, numeral) in roman_numerals.iter() {
        while num >= value {
            result += numeral;
            num -= value;
        }
    }

    result
}

fn calc_extreme_lengths(ranges: [&WordLengthRange; 4], is_ww: bool) -> (usize, usize) {
    let mut min = 3;
    let mut max = 3;
    if is_ww {
        min += 3;
        max += 3;
    }
    for range in ranges.iter() {
        min += range.current_min;
        max += range.current_max;
    }
    (min, max)
}

fn calc_idx_from_values(values: [usize; 4], len: usize) -> usize {
    let vlen = 4.;
    let vsum = values.into_iter().sum::<usize>() as f64;
    let multiplier = (vsum % (100. * vlen) / 100.) / vlen;

    // This idx calculation is shit, however I want to stick to the original to dont mess with my passwords...
    let idx = ((len - 1) as f64 * multiplier).round() as usize;
    // let idx = (multiplier * len as f64).floor() as usize;

    idx
}

fn get_greek_letter(
    institution: &str,
    secret: &str,
    account_name: &str,
    pin: usize,
    range: &WordLengthRange,
) -> String {
    let pool = GREEK_LETTER
        .iter()
        .filter(|&x| x.len() >= range.current_min && x.len() <= range.current_max)
        .collect::<Vec<&&str>>();
    let vals = [
        institution.sum() * account_name.sum(),
        institution.len() * secret.sum(),
        secret.sum() * pin,
        account_name.len() * secret.len(),
    ];
    let idx = calc_idx_from_values(vals, pool.len());
    pool[idx].to_string()
}

fn get_egyptian_god(
    institution: &str,
    secret: &str,
    account_name: &str,
    pin: usize,
    range: &WordLengthRange,
) -> String {
    let pool = EGYPTIAN_GODS
        .iter()
        .filter(|&x| x.len() >= range.current_min && x.len() <= range.current_max)
        .collect::<Vec<&&str>>();
    let vals = [
        institution.sum() * secret.sum(),
        institution.len() * pin,
        account_name.sum() * pin,
        account_name.len() * secret.len(),
    ];
    let idx = calc_idx_from_values(vals, pool.len());
    pool[idx].to_string()
}

fn get_special_chars(pool: &str, industry: &Industry, range: &WordLengthRange) -> String {
    if pool.len() == 0 {
        return "".to_string();
    }

    let industry_num = match industry {
        Industry::Games => 1,
        Industry::Tech => 2,
        Industry::Social => 3,
        Industry::Finance => 4,
        Industry::Other => 5,
        _ => 5,
    };

    match range.current_max {
        2 => {
            let first_idx = (industry_num + 1) % pool.len();
            let second_idx = (2 * industry_num + 1) % pool.len();
            let first = pool.chars().nth(first_idx).unwrap();
            let second = pool.chars().nth(second_idx).unwrap();
            format!("{}{}", first, second)
        }
        1 => {
            let first_idx = (industry_num + 1) % pool.len();
            let first = pool.chars().nth(first_idx).unwrap();
            format!("{}", first)
        }
        _ => "".to_string(),
    }
}

fn get_roman_number(pin: usize, range: &WordLengthRange) -> String {
    match range.current_max {
        5 => romanize(pin % 28 + 1),
        4 => romanize(pin % 18 + 1),
        3 => romanize(pin % 8 + 1),
        2 => romanize(pin % 2 + 1),
        1 => romanize(1),
        _ => "".to_string(),
    }
}

fn get_world_wonder(
    institution: &str,
    industry: &Industry,
    account_name: &str,
    pin: usize,
    is_ww: bool,
) -> String {
    if !is_ww {
        return "".to_string();
    }
    let industry_num = match industry {
        Industry::Games => 1,
        Industry::Tech => 2,
        Industry::Social => 3,
        Industry::Finance => 4,
        Industry::Other => 5,
        _ => 5,
    };
    let idx = (pin + industry_num + institution.len() + account_name.sum()) % WORLD_WONDERS.len();
    WORLD_WONDERS[idx].to_string()
}

fn try_generation(
    institution: &str,
    industry: &Industry,
    secret: &str,
    account_name: &str,
    pin: usize,
    special_chars: &str,
    seed: usize,
    is_ww: bool,
    greek_range: &WordLengthRange,
    egyptian_range: &WordLengthRange,
    special_range: &WordLengthRange,
    roman_range: &WordLengthRange,
) -> String {
    let greek = get_greek_letter(institution, secret, account_name, pin, greek_range);
    let egyptian = get_egyptian_god(institution, secret, account_name, pin, egyptian_range);
    let special = get_special_chars(special_chars, industry, special_range);
    let roman = get_roman_number(pin, roman_range);
    let ww = get_world_wonder(institution, industry, account_name, pin, is_ww);
    let numbers = (pin * secret.len()) % 999;

    // If seed >= 120 expect it to be a year and handle it like in the old algorithm
    if seed >= 120 {
        return match seed % 3 {
            0 => format!("{}{}{}{}{}{}", egyptian, ww, numbers, greek, special, roman),
            1 => format!("{}{}{}{}{}{}", greek, special, ww, numbers, egyptian, roman),
            2 => format!("{}{}{}{}{}{}", egyptian, ww, numbers, greek, special, roman),
            _ => format!("{}{}{}{}{}{}", greek, special, ww, numbers, egyptian, roman),
        };
    }

    // New version: seed decides the order of the parts
    // First part is either greek or egyptian
    // Rest is pseudo-random
    let (first, parts) = match seed % 2 {
        0 => (greek, [egyptian, special, roman, ww, numbers.to_string()]),
        1 => (egyptian, [greek, special, roman, ww, numbers.to_string()]),
        _ => unreachable!(),
    };
    let mut position_permutations = (0..parts.len()).permutations(5);
    // 120 = 5! / (5-5)! (all permutations of 5 elements)
    let pidx = seed % 120;
    // Unwrap is safe because we know that we use mod over the length of the iterator
    let positions = position_permutations.nth(pidx).unwrap();
    let parts = positions
        .iter()
        .map(|&i| parts[i].as_str())
        .collect::<Vec<&str>>();

    format!(
        "{}{}{}{}{}{}",
        first, parts[0], parts[1], parts[2], parts[3], parts[4]
    )
}

pub fn gen_super_pw(
    institution: &str,
    industry: &Industry,
    secret: &str,
    account_name: &str,
    pin: usize,
    min_length: usize,
    max_length: usize,
    special_chars: &str,
    seed: usize,
) -> Result<String, GenerationError> {
    let mut greek_range = WordLengthRange {
        absolute_min: 2,
        absolute_max: 7,
        current_min: 2,
        current_max: 7,
    };
    let mut egyptian_range = WordLengthRange {
        absolute_min: 3,
        absolute_max: 7,
        current_min: 2,
        current_max: 7,
    };
    let mut special_range = WordLengthRange {
        absolute_min: 0,
        absolute_max: 2,
        current_min: 0,
        current_max: 2,
    };
    let mut roman_range = WordLengthRange {
        absolute_min: 0,
        absolute_max: 5,
        current_min: 0,
        current_max: 5,
    };

    let is_ww = max_length >= 16;

    for currentrange in [
        CurrentRange::Greek,
        CurrentRange::Egyptian,
        CurrentRange::Special,
        CurrentRange::Roman,
    ]
    .repeat(20)
    {
        let password = try_generation(
            institution,
            industry,
            secret,
            account_name,
            pin,
            special_chars,
            seed,
            is_ww,
            &greek_range,
            &egyptian_range,
            &special_range,
            &roman_range,
        );

        if password.len() >= min_length && password.len() <= max_length {
            return Ok(password);
        }

        let (min_possible_length, max_possible_length) = calc_extreme_lengths(
            [&greek_range, &egyptian_range, &special_range, &roman_range],
            is_ww,
        );

        let change_range = match currentrange {
            CurrentRange::Greek => &mut greek_range,
            CurrentRange::Egyptian => &mut egyptian_range,
            CurrentRange::Special => &mut special_range,
            CurrentRange::Roman => &mut roman_range,
        };
        let is_min_less_than_max = change_range.current_min < change_range.current_max;
        if !is_min_less_than_max {
            continue;
        }

        let is_range_too_long = password.len() > max_length || min_possible_length > max_length;
        let is_range_too_short = password.len() < min_length || max_possible_length < min_length;

        if is_range_too_long && change_range.current_min < change_range.absolute_max {
            change_range.current_max -= 1;
        } else if is_range_too_short && change_range.current_max > change_range.absolute_min {
            change_range.current_min += 1;
        } else {
            continue;
        }
    }

    // Fallback password
    Err(GenerationError)
}
