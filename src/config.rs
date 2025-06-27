#[derive(Debug, Clone)]
pub struct AirtableConfig {
    pub secret_token: String,
    pub base: String,
    pub table: String,
}

impl AirtableConfig {
    pub fn new(
        secret_token: Option<String>,
        base: Option<String>,
        table: Option<String>,
    ) -> AirtableConfig {
        let secret_token = secret_token.unwrap_or(
            std::env::var("AIRTABLE_SECRET_TOKEN").expect("AIRTABLE_SECRET_TOKEN must be set"),
        );
        let base =
            base.unwrap_or(std::env::var("AIRTABLE_BASE").expect("AIRTABLE_BASE must be set"));
        let table =
            table.unwrap_or(std::env::var("AIRTABLE_TABLE").expect("AIRTABLE_TABLE must be set"));
        AirtableConfig {
            secret_token,
            base,
            table,
        }
    }
}
