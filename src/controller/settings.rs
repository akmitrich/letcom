use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub smtp_relay: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub letter_from: String,
    pub plural_title: String,
    pub single_greet: String,
}

impl Settings {
    const SMTP_RELAY: &str = "SMTP_RELAY";
    const SMTP_USER: &str = "SMTP_USER";
    const SMTP_PASSWORD: &str = "SMTP_PASSWORD";
    const LETTER_FROM: &str = "LETTER_FROM";
    const PLURAL_TITLE: &str = "PLURAL_TITLE";
    const SINGLE_GREET: &str = "SINGLE_GREET";

    pub fn load() -> Self {
        dotenv::dotenv().ok();
        Self {
            smtp_relay: env::var(Self::SMTP_RELAY).unwrap_or_else(|_| "post.mipt.ru".into()),
            smtp_user: env::var(Self::SMTP_USER).unwrap_or_default(),
            smtp_password: env::var(Self::SMTP_PASSWORD).unwrap_or_default(),
            letter_from: env::var(Self::LETTER_FROM)
                .unwrap_or_else(|_| "kalashnikov.ad@mipt.ru".into()),
            plural_title: env::var(Self::PLURAL_TITLE)
                .unwrap_or_else(|_| "Уважаемые коллеги!".into()),
            single_greet: env::var(Self::SINGLE_GREET)
                .unwrap_or_else(|_| "С уважением,\nАлександр Калашников.".into()),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            smtp_relay: Default::default(),
            smtp_user: Default::default(),
            smtp_password: Default::default(),
            letter_from: "john@dow.com".parse().unwrap(),
            plural_title: Default::default(),
            single_greet: Default::default(),
        }
    }
}
