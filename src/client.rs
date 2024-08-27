use std::{cell::RefCell, fmt::Display};

use serde::Serialize;

use crate::{
    error::Error,
    lang::{ApiResponse, Language, Response},
    EXECUTE_PATH, RUNTIMES_PATH,
};

/// `POST` /api/v2/execute This endpoint requests execution of some arbitrary code.
///
/// language (required) The language to use for execution, must be a string and must be installed.
/// version (required) The version of the language to use for execution, must be a string containing a `SemVer` selector for the version or the specific version number to use.
/// files (required) An array of files containing code or other data that should be used for execution. The first file in this array is considered the main file.
/// files[].name (optional) The name of the file to upload, must be a string containing no path or left out.
/// files[].content (required) The content of the files to upload, must be a string containing text to write.
/// files[].encoding (optional) The encoding scheme used for the file content. One of base64, hex or utf8. Defaults to utf8.
/// stdin (optional) The text to pass as stdin to the program. Must be a string or left out. Defaults to blank string.
/// args (optional) The arguments to pass to the program. Must be an array or left out. Defaults to [].
/// `compile_timeout` (optional) The maximum time allowed for the compile stage to finish before bailing out in milliseconds. Must be a number or left out. Defaults to 10000 (10 seconds).
/// `run_timeout` (optional) The maximum time allowed for the run stage to finish before bailing out in milliseconds. Must be a number or left out. Defaults to 3000 (3 seconds).
/// `compile_memory_limit` (optional) The maximum amount of memory the compile stage is allowed to use in bytes. Must be a number or left out. Defaults to -1 (no limit)
/// `run_memory_limit` (optional) The maximum amount of memory the run stage is allowed to use in bytes. Must be a number or left out. Defaults to -1 (no limit)#[derive(Serialize)]
#[derive(Clone, Serialize)]
struct Data {
    language: String,
    version: String,
    files: Vec<FileData>,
}

#[derive(Clone, Serialize)]
pub struct FileData {
    name: Option<String>,
    content: String,
}

#[derive(Clone, Copy, Default)]
pub enum ApiVersion {
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
    cache_lang: RefCell<Vec<Language>>,
    client: reqwest::Client,
}

impl Client {
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
            cache_lang: RefCell::new(cache_lang),
            exec_url: build_url("https://emkc.org", api_version, EXECUTE_PATH),
        })
    }

    pub fn api_version(self, version: ApiVersion) -> Self {
        Self {
            api_version: version,
            ..self
        }
    }

    pub fn base_url<T: AsRef<str>>(self, url: T) -> Self {
        let runtime_url = build_url(url.as_ref(), self.api_version, RUNTIMES_PATH);
        let exec_url = build_url("https://emkc.org", self.api_version, EXECUTE_PATH);
        Self {
            exec_url,
            runtime_url,
            ..self
        }
    }

    pub fn disable_cache(self) -> Self {
        Self {
            enable_cache: false,
            ..self
        }
    }

    pub fn user_agent<T: AsRef<str>>(self, agent: T) -> Result<Self, Error> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(agent.as_ref())
            .build()?;

        Ok(Self { client, ..self })
    }

    pub fn custom_client(self, client: reqwest::Client) -> Result<Self, Error> {
        Ok(Self { client, ..self })
    }

    pub async fn refresh_cache(self) -> Result<Self, Error> {
        let result = self.client.get(&self.runtime_url).send().await?;
        let result = result.json::<Vec<Language>>().await?;
        self.cache_lang.replace(result);
        Ok(self)
    }

    pub async fn get_languages(&self) -> Result<Vec<Language>, Error> {
        if self.enable_cache {
            Ok(self.cache_lang.take())
        } else {
            let result = self.client.get(&self.runtime_url).send().await?;
            Ok(result.json().await?)
        }
    }

    pub async fn lang_version<T: ToString>(&self, lang: T) -> Result<String, Error> {
        let langs = if self.enable_cache {
            self.cache_lang.take()
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
