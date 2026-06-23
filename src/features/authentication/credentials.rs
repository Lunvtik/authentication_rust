use secrecy::SecretString;

pub struct Credentials {
    pub username: String,
    pub password: SecretString,
}
