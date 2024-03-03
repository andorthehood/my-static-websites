use std::collections::HashMap;

fn parse_front_matter(front_matter: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in front_matter.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            map.insert(key, value);
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_front_matter_success() {
        let front_matter = "title: Example\nauthor: Andor Polgar";
        let parsed = parse_front_matter(front_matter);

        assert_eq!(parsed.get("title"), Some(&"Example".to_string()));
        assert_eq!(parsed.get("author"), Some(&"Andor Polgar".to_string()));
    }
}