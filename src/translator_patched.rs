use reqwest;
use tl;

/// Translator struct.
///
/// # Arguments
///
/// * `to` - The static string slice text will be translated to.
/// * `from` - The static string slice text will be translated from.
///
/// # Examples
///
/// ```
/// fn main() {
///     let translator_struct = rustlate::Translator{
///         to: "en",
///         from: "tr"
///     };
/// }
/// ```
#[derive(Clone)]
pub struct Translator{
    pub to: &'static str,
    pub from: &'static str
}

impl Translator {
    /// Translates the text.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice text will be translated.
    ///
    /// # Examples
    ///
    /// ```
    /// fn main() {
    ///     let translator_struct = rustlate::Translator{
    ///         to: "tr",
    ///         from: "en"
    ///     };
    ///
    ///     println!("{:?}", translator_struct.translate("cat"));
    /// }
    /// ```
    pub fn translate(&self, text: &str) -> Result<String, String> {
        parse_result(fetch_page(text, self.from, self.to))
    }
}

fn fetch_page(text: &str, from: &str, to: &str) -> Result<String, String> {
    let formatted_url = format!("https://translate.google.com/m?tl={}&sl={}&q={}", to, from, text);

    match reqwest::blocking::get(formatted_url) {
        Ok(response) => match response.text() {
            Ok(body) => return Ok(body),
            Err(err) => return Err(err.to_string())
        },
        Err(err) => return Err(err.to_string())
    }
}

fn parse_result(result: Result<String, String>) -> Result<String, String> {
    match result {
        Ok(body) => match tl::parse(&body.to_owned()).get_elements_by_class_name("result-container") {
                Some(element) => return Ok(element[0].inner_text().into_owned()),
                None => return Err(String::from("unexcepted error."))
        },
        Err(err) => return Err(err)
    }
}
