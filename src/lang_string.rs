pub enum Languages {
    English,
    Romanian
}

pub const DEFAULT_LANGUAGE: Languages = Languages::English;

pub enum LangKeys {
    GoBack,
    GoForward,
    DeletedFolder,
    EmptyFolder,
}

pub struct LangString {
    lang: Languages
}

impl LangString {
    pub fn new() -> Self {
        Self {
            lang: DEFAULT_LANGUAGE
        }
    }

    pub fn from(lang: Languages) -> Self {
        Self {
            lang
        }
    }

    pub fn get(&self, key: LangKeys) -> String {
        match self.lang {
            Languages::English => {
                match key {
                    LangKeys::GoBack => String::from("Go back"),
                    LangKeys::GoForward => String::from("Go forward"),
                    LangKeys::DeletedFolder => String::from("Folder has been deleted."),
                    LangKeys::EmptyFolder => String::from("Folder has been deleted."),
                }
            }

            Languages::Romanian => {
                match key {
                    LangKeys::GoBack => String::from("Înapoi"),
                    LangKeys::GoForward => String::from("Înainte"),
                    LangKeys::DeletedFolder => String::from("Folderul a fost șters."),
                    LangKeys::EmptyFolder => String::from("Folderul este gol."),
                }
            }
        }
    }
}