use chrono::{DateTime};
use serde_json::Value;
use serde::Serialize;
use std::{io::{Cursor}};
use zip::ZipArchive;

use crate::util::{Error, Features};

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/sequal32/yourcontrols/releases/latest";
const RELEASE_BASE_URL: &str = "https://github.com/Sequal32/yourcontrols/releases/download/vbase/YourControls.zip";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:53.0) Gecko/20100101 Firefox/53.0";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseData {
    pub date: i64,
    pub tag_name: String,
    pub release_url: String
}

pub struct Downloader {
    latest_release: Option<ReleaseData>
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            latest_release: None,
        }
    }

    fn get_url(&self, url: &str) -> Result<attohttpc::Response, attohttpc::Error> {
        attohttpc::get(url).header("User-Agent", USER_AGENT).send()
    }

    fn get_json_data(&self) -> Result<Value, attohttpc::Error> {
        let response = self.get_url(LATEST_RELEASE_URL)?;
        response.error_for_status()?.json()
    }

    fn parse_json_data(data: Value) -> Option<ReleaseData> {
        let asset_data = data["assets"].as_array()?[0].as_object()?;

        let time = match DateTime::parse_from_rfc3339(asset_data["updated_at"].as_str()?) {
            Ok(t) => t.timestamp(),
            Err(_) => 0
        };

        Some(
            ReleaseData {
                date: time,
                tag_name: data["tag_name"].as_str()?.to_string(),
                release_url: data["html_url"].as_str()?.to_string()
            }
        )
    }

    fn fetch_data(&mut self) -> Result<(), Error> {
        let json = match self.get_json_data() {
            Ok(data) => data,
            Err(e) => return Err(Error::WebError(e))
        };
        
        self.latest_release = match Self::parse_json_data(json) {
            Some(data) => Some(data),
            None => return Err(Error::JsonError)
        };

        Ok(())
    }

    pub fn get_data(&mut self) -> Result<Option<&ReleaseData>, Error> {
        if self.latest_release.is_none() {self.fetch_data()?;}

        Ok(self.latest_release.as_ref())
    }

    pub fn download_release(&self) -> Result<ZipArchive<Cursor<Vec<u8>>>, Error> {
        let bytes = match self.get_url(RELEASE_BASE_URL) {
            Ok(response) => response.bytes().unwrap(),
            Err(e) => return Err(Error::WebError(e))
        };

        let cursor = Cursor::new(bytes);
        let zip = match ZipArchive::new(cursor) {
            Ok(zip) => zip,
            Err(e) => return Err(Error::ZipError(e))
        };

        Ok(zip)
    }

    pub fn get_features(&self) -> Result<Features, Error> {
        let tag_name = match self.latest_release.as_ref() {
            Some(d) => &d.tag_name,
            None => return Err(Error::ReleaseError)
        };

        let response = match self.get_url(&format!("https://raw.githubusercontent.com/Sequal32/yourcontrols/{}/definitions/features.json", tag_name)) {
            Ok(response) => response,
            Err(e) => return Err(Error::WebError(e))
        };

        match response.json() {
            Ok(data) => Ok(data),
            Err(_) => Err(Error::JsonError)
        }
    }
}