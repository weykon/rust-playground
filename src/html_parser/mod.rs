use std::str::FromStr;

struct HtmlParser {
    html: String,
    tree: Ele,
}

struct Ele {
    tag: String,
    children: Vec<Ele>,
}

impl HtmlParser {
    fn parse(html: String) {
        let mut tree = Ele {
            tag: String::from("html"),
            children: Vec::new(),
        };
        let mut current = &mut tree;

        use regex::Regex;

        let re = Regex::new(r"(?i)<(/?)([a-z0-9]+)(?:\s*([^>]*))?>").unwrap();

        
    }
}

mod tests {
    use crate::html_parser::HtmlParser;

    const HTML: &str = r#"<html>
        <head>
            <title>Test</title>
        </head>
        <body>
            <h1>Test</h1>
            <p>Test</p>
        </body>
        </html>
        "#;

    #[test]
    fn test_html_parser() {}
}
