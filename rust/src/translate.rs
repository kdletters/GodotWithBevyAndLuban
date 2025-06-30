static mut LANGUAGE: Language = Language::Chinese;

enum Language {
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

pub fn set_language(language: Language) {
    unsafe {
        LANGUAGE = language;
    }
}

#[macro_export]
macro_rules! lang {
    ($key:expr) => {{ $crate::translate::lang($key) }};
}

pub trait Localize {
    fn local(&self) -> String;
}

impl Localize for &'static str {
    fn local(&self) -> String {
        lang!(self)
    }
}


impl Localize for &String {
    fn local(&self) -> String {
        lang!(self)
    }
}
