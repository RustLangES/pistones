use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use serde::Serialize;

use crate::{
    error::Error,
    lang::{ApiResponse, Language, Response},
    EXECUTE_PATH, RUNTIMES_PATH,
};

#[derive(Clone, Serialize)]
struct Data {
    // The programming language for the code
    language: String,
    // The specific version of the language to use
    version: String,
    // A vector containing files associated with the code
    files: Vec<FileData>,
}

#[derive(Clone, Serialize)]
pub struct FileData {
    // Optional name for the file
    name: Option<String>,
    // The content of the file
    content: String,
}

#[derive(Clone, Copy, Default)]
pub enum ApiVersion {
    // Represents the default API version
    #[default]
    V2,
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiVersion::V2 => write!(f, "api/v2"),
        }
    }
}

fn build_url(base_url: &str, api_version: ApiVersion, path: &str) -> String {
    format!("{base_url}/{api_version}{path}")
}

pub struct Client {
    exec_url: String,
    runtime_url: String,
    enable_cache: bool,
    api_version: ApiVersion,
    cache_lang: Arc<Mutex<Vec<Language>>>,
    client: reqwest::Client,
}

impl Client {
    // Creates a new Client instance with default settings.
    pub async fn new() -> Result<Self, Error> {
        let api_version = ApiVersion::default();
        let runtime_url = build_url("https://emkc.org", api_version, RUNTIMES_PATH);
        let client = reqwest::ClientBuilder::new()
            .user_agent("Automated Piston Agent")
            .build()?;

        let cache_lang = client
            .get(&runtime_url)
            .send()
            .await?
            .json::<Vec<Language>>()
            .await?;

        Ok(Self {
            client,
            runtime_url,
            api_version,
            enable_cache: true,
            cache_lang: Arc::new(Mutex::new(cache_lang)),
            exec_url: build_url("https://emkc.org", api_version, EXECUTE_PATH),
        })
    }

    // Sets the API version to be used in subsequent calls
    pub fn api_version(self, version: ApiVersion) -> Self {
        Self {
            api_version: version,
            ..self
        }
    }

    // Sets the base URL for the Piston API. Rebuilds internal URLs
    pub fn base_url<T: AsRef<str>>(self, url: T) -> Self {
        let runtime_url = build_url(url.as_ref(), self.api_version, RUNTIMES_PATH);
        let exec_url = build_url("https://emkc.org", self.api_version, EXECUTE_PATH);
        Self {
            exec_url,
            runtime_url,
            ..self
        }
    }

    // Disables language information caching
    pub fn disable_cache(self) -> Self {
        Self {
            enable_cache: false,
            ..self
        }
    }

    // Sets a custom user agent for HTTP requests
    pub fn user_agent<T: AsRef<str>>(self, agent: T) -> Result<Self, Error> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(agent.as_ref())
            .build()?;

        Ok(Self { client, ..self })
    }

    // Sets a custom `reqwest::Client` instance for HTTP requests
    pub fn custom_client(self, client: reqwest::Client) -> Result<Self, Error> {
        Ok(Self { client, ..self })
    }

    // Updates the cached language information from the Piston API
    pub async fn refresh_cache(self) -> Result<Self, Error> {
        let result = self.client.get(&self.runtime_url).send().await?;
        let result = result.json::<Vec<Language>>().await?;
        *self.cache_lang.lock().unwrap() = result;
        Ok(self)
    }

    // Retrieves a list of supported languages and their versions.
    // Uses the language cache if enabled, otherwise fetches data from the API.
    pub async fn get_languages(&self) -> Result<Vec<Language>, Error> {
        if self.enable_cache {
            let cache = self.cache_lang.as_ref().lock().unwrap();
            Ok(cache.to_vec())
        } else {
            let result = self.client.get(&self.runtime_url).send().await?;
            Ok(result.json().await?)
        }
    }

    // Gets the version for a specific programming language.
    // Uses the language cache if enabled, otherwise fetches data from the API.
    pub async fn lang_version<T: ToString>(&self, lang: T) -> Result<String, Error> {
        let langs = if self.enable_cache {
            let cache = self.cache_lang.as_ref().lock().unwrap();
            cache.to_vec()
        } else {
            self.client
                .get(&self.runtime_url)
                .send()
                .await?
                .json::<Vec<Language>>()
                .await?
        };

        langs
            .iter()
            .find(|l| (l.language == lang.to_string()) | (l.aliases.contains(&lang.to_string())))
            .map(|l| l.version.clone())
            .ok_or(Error::UnknownLang)
    }

    // Executes code provided as language, version, and a list of files.
    // Sends a POST request to the Piston API with the code data.
    async fn exec<T: ToString, U: ToString>(
        &self,
        lang: T,
        lang_version: U,
        files: Vec<FileData>,
    ) -> Result<Response, Error> {
        let data = self
            .client
            .post(&self.exec_url)
            .json(&Data {
                files,
                language: lang.to_string(),
                version: lang_version.to_string(),
            })
            .send()
            .await?;

        match data.json::<ApiResponse>().await? {
            ApiResponse::Good(response) => Ok(response),
            ApiResponse::Error(error) => Err(Error::Unknown(error.message().to_owned())),
        }
    }

    // Executes code provided as language and an iterator of `FileData` structs.
    // Retrieves the language version and calls `exec` internally.
    pub async fn run_files<T, U, I>(&self, lang: T, content: I) -> Result<Response, Error>
    where
        T: ToString + Clone,
        U: Into<FileData>,
        I: IntoIterator<Item = U>,
    {
        let version = self.lang_version(lang.clone()).await?;

        self.exec(lang, version, content.into_iter().map(Into::into).collect())
            .await
    }

    // Executes code provided as language, version, and content string.
    // Creates a single `FileData` struct from the content and calls `exec` internally.
    pub async fn run_with_version<T: ToString + Clone>(
        &self,
        lang: T,
        version: T,
        content: T,
    ) -> Result<Response, Error> {
        self.exec(
            lang,
            version,
            std::iter::once(FileData {
                name: None,
                content: content.to_string(),
            })
            .collect(),
        )
        .await
    }

    // Executes code provided as language and content string.
    // Retrieves the language version and calls `run_with_version` internally.
    pub async fn run<T: ToString + Clone, U: ToString>(
        &self,
        lang: T,
        content: U,
    ) -> Result<Response, Error> {
        let version = self.lang_version(lang.clone()).await?;

        self.exec(
            lang,
            version,
            std::iter::once(FileData {
                name: None,
                content: content.to_string(),
            })
            .collect(),
        )
        .await
    }
}

impl From<&str> for FileData {
    fn from(content: &str) -> Self {
        Self {
            name: None,
            content: content.to_owned(),
        }
    }
}

impl From<String> for FileData {
    fn from(content: String) -> Self {
        Self {
            name: None,
            content,
        }
    }
}

impl<T> From<(T, T)> for FileData
where
    T: ToString,
{
    fn from((name, content): (T, T)) -> Self {
        Self {
            name: Some(name.to_string()),
            content: content.to_string(),
        }
    }
}
