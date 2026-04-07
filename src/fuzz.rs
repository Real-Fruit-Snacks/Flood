/// Represents where a FUZZ keyword was found.
#[derive(Debug, Clone, PartialEq)]
pub enum FuzzPosition {
    /// FUZZ keyword found in the URL, with the keyword string (FUZZ, FUZ2Z, etc.)
    Url(String),
    /// FUZZ keyword found in a header. (header_index, keyword)
    Header(usize, String),
    /// FUZZ keyword found in POST data, with the keyword string.
    Data(String),
}

/// Generate the list of FUZZ keywords for N wordlists.
pub fn fuzz_keywords(count: usize) -> Vec<&'static str> {
    const KEYWORDS: &[&str] = &[
        "FUZZ", "FUZ2Z", "FUZ3Z", "FUZ4Z", "FUZ5Z", "FUZ6Z", "FUZ7Z", "FUZ8Z", "FUZ9Z",
    ];
    KEYWORDS[..count.min(KEYWORDS.len())].to_vec()
}

/// Detect all FUZZ keyword positions across URL, headers, and POST data.
pub fn detect_fuzz_positions(
    url: &str,
    headers: &[String],
    data: &Option<String>,
) -> Vec<FuzzPosition> {
    let mut positions = Vec::new();
    let all_keywords = fuzz_keywords(9);

    for keyword in &all_keywords {
        if url.contains(keyword) {
            positions.push(FuzzPosition::Url(keyword.to_string()));
        }
    }

    for (i, header) in headers.iter().enumerate() {
        for keyword in &all_keywords {
            if header.contains(keyword) {
                positions.push(FuzzPosition::Header(i, keyword.to_string()));
            }
        }
    }

    if let Some(ref d) = data {
        for keyword in &all_keywords {
            if d.contains(keyword) {
                positions.push(FuzzPosition::Data(keyword.to_string()));
            }
        }
    }

    positions
}

/// Replace all occurrences of `keyword` in `template` with `value`.
pub fn substitute(template: &str, value: &str, keyword: &str) -> String {
    template.replace(keyword, value)
}

/// Build the full URL by substituting all keyword→value pairs.
pub fn build_url(template: &str, substitutions: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (keyword, value) in substitutions {
        result = substitute(&result, value, keyword);
    }
    result
}

/// Build headers by substituting all keyword→value pairs.
pub fn build_headers(templates: &[String], substitutions: &[(&str, &str)]) -> Vec<String> {
    templates
        .iter()
        .map(|h| {
            let mut result = h.clone();
            for (keyword, value) in substitutions {
                result = substitute(&result, value, keyword);
            }
            result
        })
        .collect()
}

/// Build POST data by substituting all keyword→value pairs.
pub fn build_data(template: &Option<String>, substitutions: &[(&str, &str)]) -> Option<String> {
    template.as_ref().map(|d| {
        let mut result = d.clone();
        for (keyword, value) in substitutions {
            result = substitute(&result, value, keyword);
        }
        result
    })
}
