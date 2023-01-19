#[forbid(missing_docs)]
#[forbid(unused_imports)]
#[forbid(unsafe_code)]
use std::collections::HashMap;
use std::fmt::Error;
use std::fs::{File};
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::process::Command;
use crate::rootcheck;


///  Struct that contains app name, config string, ports HashMap
/// Example:
/// ```
/// fn main() {
///     let mut x = ufwprofile::config::UFWConf::default();
///     x.append_ports("80", "")
///         .append_ports("81:82", "tcp")
///         .append_ports("84", "udp")
///         .append_ports("83", "")
///         .append_ports("8000", "tcp")
///         .init("Foo", "Alo", "Alo").unwrap();
///    println!("{}",x.try_adding_to_ufw(true).unwrap());
/// }
/// ```
pub struct UFWConf {
    app_name: String,
    config: String,
    ports_map: HashMap<String, String>,
}

impl Default for UFWConf {
    /// Default function to initiate UFWConf struct.
    fn default() -> Self {
        UFWConf {
            app_name: "".to_string(),
            config: "".to_string(),
            ports_map: Default::default(),
        }
    }
}

impl UFWConf {
    pub fn is_root() -> bool {
        rootcheck::escalate_if_needed()
    }

    /// To add ports to the config, pass `port` and `protocol` to be added
    ///
    /// You can pass empty string to allow all protocols
    pub fn append_ports(&mut self, port: &str, protocol: &str) -> &mut UFWConf {
        self.ports_map.insert(port.to_string(), protocol.to_string());
        self
    }

    /// check required permissions and if ufw is installed
    pub fn check_write_permission() -> bool {
        match Command::new("ufw").arg("version").spawn() {
            Ok(_) => {
                !std::fs::metadata("/etc/ufw/applications.d/").unwrap().permissions().readonly()
            }
            Err(_) => {
                false
            }
        }
    }

    /// Pass `app_name`, `title` and `description` of the app.
    pub fn init(&mut self, app_name: &str, title: &str, description: &str) -> Result<&mut UFWConf, Error> {
        // let mut uc = UFWConf::default();
        /* self.app_name = app_name.clone();
         self.title = title;
         self.description = description;
         self.ports = format_ports(port);*/

        self.app_name = app_name.to_string().replace(" ", "");
        let x = format_ports(self.ports_map.clone())?;
        let x = format!("[{}]\ntitle={}\ndescription={}\nports={}\n", self.app_name.clone(), title, description, x);
        // println!("{}", x.clone());
        self.config = x;
        Ok(self)
    }


    /// returns config string.
    pub fn get_config_string(&self) -> String {
        self.config.clone()
    }

    /// tries writing it to `/etc/ufw/applications.d/`
    pub fn try_write(&self) -> Result<(), Error> {
        let path = format!("/etc/ufw/applications.d/{}", self.app_name);
        if Path::new(path.as_str()).exists() {
            std::fs::remove_file(path.as_str()).unwrap();
        }

        match File::create(path) {
            Ok(mut f) => {
                match f.write_all(self.config.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    /// pass `true` if you want to ALLOW the ports and `false` to DENY the ports.
    pub fn try_adding_to_ufw_with_sudo(&self, allow: bool) -> Result<String, Error> {
        let path = format!("/etc/ufw/applications.d/{}", self.app_name);
        if Path::new(path.as_str()).exists() {
            std::fs::remove_file(path.as_str()).unwrap();
        }
        match File::create(path) {
            Ok(mut f) => {
                match f.write_all(self.config.as_bytes()) {
                    Ok(_) => {
                        let mut x = Command::new("sudo");
                        match allow {
                            true => {
                                match x.arg("ufw").arg("allow").arg(self.app_name.clone()).output() {
                                    Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                                    Err(e) => Err(e)
                                }
                            }
                            false => {
                                match x.arg("ufw").arg("deny").arg(self.app_name.clone()).output() {
                                    Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                                    Err(e) => Err(e)
                                }
                            }
                        }
                    }
                    Err(e) => Err(Error::new(ErrorKind::Other, format!("Error writing file {}", e)))
                }
            }
            Err(e) => Err(Error::new(ErrorKind::Other, format!("Error creating file {}", e)))
        }
        // md.permissions().set_readonly(true);
        // Ok(())
    }
}

/// pass `true` if you want to ALLOW the ports and `false` to DENY the ports.
pub fn try_adding_to_ufw(&self, allow: bool) -> Result<String, Error> {
    let path = format!("/etc/ufw/applications.d/{}", self.app_name);
    if Path::new(path.as_str()).exists() {
        std::fs::remove_file(path.as_str()).unwrap();
    }
    match File::create(path) {
        Ok(mut f) => {
            match f.write_all(self.config.as_bytes()) {
                Ok(_) => {
                    let mut x = Command::new("ufw");
                    match allow {
                        true => {
                            match x.arg("allow").arg(self.app_name.clone()).output() {
                                Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                                Err(e) => Err(e)
                            }
                        }
                        false => {
                            match x.arg("deny").arg(self.app_name.clone()).output() {
                                Ok(d) => Ok(String::from_utf8(d.stdout).unwrap()),
                                Err(e) => Err(e)
                            }
                        }
                    }
                }
                Err(e) => Err(Error::new(ErrorKind::Other, format!("Error writing file {}", e)))
            }
        }
        Err(e) => Err(Error::new(ErrorKind::Other, format!("Error creating file {}", e)))
    }
    // md.permissions().set_readonly(true);
    // Ok(())
}
}

fn format_ports(port: HashMap<String, String>) -> Result<String, Error> {
    let x = check_ports(port.clone());
    if x != "1" {
        return Err(Error::new(ErrorKind::Other, x));
    }
    let mut x = String::new();
    let mut y = String::new();
    for (k, v) in port.iter() {
        if !v.is_empty() && !y.is_empty() {
            y = y + "|" + k + "/" + v;
            continue;
        } else if y.is_empty() && !v.is_empty() {
            y = k.to_owned() + "/" + v;
            continue;
        }

        if x.is_empty() {
            x = k.to_owned();
            continue;
        }
        x = x + "," + k;
    }
    Ok(format!("{},{}", x, y))
}

fn check_ports(p: HashMap<String, String>) -> String {
    for (k, v) in p.iter() {
        if v != "tcp" && v != "udp" && !v.is_empty() {
            return format!("Bad port at {}", v);
        }
        if k.contains(":") {
            continue;
        }
        match k.parse::<usize>() {
            Ok(_) => continue,
            Err(_) => {
                return format!("Bad port at {}", k);
            }
        }
    }
    "1".to_string()
}