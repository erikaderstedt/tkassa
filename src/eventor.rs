use reqwest::{blocking::Client, IntoUrl};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::io::ErrorKind;
use std::{fs::read_to_string, hash::Hash, hash::Hasher, path::Path};

pub struct EventorClient<'a> {
    api_key: &'a str,
    verbose: bool,
    cache_folder: &'a Path,
    client: reqwest::blocking::Client,
}

impl<'a> EventorClient<'a> {
    pub fn new(api_key: &'a str, cache_folder: &'a str, verbose: bool) -> EventorClient {
        EventorClient {
            api_key,
            verbose,
            cache_folder: Path::new(cache_folder),
            client: Client::new(),
        }
    }

    pub fn request<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        parameters: &T,
    ) -> xmltree::Element {
        if self.verbose {
            println!("Eventor request: {}", url.as_str());
        }

        let request = self
            .client
            .get(url)
            .header("ApiKey", self.api_key)
            .query(parameters);

        let mut hasher = DefaultHasher::new();
        format!("{:?}", request).hash(&mut hasher);
        let request_hash = hasher.finish();
        let file_name = format!("{}.cache.xml", request_hash);

        let cache_path = self.cache_folder.join(Path::new(&file_name));
        let result = if cache_path.exists() {
            if self.verbose {
                println!("\tReading from cache at {:?}.", cache_path);
            }
            read_to_string(cache_path)
        } else {
            if self.verbose {
                println!(
                    "\tPerforming request to Eventor. Will save to cache at {:?}.",
                    cache_path
                );
            }
            request
                .send()
                .expect("Unable to construct request to Eventor server.")
                .text()
                .and_then(|s: String| -> Result<String, reqwest::Error> {
                    if let Err(_) = std::fs::write(cache_path, s.clone()) {
                        println!("\tUnable to save request data.");
                    }
                    Ok(s)
                })
                .map_err(|error: reqwest::Error| -> std::io::Error {
                    std::io::Error::new(ErrorKind::Other, error.to_string())
                })
        };

        result
            .map(|s| xmltree::Element::parse(s.as_bytes()).expect("Invalid XML from Eventor."))
            .expect("Unable to load XML.")
    }
}
