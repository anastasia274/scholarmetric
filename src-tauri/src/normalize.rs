/// Normalize a group name: remove extra spaces, ensure dash between letter and number parts,
/// uppercase everything.
/// Examples: "п 601а" -> "П-601А", "П-601 А" -> "П-601А", "п-601а" -> "П-601А"
pub fn normalize_group_name(input: &str) -> String {
    let cleaned: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '–' && *c != '—')
        .collect();

    let upper = cleaned.to_uppercase();

    // Find the boundary between letters and digits
    let mut result = String::new();
    let mut prev_is_alpha = false;

    for ch in upper.chars() {
        let curr_is_digit = ch.is_ascii_digit();
        if prev_is_alpha && curr_is_digit {
            result.push('-');
        }
        result.push(ch);
        prev_is_alpha = ch.is_alphabetic();
    }

    result
}

/// Capitalize the first letter of each word, lowercase the rest.
pub fn normalize_name(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    let rest: String = chars.flat_map(|c| c.to_lowercase()).collect();
                    format!("{}{}", upper, rest)
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_group() {
        assert_eq!(normalize_group_name("п 601а"), "П-601А");
        assert_eq!(normalize_group_name("П-601А"), "П-601А");
        assert_eq!(normalize_group_name("п-601 а"), "П-601А");
        assert_eq!(normalize_group_name("П-601 А"), "П-601А");
        assert_eq!(normalize_group_name("пм 201"), "ПМ-201");
    }

    #[test]
    fn test_normalize_name() {
        assert_eq!(normalize_name("иванов"), "Иванов");
        assert_eq!(normalize_name("ИВАНОВ"), "Иванов");
        assert_eq!(normalize_name("  иванов  "), "Иванов");
    }
}
