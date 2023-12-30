use crate::rootcheck;
use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::process::Command;

///  Struct that contains app name, config string, ports HashMap
/// Example:
/// ```
/// fn main() -> anyhow::Result<()> {
///     if ufwprofile::UFWConf::check_write_permission() {
///         //checks if ufw exists and the path /etc/ufw/applications.d is writable
///         let conf = ufwprofile::UFWConf::init("AppName", "Title", "Description")?
///             .append_ports("80", "")?
///             .append_ports("81:82", "tcp")?
///             .append_ports("84", "udp")?
///             .append_ports("83", "")?
///             .append_ports("8000", "tcp")?;
///
///         if ufwprofile::UFWConf::is_root() {
///             // check if the app has root permission.
///             println!("{}", conf.try_adding_to_ufw(true).unwrap());
///         } else {
///             println!("{}", conf.try_write_with_sudo(true).unwrap());
///         }
///     }
///     Ok(())
/// }
/// ```
#[derive(Default, Clone)]
pub struct UFWConf {
    app_name: String,
    config: String,
}

impl UFWConf {
    pub fn is_root() -> bool {
        rootcheck::escalate_if_needed()
    }

    /// To add ports to the config, pass `port` and `protocol` to be added
    ///
    /// You can pass empty string to allow all protocols
    pub fn append_ports(&mut self, port: &str, protocol: &str) -> Result<Self> {
        let cfg = format_ports(port, protocol)?;
        if self.config.chars().last().unwrap().eq(&'|') {
            self.config.pop();
        }
        self.config = format!("{}|{}", self.config, cfg);
        Ok(self.clone())
    }

    /// check required permissions and if ufw is installed
    pub fn check_write_permission() -> bool {
        match Command::new("ufw").arg("version").spawn() {
            Ok(_) => !std::fs::metadata("/etc/ufw/applications.d/")
                .unwrap()
                .permissions()
                .readonly(),
            Err(_) => false,
        }
    }

    /// Pass `app_name`, `title` and `description` of the app.
    pub fn init(app_name: &str, title: &str, description: &str) -> Result<Self> {
        let app_name = app_name.replace(' ', "");
        let config = format!(
            "[{}]\ntitle={}\ndescription={}\nports=\n",
            app_name, title, description,
        );
        Ok(Self { app_name, config })
    }

    /// returns config string.
    pub fn get_config_string(&self) -> String {
        self.config.clone()
    }
    /// tries writing it to `/etc/ufw/applications.d/`
    pub fn try_write(&self) -> Result<(), Error> {
        let path = format!("/etc/ufw/applications.d/ufw-{}", self.app_name);
        if Path::new(path.as_str()).exists() {
            std::fs::remove_file(path.as_str()).unwrap();
        }

        match File::create(path) {
            Ok(mut f) => match f.write_all(self.config.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// pass `true` if you want to ALLOW the ports and `false` to DENY the ports.
    pub fn try_write_with_sudo(&self, allow: bool) -> Result<String, Error> {
        let path = format!("/etc/ufw/applications.d/ufw-{}", self.app_name);
        if Path::new(path.as_str()).exists() {
            std::fs::remove_file(path.as_str()).unwrap();
        }
        match File::create(path) {
            Ok(mut f) => match f.write_all(self.config.as_bytes()) {
                Ok(_) => {
                    let mut x = Command::new("sudo");
                    match allow {
                        true => {
                            match x
                                .arg("ufw")
                                .arg("allow")
                                .arg(self.app_name.clone())
                                .output()
                            {
                                Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                                Err(e) => Err(e),
                            }
                        }
                        false => {
                            match x.arg("ufw").arg("deny").arg(self.app_name.clone()).output() {
                                Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
                Err(e) => Err(Error::new(
                    ErrorKind::Other,
                    format!("Error writing file {}", e),
                )),
            },
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Error creating file {}", e),
            )),
        }
    }

    /// pass `true` if you want to ALLOW the ports and `false` to DENY the ports.
    pub fn try_adding_to_ufw(&self, allow: bool) -> Result<String> {
        let path = format!("/etc/ufw/applications.d/ufw-{}", self.app_name);
        if Path::new(path.as_str()).exists() {
            std::fs::remove_file(path.as_str()).unwrap();
        }
        match File::create(path) {
            Ok(mut f) => match f.write_all(self.config.as_bytes()) {
                Ok(_) => {
                    let mut x = Command::new("ufw");
                    match allow {
                        true => match x.arg("allow").arg(self.app_name.clone()).output() {
                            Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                            Err(_) => {
                                let mut x = Command::new("sudo");
                                match x
                                    .arg("ufw")
                                    .arg("allow")
                                    .arg(self.app_name.clone())
                                    .output()
                                {
                                    Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                                    Err(e) => Err(anyhow!(format!(
                                        "Error running ufw deny {}: {}",
                                        self.app_name, e
                                    ),)),
                                }
                            }
                        },
                        false => match x.arg("deny").arg(self.app_name.clone()).output() {
                            Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                            Err(e) => Err(anyhow!("{}", e)),
                        },
                    }
                }
                Err(e) => Err(anyhow!(format!("Error writing file {}", e),)),
            },
            Err(e) => Err(anyhow!(format!("Error creating file {}", e),)),
        }
    }
}

fn format_ports(port: &str, protocol: &str) -> Result<String> {
    check_ports(port, protocol)?;
    let protocol = if protocol.is_empty() {
        String::new()
    } else {
        format!("/{protocol}")
    };
    Ok(format!("{port}{protocol}|"))
}

#[inline]
fn check_ports(port_range: &str, protocol: &str) -> Result<()> {
    let port_pattern = Regex::new(r"^\d+:\d+$")?;
    if !["tcp", "udp", ""].contains(&protocol) {
        return Err(anyhow!("Invalid protocol {port_range}={protocol}"));
    }
    if !port_pattern.is_match(port_range) {
        return Err(anyhow!("Bad port/range at {port_range}={protocol}"));
    }
    Ok(())
}
