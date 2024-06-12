use proc_macro::{token_stream::IntoIter, TokenStream, TokenTree};

pub struct Attributes {
    pub setup: String,
    pub teardown: String,
}

fn parse_string_literal(iter: &mut IntoIter) -> Option<String> {
    if let Some(TokenTree::Punct(punct)) = iter.next() {
        if punct.as_char() == '=' {
            if let Some(TokenTree::Ident(ident)) = iter.next() {
                return Some(ident.to_string());
            }
        }
    }
    None
}

pub fn parse(tokens: TokenStream) -> Result<Attributes, String> {
    let mut setup = None;
    let mut teardown = None;
    let mut iter = tokens.into_iter();
    while let Some(token) = iter.next() {
        if setup.is_some() && teardown.is_some() {
            break;
        }
        if let TokenTree::Ident(ident) = token {
            match ident.to_string().as_str() {
                "setup" => {
                    setup = parse_string_literal(&mut iter);
                }
                "teardown" => {
                    teardown = parse_string_literal(&mut iter);
                }
                _ => {}
            }
        }
    }
    Ok(Attributes {
        setup: setup.ok_or("setup attribute not found")?,
        teardown: teardown.ok_or("teardown attribute not found")?,
    })
}
