![Visitor Badge](https://visitor-badge.laobi.icu/badge?page_id=rust-ufw-app-profile)
![Crates Badge](https://img.shields.io/crates/v/ufwprofile)
![Crates Downloads](https://img.shields.io/crates/d/ufwprofile)

# About Project
UFW app profile written in pure rust.

# Example
```rust
    if ufwprofile::config::UFWConf::check_permissions() {
    //checks if ufw exists and the path /etc/ufw/applications.d is writable
    let mut x = ufwprofile::config::UFWConf::default();
    x.append_ports("80", "")
        .append_ports("81:82", "tcp")
        .append_ports("84", "udp")
        .append_ports("83", "")
        .append_ports("8000", "tcp")
        .init("AppName", "Title", "Description").unwrap();
    println!("{}",x.try_adding_to_ufw(true).unwrap());
// alternatively you can call x.try_write() which just writes the file 
// at /etc/ufw/applications.d and not enable/disable the profile.
}
```
# Drawbacks
### This is hardcoded dependency
1. The config file is hardcoded.
2. The path is assumed to be `/etc/ufw/applications.d`

# Changelog
check [CHANGELOG.md](CHANGELOG.md)