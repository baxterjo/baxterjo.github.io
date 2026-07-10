use crate::errors::WebAppError;
use anyhow::Result;
use include_dir::{include_dir, Dir, DirEntry};
use serde::Deserialize;
use tracing::error;
use std::collections::HashMap;
use toml::value::Datetime;
use urlencoding::encode;

const CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/site_content");

#[derive(Debug, Clone)]
pub struct SiteContent {
    pub experience: HashMap<String, ContentSegment>,
    pub hardware_projects: HashMap<String, ContentSegment>,
    pub software_projects: HashMap<String, ContentSegment>,
}

#[derive(Debug, Clone)]
pub struct ContentSegment {
    pub config: ContentConfig,
    pub markdown: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentConfig {
    pub title: Option<String>,
    pub thumbnail: String,
    pub description: Option<String>,
    pub date_added: Datetime,
    pub priority_level: usize,
}

impl SiteContent {
    pub fn new() -> SiteContent {
        let experience_dir = CONTENT_DIR.get_dir("experience").unwrap();
        let experience = match get_content_map(experience_dir) {
            Ok(x) => x,
            Err(err) => {
                error!("Got error while trying to fetch experience content: {err}");
                HashMap::<String, ContentSegment>::new()
            }
        };

        let hardware_dir = CONTENT_DIR.get_dir("hardware_projects").unwrap();
        let software_dir = CONTENT_DIR.get_dir("software_projects").unwrap();

        let hardware_projects = match get_content_map(hardware_dir) {
            Ok(x) => x,
            Err(err) => {
                error!("Got error while trying to fetch hardware content: {err}");
                HashMap::<String, ContentSegment>::new()
            }
        };
        let software_projects = match get_content_map(software_dir) {
            Ok(x) => x,
            Err(err) => {
                error!("Got error while trying to fetch software content: {err}");
                HashMap::<String, ContentSegment>::new()
            }
        };

        SiteContent {
            experience,
            hardware_projects,
            software_projects,
        }
    }
}

impl Default for SiteContent {
    fn default() -> Self {
        Self::new()
    }
}

const FRONTMATTER_DELIMITER: &str = "+++\n";

impl ContentSegment {
    pub fn try_from_source(source: &str) -> Result<ContentSegment> {
        let after_open = source.strip_prefix(FRONTMATTER_DELIMITER).ok_or_else(|| {
            WebAppError::NotValidConfig(
                "File does not start with a `+++` frontmatter delimiter".to_string(),
            )
        })?;

        let (frontmatter, markdown) = after_open
            .split_once(FRONTMATTER_DELIMITER)
            .ok_or_else(|| {
                WebAppError::NotValidConfig(
                    "No closing `+++` frontmatter delimiter found".to_string(),
                )
            })?;

        let config: ContentConfig = toml::from_str(frontmatter)?;
        let markdown = markdown.trim_start_matches('\n').to_string();

        Ok(ContentSegment { config, markdown })
    }

    fn try_from_file(file: &include_dir::File) -> Result<ContentSegment> {
        let source = file
            .contents_utf8()
            .ok_or(WebAppError::NotValidConfig("Not UTF-8 file".to_string()))?;
        Self::try_from_source(source)
    }
}

fn get_content_map(content_type_path: &Dir) -> Result<HashMap<String, ContentSegment>> {
    let mut output: HashMap<String, ContentSegment> = HashMap::new();
    for entry in content_type_path.entries() {
        match entry {
            DirEntry::File(file) => {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                    continue;
                }

                let segment = match ContentSegment::try_from_file(file) {
                    Ok(x) => x,
                    Err(err) => {
                        error!(
                            "Got an error while trying to parse content at {} - {}",
                            path.display(),
                            err
                        );
                        if cfg!(test) {
                            Err(err)?;
                        }
                        continue;
                    }
                };
                let content_id = match path.file_stem() {
                    Some(x) => x.to_string_lossy().to_string(),
                    None => {
                        error!(
                            "Got error while trying to get content_id from suffix of {}",
                            path.display()
                        );
                        if cfg!(test) {
                            panic!("File name does not work! {}", path.display());
                        }
                        continue;
                    }
                };
                let content_id = encode(&content_id).into_owned();
                output.insert(content_id, segment);
            }
            DirEntry::Dir(_dir) => {}
        }
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_existing_content_directory_loads_content_correctly() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();
        tracing::debug!("Content Dir: {:#?}", CONTENT_DIR);
        let experience_dir = CONTENT_DIR.get_dir("experience").unwrap();
        let experience = get_content_map(experience_dir).unwrap();

        let hardware_dir = CONTENT_DIR.get_dir("hardware_projects").unwrap();
        let software_dir = CONTENT_DIR.get_dir("software_projects").unwrap();

        let hardware_projects = match get_content_map(hardware_dir) {
            Ok(x) => x,
            Err(err) => {
                error!("Got error while trying to fetch hardware content: {err}");
                HashMap::<String, ContentSegment>::new()
            }
        };
        let software_projects = match get_content_map(software_dir) {
            Ok(x) => x,
            Err(err) => {
                error!("Got error while trying to fetch software content: {err}");
                HashMap::<String, ContentSegment>::new()
            }
        };

        let site_content = SiteContent {
            experience,
            hardware_projects,
            software_projects,
        };
        tracing::debug!("Site Content: {site_content:#?}")
    }
}
