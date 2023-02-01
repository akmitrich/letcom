use std::{
    env, fs,
    sync::{Arc, RwLock},
};

pub type Settings = Arc<RwLock<SettingsRepr>>;

#[derive(Debug)]
pub struct SettingsRepr {
    pub smtp_relay: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub letter_from: String,
    pub plural_title: String,
    pub single_greet: String,
    pub letter_signature: String,
}

impl SettingsRepr {
    const SMTP_RELAY: &str = "SMTP_RELAY";
    const SMTP_USER: &str = "SMTP_USER";
    const SMTP_PASSWORD: &str = "SMTP_PASSWORD";
    const LETTER_FROM: &str = "LETTER_FROM";
    const PLURAL_TITLE: &str = "PLURAL_TITLE";
    const SINGLE_GREET: &str = "SINGLE_GREET";
    const LETTER_SIGNATURE: &str = "LETTER_SIGNATURE";

    pub fn save(&self) {
        let mut settings = vec![];
        settings.push(format!("{}=\"{}\"", Self::SMTP_RELAY, self.smtp_relay));
        settings.push(format!("{}=\"{}\"", Self::SMTP_USER, self.smtp_user));
        settings.push(format!(
            "{}=\"{}\"",
            Self::SMTP_PASSWORD,
            self.smtp_password
        ));
        settings.push(format!("{}=\"{}\"", Self::LETTER_FROM, self.letter_from));
        settings.push(format!("{}=\"{}\"", Self::PLURAL_TITLE, self.plural_title));
        settings.push(format!("{}=\"{}\"", Self::SINGLE_GREET, self.single_greet));
        settings.push(format!(
            "{}=\"{}\"",
            Self::LETTER_SIGNATURE,
            self.letter_signature
        ));
        fs::write(".env", settings.join("\n")).unwrap();
    }
}

impl Default for SettingsRepr {
    fn default() -> Self {
        Self {
            smtp_relay: Default::default(),
            smtp_user: Default::default(),
            smtp_password: Default::default(),
            letter_from: "john@dow.com".parse().unwrap(),
            plural_title: Default::default(),
            single_greet: Default::default(),
            letter_signature: Default::default(),
        }
    }
}

pub fn load_settings() -> Settings {
    dotenv::dotenv().ok();
    let result = SettingsRepr {
        smtp_relay: env::var(SettingsRepr::SMTP_RELAY).unwrap_or_else(|_| "post.mipt.ru".into()),
        smtp_user: env::var(SettingsRepr::SMTP_USER).unwrap_or_default(),
        smtp_password: env::var(SettingsRepr::SMTP_PASSWORD).unwrap_or_default(),
        letter_from: env::var(SettingsRepr::LETTER_FROM)
            .unwrap_or_else(|_| "kalashnikov.ad@mipt.ru".into()),
        plural_title: env::var(SettingsRepr::PLURAL_TITLE)
            .unwrap_or_else(|_| "Уважаемые коллеги!".into()),
        single_greet: env::var(SettingsRepr::SINGLE_GREET).unwrap_or_else(|_| "".into()),
        letter_signature: env::var(SettingsRepr::LETTER_SIGNATURE)
            .unwrap_or_else(|_| "С уважением,\nАлександр Калашников.".into()),
    };
    Arc::new(RwLock::new(result))
}
