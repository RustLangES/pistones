use serde::Serialize;

use crate::{
    error::Error,
    lang::{ApiResponse, Language, Response},
    EXECUTE_PATH, RUNTIMES_PATH,
};

/// `POST` /api/v2/execute This endpoint requests execution of some arbitrary code.
///
/// language (required) The language to use for execution, must be a string and must be installed.
/// version (required) The version of the language to use for execution, must be a string containing a SemVer selector for the version or the specific version number to use.
/// files (required) An array of files containing code or other data that should be used for execution. The first file in this array is considered the main file.
/// files[].name (optional) The name of the file to upload, must be a string containing no path or left out.
/// files[].content (required) The content of the files to upload, must be a string containing text to write.
/// files[].encoding (optional) The encoding scheme used for the file content. One of base64, hex or utf8. Defaults to utf8.
/// stdin (optional) The text to pass as stdin to the program. Must be a string or left out. Defaults to blank string.
/// args (optional) The arguments to pass to the program. Must be an array or left out. Defaults to [].
/// compile_timeout (optional) The maximum time allowed for the compile stage to finish before bailing out in milliseconds. Must be a number or left out. Defaults to 10000 (10 seconds).
/// run_timeout (optional) The maximum time allowed for the run stage to finish before bailing out in milliseconds. Must be a number or left out. Defaults to 3000 (3 seconds).
/// compile_memory_limit (optional) The maximum amount of memory the compile stage is allowed to use in bytes. Must be a number or left out. Defaults to -1 (no limit)
/// run_memory_limit (optional) The maximum amount of memory the run stage is allowed to use in bytes. Must be a number or left out. Defaults to -1 (no limit)#[derive(Serialize)]
#[derive(Serialize)]
struct Data {
    language: String,
    version: String,
    files: Vec<FileData>,
}

#[derive(Serialize)]
struct FileData {
    name: Option<String>,
    content: String,
}

pub enum ApiVersion {
    V2,
}

pub struct Client {
    exec_url: String,
    runtime_url: String,
    language: String,
    main_file: String,
    add_files: Vec<String>,
    client: reqwest::Client,
}

impl ToString for ApiVersion {
    fn to_string(&self) -> String {
        match self {
            ApiVersion::V2 => format!("api/v2"),
        }
    }
}

impl Client {
    async fn get_lang_version(&self) -> Result<Option<String>, Error> {
        let result = self.client.get(&self.runtime_url).send().await?;
        let json = result.json::<Vec<Language>>().await?;
        Ok(json
            .iter()
            .find(|lang| (lang.language == self.language) | (lang.aliases.contains(&self.language)))
            .and_then(|l| Some(l.version.clone())))
    }

    pub async fn execute(self) -> Result<Response, Error> {
        let Some(version) = self.get_lang_version().await? else {
            return Err(Error::InvalidLanguage);
        };
        let mut files = vec![FileData {
            name: None,
            content: self.main_file,
        }];
        files.extend(
            self.add_files
                .iter()
                .map(|code| FileData {
                    name: None,
                    content: code.to_owned(),
                })
                .collect::<Vec<_>>(),
        );

        let data = self
            .client
            .post(&self.exec_url)
            .json(&Data {
                language: self.language,
                version,
                files,
            })
            .send()
            .await?;

        match data.json::<ApiResponse>().await {
            Ok(ApiResponse::Good(response)) => Ok(response),
            Ok(ApiResponse::Error(error)) => Err(Error::Unknown(error.message().to_owned())),
            Err(err) => Err(Error::Unknown(err.to_string())),
        }
    }
}

pub struct ClientBuilder {
    language: Option<String>,
    main_file: Option<String>,
    add_files: Vec<String>,
    base_url: String,
    api_version: ApiVersion,
    agent: String,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        Self {
            language: None,
            main_file: None,
            add_files: vec![],
            base_url: "https://emkc.org".to_owned(),
            api_version: ApiVersion::V2,
            agent: "Automated Piston Agent".to_owned(),
        }
    }

    pub fn set_base_url<T: ToString>(self, url: T) -> ClientBuilder {
        ClientBuilder {
            base_url: url.to_string(),
            ..self
        }
    }

    pub fn set_version(self, version: ApiVersion) -> ClientBuilder {
        ClientBuilder {
            api_version: version,
            ..self
        }
    }

    pub fn set_lang<T: ToString>(self, lang: T) -> ClientBuilder {
        ClientBuilder {
            language: Some(lang.to_string()),
            ..self
        }
    }

    pub fn set_main_file<T: ToString>(self, code: T) -> ClientBuilder {
        ClientBuilder {
            main_file: Some(code.to_string()),
            ..self
        }
    }

    pub fn add_files<T: ToString>(self, files: Vec<T>) -> ClientBuilder {
        ClientBuilder {
            add_files: files.iter().map(|s| s.to_string()).collect(),
            ..self
        }
    }

    pub fn user_agent<T: ToString>(self, agent: T) -> ClientBuilder {
        ClientBuilder {
            agent: agent.to_string(),
            ..self
        }
    }

    pub fn build(self) -> Result<Client, Error> {
        let language = self.language.ok_or(Error::MissingLang)?;
        let main_file = self.main_file.ok_or(Error::MissingMain)?;

        let http_client = reqwest::ClientBuilder::new()
            .user_agent(&self.agent)
            .build()
            .unwrap();
        Ok(Client {
            language,
            main_file,
            runtime_url: format!(
                "{}/{}{RUNTIMES_PATH}",
                self.base_url,
                self.api_version.to_string(),
            ),
            exec_url: format!(
                "{}/{}{EXECUTE_PATH}",
                self.base_url,
                self.api_version.to_string(),
            ),
            add_files: self.add_files,
            client: http_client,
        })
    }
}
