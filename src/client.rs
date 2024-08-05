use serde::Serialize;

use crate::{
    consts::{EXECUTE_URL, RUNTIMES_URL},
    errors::Errors,
    lang::{ApiResponse, Language, Response},
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

pub struct Client {
    language: String,
    main_file: String,
    add_files: Vec<String>,
    client: reqwest::Client,
}

impl Client {
    async fn get_lang_version(&self) -> Result<Option<String>, reqwest::Error> {
        let result = self.client.get(RUNTIMES_URL).send().await?;
        let json = result.json::<Vec<Language>>().await?;
        Ok(json
            .iter()
            .find(|lang| {
                (lang.language == self.language.to_owned())
                    | (lang.aliases.contains(&self.language))
            })
            .and_then(|l| Some(l.version.clone())))
    }

    pub async fn execute(self) -> Result<Response, Errors> {
        let version = match self.get_lang_version().await {
            Err(err) => return Err(Errors::Unknown(err.to_string())),
            Ok(None) => return Err(Errors::InvalidLanguage),
            Ok(Some(v)) => v,
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
            .post(EXECUTE_URL)
            .json(&Data {
                language: self.language,
                version,
                files,
            })
            .send()
            .await;
        let data = match data {
            Err(e) => return Err(Errors::Unknown(e.to_string())),
            Ok(res) => res,
        };

        match data.json::<ApiResponse>().await {
            Ok(ApiResponse::Good(response)) => Ok(response),
            Ok(ApiResponse::Error(error)) => Err(Errors::Unknown(error.message().to_owned())),
            Err(err) => Err(Errors::Unknown(err.to_string())),
        }
    }
}

pub struct ClientBuilder {
    language: Option<String>,
    main_file: Option<String>,
    add_files: Vec<String>,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        Self {
            language: None,
            main_file: None,
            add_files: vec![],
        }
    }

    pub fn set_lang(self, lang: &str) -> ClientBuilder {
        ClientBuilder {
            language: Some(lang.to_owned()),
            ..self
        }
    }

    pub fn set_main_file(self, code: &str) -> ClientBuilder {
        ClientBuilder {
            main_file: Some(code.to_owned()),
            ..self
        }
    }

    pub fn add_files(self, files: Vec<&str>) -> ClientBuilder {
        ClientBuilder {
            add_files: files.iter().map(|s| s.to_string()).collect(),
            ..self
        }
    }

    pub fn build(self) -> Result<Client, Errors> {
        let language = self.language.ok_or(Errors::MissingLang)?;
        let main_file = self.main_file.ok_or(Errors::MissingMain)?;

        let http_client = reqwest::ClientBuilder::new()
            .user_agent("fewwis-bot/@romancitodev")
            .build()
            .unwrap();
        Ok(Client {
            language,
            main_file,
            add_files: self.add_files,
            client: http_client,
        })
    }
}
