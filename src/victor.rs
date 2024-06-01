use std::collections::HashMap;
use std::os::unix::fs::FileExt;
use log::info;
use serde_json::Value as JsonValue;
use serde_json::Map as JsonMap;
use std::{fs::File, io::copy};
use anyhow::Result;
use log::{debug, trace};
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::Path;
use std::fs;
use std::io::{SeekFrom, Seek};

use crate::error::{VicResult, VicError, IntoVicResult};
use crate::vic_err;

pub static BUILD_CONFIG: &str = include_str!(std::env!("VIC_BUILD_CONFIG"));

// a struct for state
pub struct Victor {
    config: VictorConfig,
}

impl Victor {
    pub fn new() -> VicResult<Victor> {
        Ok(Victor { config: VictorConfig::new()? })
    }

    pub fn config_get<P: IntoVecString>(&self, path: P) -> VicResult<String> {
        self.config.get(path.to_vec_string())
    }

    pub fn config_set<P: IntoVecString, V: Into<String>>(&mut self, path: P, value: V) -> VicResult<()> {
        self.config.set(path.to_vec_string(), value.into());
        Ok(())
    }

    pub fn config_exists<P: IntoVecString>(&self, path: P) -> bool {
        self.config.exists(path.to_vec_string())
    }
    
    fn fetch_tarbal(&mut self) -> VicResult<()> {
        if !self.config_exists("internal.tarball_is_fetched") || self.config_get("internal.tarball_is_fetched")? != "true" {
            // create_folder if it doesn't already
            self.create_folder()?;

            let mut url = self.config_get("url")? + "/tars/x86_64-linux.tar.gz";
            let vic_folder = self.config_get("vic_folder")?;
            let tmp_file_path_gz = vic_folder.clone() + "/tmp.tar.gz";

            info!("fetching tarball from: {}", url);

            let mut response = reqwest::blocking::get(url)?;

            let mut tmp_file_gz = File::create(&tmp_file_path_gz)?;

            copy(&mut response, &mut tmp_file_gz)?;

            let tar_gz = File::open(&tmp_file_path_gz)?;
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            //archive.set_mask(umask::Mode::parse("rwxrwxrwx")?.into());
            archive.unpack(&vic_folder)?;

            fs::remove_file(&tmp_file_path_gz)?;

            self.config_set("internal.tarball_is_fetched", "true");
        };
        Ok(())
    }

    fn create_folder(&self) -> VicResult<()> {
        let path = self.config_get("vic_folder")?;
        info!("creating vic_folder at: {}", path);
        if !Path::new(&path).exists() {
            fs::create_dir(path);
        }
        Ok(())
    }

    pub fn run_from_vic_pkgs(&mut self, runnable: &str) -> VicResult<()> {

        self.fetch_tarbal()?;

        //self.fetch_vic_src()?;

        println!("running from vic pkgs: {}", runnable);
        println!("config: {}", BUILD_CONFIG);
        Ok(())
    }

    pub fn run_from_resource(&self, runnable: &str) -> VicResult<()> {
        println!("running from resource: {}", runnable);
        Ok(())
    }

    pub fn run_flake_url(&self, runnable: &str) -> VicResult<()> {
        println!("running flake url: {}", runnable);
        Ok(())
    }

}

struct VictorConfig {
    inner: HashMap<String, String>,
}

impl VictorConfig {
    pub fn new() -> VicResult<VictorConfig> {
        let mut config = VictorConfig::empty();
        config.read_build_config()?;
        config.read_folder_config()?;
        Ok(config)
    }

    pub fn empty() -> VictorConfig {
        VictorConfig { inner: HashMap::new() }
    }

    fn read_folder_config(&mut self) -> VicResult<()> {
        debug!("reading config from vic_folder");
        trace!("read folder config ... VictorConfig is now: {:?}", self.inner);
        self.fix_vic_folder_path()?;
        Ok(())
    }

    // this fn replaces a leading '~' in the vic_folder conf var with an absolute path to a users
    // home dir
    fn fix_vic_folder_path(&mut self) -> VicResult<()> {
        let old_vic_folder = self.get(vec!["vic_folder".to_owned()])?;

        let first_char = old_vic_folder.chars().nth(0)
            .ok_or(vic_err!("vic_folder config path is an empty string, should not be possible"))?;

        if first_char == '~' {
            debug!("substituting ~ with val from $HOME in vic_folder");
            let home_path = std::env::var("HOME").vic_result_msg("Could not get $HOME")?;
            let new_vic_folder = old_vic_folder.replace("~", &home_path);
            self.set(vec!["vic_folder".to_owned()], new_vic_folder)?;
        }

        Ok(())
    }

    fn read_build_config(&mut self) -> VicResult<()> {
        debug!("reading config that was set at build time");
        let mut json_val: JsonValue = serde_json::from_str(BUILD_CONFIG)
            .vic_result_msg("Error parsing string from BUILD_CONFIG")
            .map_err(|e| e.msg("which is read at build time from a file specified in the VIC_BUILD_CONFIG env var"))?;

        if let JsonValue::Object(ref mut map) = json_val {
            let vec = json_map_to_vec(map, Vec::new(), Vec::new())?;
            for (key, val) in vec {
                self.inner.insert(key, val);
            }
        } else {
            return Err(vic_err!("root of json string from BUILD_CONFIG is not a map"));
        }
        self.fix_vic_folder_path()?;
        trace!("read build config ... VictorConfig is now: {:?}", self.inner);
        Ok(())
    }

    pub fn get(&self, path: Vec<String>) -> VicResult<String> {
        let val = self.inner.get(&path.join("."))
            .ok_or(vic_err!("Could not get config path '{}'", path.join(".")))?;
        return Ok(val.to_owned());
    }

    pub fn set(&mut self, path: Vec<String>, val: String) -> VicResult<()> {
        self.inner.insert(path.join("."), val);
        Ok(())
    }

    pub fn exists(&self, path: Vec<String>) -> bool {
        self.inner.contains_key(&path.join("."))
    }
}


fn json_map_to_vec(map: &mut JsonMap<String, JsonValue>, cur_path: Vec<String>, mut out_vec: Vec<(String, String)>) -> VicResult<Vec<(String, String)>> {
    for (key, value) in map.iter_mut() {
        trace!("json_map_to_vec loop - key: {}", key);
        trace!("json_map_to_vec loop - value: {}", value);
        let mut inner_path = cur_path.clone();
        inner_path.push(key.to_owned());
        match value {
            JsonValue::Object(ref mut inner_map) => {
                return json_map_to_vec(inner_map, inner_path, out_vec);
            },
            JsonValue::String(string) => {
                out_vec.push((inner_path.join("."), string.to_owned()));
            }
            val => {
                out_vec.push((inner_path.join("."), format!("{}", val)));
            }
        }
    };
    Ok(out_vec)
}

trait IntoVecString {
    fn to_vec_string(self) -> Vec<String>;
}

impl IntoVecString for &str {
    fn to_vec_string(self) -> Vec<String> {
        self.to_owned().split(".").map(|v| v.to_owned()).collect()
    }
}



