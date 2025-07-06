static mut LANGUAGE: Language = Language::Chinese;

#[allow(unused)]
pub enum Language {
    Chinese,
}

pub fn lang(key: &str) -> String {
    let text = crate::data::TABLES.Tbtranslate.get(key);
    if let Some(text) = text {
        unsafe {
            match LANGUAGE {
                Language::Chinese => text.cn.clone(),
            }
        }
    } else {
        "".to_string()
    }
}

#[allow(unused)]
pub fn set_language(language: Language) {
    unsafe {
        LANGUAGE = language;
    }
}

pub trait Localize {
    fn local(&self) -> String;
}

impl Localize for &str {
    fn local(&self) -> String {
        lang(self)
    }
}

impl Localize for str {
    fn local(&self) -> String {
        lang(self)
    }
}

impl Localize for &String {
    fn local(&self) -> String {
        lang(self)
    }
}

impl Localize for String {
    fn local(&self) -> String {
        lang(self)
    }
}
