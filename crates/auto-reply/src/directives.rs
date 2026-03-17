/// Parse inline directives from message text (e.g. #think, #exec, #reset).
pub struct Directive {
    pub kind: DirectiveKind,
    pub value: Option<String>,
}

pub enum DirectiveKind {
    Think,
    Exec,
    Reset,
}

pub fn parse_directives(text: &str) -> Vec<Directive> {
    let mut directives = Vec::new();

    // Split by whitespace and look for #directive patterns
    for word in text.split_whitespace() {
        if !word.starts_with('#') {
            continue;
        }

        // Parse directive with optional value (e.g., #exec:bash or #think)
        let parts: Vec<&str> = word[1..].splitn(2, ':').collect();
        let directive_name = parts[0].to_lowercase();
        let value = parts.get(1).map(|s| s.to_string());

        let kind = match directive_name.as_str() {
            "think" => DirectiveKind::Think,
            "exec" => DirectiveKind::Exec,
            "reset" => DirectiveKind::Reset,
            _ => continue, // Unknown directive, skip
        };

        directives.push(Directive { kind, value });
    }

    directives
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directives_empty() {
        let directives = parse_directives("");
        assert_eq!(directives.len(), 0);
    }

    #[test]
    fn test_parse_directives_no_directives() {
        let directives = parse_directives("This is a normal message without directives");
        assert_eq!(directives.len(), 0);
    }

    #[test]
    fn test_parse_directives_think() {
        let directives = parse_directives("Please #think about this");
        assert_eq!(directives.len(), 1);
        assert!(matches!(directives[0].kind, DirectiveKind::Think));
        assert_eq!(directives[0].value, None);
    }

    #[test]
    fn test_parse_directives_exec_with_value() {
        let directives = parse_directives("Run #exec:bash script");
        assert_eq!(directives.len(), 1);
        assert!(matches!(directives[0].kind, DirectiveKind::Exec));
        assert_eq!(directives[0].value, Some("bash".to_string()));
    }

    #[test]
    fn test_parse_directives_reset() {
        let directives = parse_directives("#reset the conversation");
        assert_eq!(directives.len(), 1);
        assert!(matches!(directives[0].kind, DirectiveKind::Reset));
        assert_eq!(directives[0].value, None);
    }

    #[test]
    fn test_parse_directives_multiple() {
        let directives =
            parse_directives("#think first, then #exec:python script.py, finally #reset");
        assert_eq!(directives.len(), 3);
        assert!(matches!(directives[0].kind, DirectiveKind::Think));
        assert!(matches!(directives[1].kind, DirectiveKind::Exec));
        assert!(matches!(directives[2].kind, DirectiveKind::Reset));
        assert_eq!(directives[1].value, Some("python".to_string()));
    }

    #[test]
    fn test_parse_directives_case_insensitive() {
        let directives = parse_directives("#THINK #Exec:bash #ReSeT");
        assert_eq!(directives.len(), 3);
        assert!(matches!(directives[0].kind, DirectiveKind::Think));
        assert!(matches!(directives[1].kind, DirectiveKind::Exec));
        assert!(matches!(directives[2].kind, DirectiveKind::Reset));
    }

    #[test]
    fn test_parse_directives_unknown_ignored() {
        let directives = parse_directives("#unknown #think #invalid");
        assert_eq!(directives.len(), 1);
        assert!(matches!(directives[0].kind, DirectiveKind::Think));
    }

    #[test]
    fn test_parse_directives_value_with_colon() {
        let directives = parse_directives("#exec:bash:script.sh");
        assert_eq!(directives.len(), 1);
        assert!(matches!(directives[0].kind, DirectiveKind::Exec));
        assert_eq!(directives[0].value, Some("bash:script.sh".to_string()));
    }
}
