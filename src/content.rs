use crate::errors::WebAppError;
use anyhow::Result;
use include_dir::{include_dir, Dir, DirEntry};
use log::{debug, error};
use serde::Deserialize;
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

impl ContentSegment {
    pub fn try_new(content_segment_dir: &Dir) -> Result<ContentSegment> {
        debug!("Fetching content segment from: {:#?}", content_segment_dir);
        let mut config_path = content_segment_dir.path().to_path_buf();
        config_path.push("config.toml");
        let config_file =
            content_segment_dir
                .get_file(&config_path)
                .ok_or(WebAppError::FileNotExist(format!(
                    "{}",
                    config_path.display()
                )))?;

        let config: ContentConfig = toml::from_str(
            config_file
                .contents_utf8()
                .ok_or(WebAppError::NotValidConfig("Not UTF-8 file".to_string()))?,
        )?;

        let mut markdown_path = content_segment_dir.path().to_path_buf();
        markdown_path.push("content.md");
        let markdown_file =
            content_segment_dir
                .get_file(&markdown_path)
                .ok_or(WebAppError::FileNotExist(format!(
                    "{}",
                    markdown_path.display()
                )))?;

        let markdown: String = markdown_file
            .contents_utf8()
            .ok_or(WebAppError::NotValidConfig("Not UTF-8 file".to_string()))?
            .to_string();

        Ok(ContentSegment { config, markdown })
    }
}

fn get_content_map(content_type_path: &Dir) -> Result<HashMap<String, ContentSegment>> {
    let mut output: HashMap<String, ContentSegment> = HashMap::new();
    for entry in content_type_path.entries() {
        match entry {
            DirEntry::Dir(dir) => {
                let path = entry.path();
                let segment = match ContentSegment::try_new(dir) {
                    Ok(x) => x,
                    Err(err) => {
                        log::error!(
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
            DirEntry::File(_file) => {}
        }
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_existing_content_directory_loads_content_correctly() {
        let _ = env_logger::try_init();
        log::debug!("Content Dir: {:#?}", CONTENT_DIR);
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
        log::debug!("Site Content: {site_content:#?}")
    }
}
