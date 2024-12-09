use color_eyre::{eyre::Context, Result, Section, SectionExt};
use schemars::{gen::SchemaSettings, schema::RootSchema, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug)]
pub struct OllamaClient {
    client: reqwest::Client,
    model: String,
    url: String,
}

impl OllamaClient {
    pub fn new(url: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            model,
            url,
        }
    }

    pub async fn execute<T: DeserializeOwned>(&self, request: OllamaRequest) -> Result<T> {
        #[derive(Debug, Serialize)]
        struct FullRequest {
            model: String,
            messages: Vec<OllamaMessage>,
            stream: bool,
            format: schemars::schema::RootSchema,
        }

        let full_request = FullRequest {
            model: self.model.clone(),
            messages: request.messages,
            stream: request.stream,
            format: request.format,
        };

        let endpoint = format!("{}/api/chat", self.url);
        let response = self
            .client
            .post(&endpoint)
            .json(&full_request)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let deser: T = serde_json::from_str::<OllamaResponse>(&response)
            .map(|res| serde_json::from_str(&res.message.content))
            .wrap_err("Parsing model response")
            .with_section(|| response.clone().header("Model Response"))?
            .wrap_err(format!(
                "Parsing model response as {}",
                std::any::type_name::<T>()
            ))
            .with_section(|| response.header("Model Response"))?;

        Ok(deser)
    }
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
    pub format: schemars::schema::RootSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

impl OllamaMessage {
    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
        }
    }

    #[allow(unused)]
    pub fn assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub message: OllamaMessage,
}

pub fn ollama_schema<T: JsonSchema>() -> RootSchema {
    let mut schema_settings = SchemaSettings::default();
    schema_settings.inline_subschemas = true;
    schema_settings.meta_schema = None;
    let schema_generator = schema_settings.into_generator();

    schema_generator.into_root_schema_for::<T>()
}
