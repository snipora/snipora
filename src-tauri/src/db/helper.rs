pub fn build_fts_query(input: &str) -> String {
    input
        .split_whitespace()
        .map(|token| format!("{}*", token))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn parse_query(input: &str) -> (String, Vec<String>) {
    let mut terms = vec![];
    let mut tags = vec![];

    for token in input.split_whitespace() {
        if let Some(tag) = token.strip_prefix('@') {
            let tag = tag.trim().to_lowercase();
            if !tag.is_empty() {
                tags.push(tag);
            }
        } else {
            terms.push(token);
        }
    }

    (terms.join(" "), tags)
}
