use {
    crate::*,
    lazy_regex::*,
    serde::{
        Deserialize,
        Serialize,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Token {
    Literal(String),
    IqPath(Vec<String>),
}

/// A template that can be rendered with data.
///
/// ```
/// let template = iq::Template::new("test {1}");
/// let data = ('a', 'b');
/// assert_eq!(template.render(data), "test b");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    tokens: Vec<Token>,
}

impl Template {
    pub fn new(template: &str) -> Self {
        let re = regex!(r"\{([^{} ]+)\}");
        let mut tokens = Vec::new();
        let mut last_end = 0;
        for mat in re.find_iter(template) {
            let start = mat.start();
            let end = mat.end();
            if start > last_end {
                tokens.push(Token::Literal(template[last_end..start].to_string()));
            }
            let iq_path = &template[start + 1..end - 1];
            tokens.push(Token::IqPath(iq_path.iq_path()));
            last_end = end;
        }
        if last_end < template.len() {
            tokens.push(Token::Literal(template[last_end..].to_string()));
        }
        Self { tokens }
    }
    pub fn render<T>(
        &self,
        data: T,
    ) -> String
    where
        T: Serialize,
    {
        let mut applied = String::new();
        for token in &self.tokens {
            match token {
                Token::Literal(lit) => applied.push_str(lit),
                Token::IqPath(path) => {
                    if let Some(s) = data.extract_primitive(path) {
                        applied.push_str(&s);
                    }
                }
            }
        }
        applied
    }
}

#[test]
fn test_templates() {
    #[derive(Serialize)]
    struct Span<'s> {
        text: &'s str,
        age: u32,
    }
    #[derive(Serialize)]
    struct Diagnostic {
        disease: Option<String>,
        diag_span: Span<'static>,
    }
    #[derive(Serialize)]
    struct Data<'s> {
        spans: Vec<Span<'s>>,
        diag: Diagnostic,
        stuf: (u16, u16),
    }
    let data = Data {
        spans: vec![
            Span {
                text: "hello",
                age: 1,
            },
            Span {
                text: "world",
                age: 2,
            },
        ],
        diag: Diagnostic {
            disease: Some("covid".to_string()),
            diag_span: Span {
                text: "diagnosis",
                age: 3,
            },
        },
        stuf: (4, 5),
    };
    let template = Template::new(
        "spans: {spans.0.text} {spans.1.age}, diag: {diag.disease} {diag.diag_span.text}, stuf: {stuf.0} {stuf.1}",
    );
    assert_eq!(
        template.render(&data),
        "spans: hello 2, diag: covid diagnosis, stuf: 4 5",
    );
}
