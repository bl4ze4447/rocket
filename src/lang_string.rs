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
    NothingSelected,
    Extension,
    CreatedAt,
    AccessedAt,
    ModifiedAt,
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
                    LangKeys::NothingSelected => String::from("No file is selected."),
                    LangKeys::Extension => String::from("Extension: "),
                    LangKeys::CreatedAt => String::from("Created at: "),
                    LangKeys::AccessedAt => String::from("Accessed at: "),
                    LangKeys::ModifiedAt => String::from("Modified at: "),
                }
            }

            Languages::Romanian => {
                match key {
                    LangKeys::GoBack => String::from("Înapoi"),
                    LangKeys::GoForward => String::from("Înainte"),
                    LangKeys::DeletedFolder => String::from("Folderul a fost șters."),
                    LangKeys::EmptyFolder => String::from("Folderul este gol."),
                    LangKeys::NothingSelected => String::from("Niciun fișier selectat."),
                    LangKeys::Extension => String::from("Extensie: "),
                    LangKeys::CreatedAt => String::from("Creat la: "),
                    LangKeys::AccessedAt => String::from("Accesat la: "),
                    LangKeys::ModifiedAt => String::from("Modificat la: "),
                }
            }
        }
    }
}