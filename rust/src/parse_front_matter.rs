use std::collections::HashMap;

fn trim_quotes(s: &str) -> String {
    if (s.starts_with('\'') && s.ends_with('\'') || s.starts_with('"') && s.ends_with('"')) && s.len() > 1 {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn parse_front_matter(front_matter: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in front_matter.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = trim_quotes(parts[1].trim());
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
        let front_matter = "title: Example\nauthor: Andor Polgar\nlocation: 'Zandvoort, Netherlands'";
        let parsed = parse_front_matter(front_matter);

        assert_eq!(parsed.get("title"), Some(&"Example".to_string()));
        assert_eq!(parsed.get("author"), Some(&"Andor Polgar".to_string()));
        assert_eq!(parsed.get("camera"), None);
        assert_eq!(parsed.get("location"), Some(&"Zandvoort, Netherlands".to_string()));
    }
}