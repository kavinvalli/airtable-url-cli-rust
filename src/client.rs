use crate::config::AirtableConfig;
use crate::error::ClientError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordField {
    pub slug: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub fields: RecordField,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirtableResponse {
    pub records: Vec<Record>,
}

// this looks the same as the response but i wanted to be explicit
#[derive(Serialize, Debug)]
pub struct AirtableRequest {
    pub records: Vec<Record>,
}

#[derive(Debug, Clone)]
pub struct AirtableClient {
    config: AirtableConfig,
}

impl AirtableClient {
    pub fn new(config: AirtableConfig) -> AirtableClient {
        AirtableClient { config }
    }

    pub async fn get_all_records(&self) -> Result<Vec<Record>, ClientError> {
        let url = format!(
            "https://api.airtable.com/v0/{}/{}/?maxRecords=100",
            self.config.base, self.config.table
        );
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.secret_token),
            )
            .send()
            .await?
            .json::<AirtableResponse>()
            .await?;

        Ok(response.records)
    }

    pub async fn get_record_by_slug(&self, _slug: &str) -> Result<Record, ClientError> {
        let url = format!(
            "https://api.airtable.com/v0/{}/{}/?maxRecords=100&filterByFormula=SEARCH(\"{}\", slug) ",
            self.config.base, self.config.table, _slug
        );

        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.secret_token),
            )
            .send()
            .await?
            .json::<AirtableResponse>()
            .await?;

        response.records.into_iter().next().ok_or_else(|| {
            ClientError::ParseError(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No record found with this slug",
            )))
        })
    }

    pub async fn create_record(&self, slug: &str, target: &str) -> Result<Record, ClientError> {
        let url = format!(
            "https://api.airtable.com/v0/{}/{}/",
            self.config.base, self.config.table
        );

        let client = reqwest::Client::new();

        let mut request = AirtableRequest {
            records: Vec::new(),
        };

        request.records.push(Record {
            fields: RecordField {
                slug: slug.to_string(),
                target: target.to_string(),
            },
        });

        let response = client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.secret_token),
            )
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        println!("Response: {:?}", response);

        let response_text = response.text().await?;
        println!("Response text: {}", response_text);

        let airtable_response: AirtableResponse = serde_json::from_str(&response_text)?;

        airtable_response.records.into_iter().next().ok_or_else(|| {
            ClientError::ParseError(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No record found with this slug",
            )))
        })
    }

    // TODO: Implement
    // pub async fn update_record(&self, _slug: &str, _target: &str) -> Result<(), reqwest::Error> {
    //     Ok(())
    // }

    // TODO: Implement
    // pub async fn delete_record(&self, _slug: &str) -> Result<(), reqwest::Error> {
    //     Ok(())
    // }
}
