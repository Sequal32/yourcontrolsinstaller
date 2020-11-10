use bytes::Bytes;
use chrono::{DateTime};
use reqwest::{blocking::{Client, ClientBuilder}, header::{HeaderMap, HeaderValue}};
use serde_json::Value;
use serde::Serialize;
use std::{io::{Cursor}};
use zip::ZipArchive;

use crate::util::{Error, Features};

// const LATEST_RELEASE_URL: &str = "https://api.github.com/repositories/290448187/releases/latest";
const LATEST_RELEASE_URL: &str = "http://localhost:8000/release.json";
const FEATURES_URL: &str = "http://localhost:8000/features.json";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseData {
    pub download_url: String,
    pub date: i64,
    pub description: String
}

pub struct Downloader {
    latest_release: Option<ReleaseData>,
    client: Client
}

impl Downloader {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:53.0) Gecko/20100101 Firefox/53.0").unwrap());
        
        let client = ClientBuilder::new().default_headers(headers).build().unwrap();

        Self {
            latest_release: None,
            client
        }
    }

    fn get_json_data(&self) -> Result<Value, reqwest::Error> {
        let response = self.client.get(LATEST_RELEASE_URL).send()?;
        response.json()
    }

    fn parse_json_data(data: Value) -> Option<ReleaseData> {
        let asset_data = data["assets"].as_array()?[0].as_object()?;

        let time = match DateTime::parse_from_rfc3339(asset_data["updated_at"].as_str()?) {
            Ok(t) => t.timestamp(),
            Err(e) => 0
        };

        Some(
            ReleaseData {
                download_url: asset_data["browser_download_url"].as_str()?.to_string(),
                date: time,
                description: data["body"].as_str()?.to_string(),
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

    pub fn download_release(&self) -> Result<ZipArchive<Cursor<Bytes>>, Error> {
        let release_url = match self.latest_release.as_ref() {
            Some(release) => &release.download_url,
            None => return Err(Error::ReleaseError)
        };

        let bytes = match self.client.get(release_url).send() {
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
        let response = match self.client.get(FEATURES_URL).send() {
            Ok(response) => response,
            Err(e) => return Err(Error::WebError(e))
        };

        match response.json() {
            Ok(data) => Ok(data),
            Err(_) => Err(Error::JsonError)
        }
    }
}