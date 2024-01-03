use anyhow::Result;
use log::error;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};
use toml::value::Datetime;
use urlencoding::encode;
#[derive(Debug, Clone)]
pub struct SiteContent {
    pub experience: HashMap<String, ContentSegment>,
    pub education: HashMap<String, ContentSegment>,
    pub projects: HashMap<String, ContentSegment>,
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
    pub fn new(content_dir: &str) -> SiteContent {
        let mut experience_dir = PathBuf::from(content_dir);
        experience_dir.push("experience");
        let experience = get_content_map(experience_dir).unwrap_or_default();

        let mut education_dir = PathBuf::from(content_dir);
        education_dir.push("education");
        let education = get_content_map(education_dir).unwrap_or_default();

        let mut projects_dir = PathBuf::from(content_dir);
        projects_dir.push("projects");
        let projects = get_content_map(projects_dir).unwrap_or_default();

        SiteContent {
            experience,
            education,
            projects,
        }
    }
}

impl ContentSegment {
    pub fn try_new(content_segment_dir: PathBuf) -> Result<ContentSegment> {
        let mut config_path = content_segment_dir.clone();
        config_path.push("config.toml");

        let config_str = fs::read_to_string(config_path)?;
        let config: ContentConfig = toml::from_str(config_str.as_str())?;

        let mut markdown_path = content_segment_dir.clone();
        markdown_path.push("content.md");

        let markdown = fs::read_to_string(markdown_path)?;
        Ok(ContentSegment { config, markdown })
    }
}

fn get_content_map(content_type_path: PathBuf) -> Result<HashMap<String, ContentSegment>> {
    let mut output: HashMap<String, ContentSegment> = HashMap::new();
    for entry in fs::read_dir(content_type_path.clone())? {
        let path = match entry {
            Ok(x) => x.path(),
            Err(err) => {
                error!(
                    "Got an error while trying gather content from {} - {}",
                    content_type_path.display(),
                    err
                );
                if cfg!(test) {
                    Err(err)?;
                }
                continue;
            }
        };
        let segment = match ContentSegment::try_new(path.clone()) {
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
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_existing_content_directory_loads_content_correctly() {
        let _ = env_logger::try_init();

        let content_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/public/content");
        let mut experience_dir = PathBuf::from(content_dir);
        experience_dir.push("experience");
        let experience = get_content_map(experience_dir).unwrap();

        let mut education_dir = PathBuf::from(content_dir);
        education_dir.push("education");
        let education = get_content_map(education_dir).unwrap();

        let mut projects_dir = PathBuf::from(content_dir);
        projects_dir.push("projects");
        let projects = get_content_map(projects_dir).unwrap();

        let site_content = SiteContent {
            experience,
            education,
            projects,
        };
        log::debug!("Site Content: {site_content:#?}")
    }
}
