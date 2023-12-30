![Visitor Badge](https://visitor-badge.laobi.icu/badge?page_id=rust-ufw-app-profile)
![Crates Badge](https://img.shields.io/crates/v/ufwprofile)
![Crates Downloads](https://img.shields.io/crates/d/ufwprofile)

# About Project

UFW app profile written in pure rust.

## Implementation:

```toml
ufwprofile = "" #check latest version above
```

Or

```
cargo add ufwprofile
```

# Example

```rust
fn main() -> anyhow::Result<()> {
    if ufwprofile::UFWConf::check_write_permission() {
        //checks if ufw exists and the path /etc/ufw/applications.d is writable
        let conf = ufwprofile::UFWConf::init("AppName", "Title", "Description")?
            .append_ports("80", "")?
            .append_ports("81:82", "tcp")?
            .append_ports("84", "udp")?
            .append_ports("83", "")?
            .append_ports("8000", "tcp")?;

        if ufwprofile::UFWConf::is_root() {
            // check if the app has root permission.
            println!("{}", conf.try_adding_to_ufw(true).unwrap());
        } else {
            println!("{}", conf.try_write_with_sudo(true).unwrap());
        }
    } else {
        println!("Unable to write");
    }
    Ok(())
}
```

# Drawbacks

### This is hardcoded dependency

1. The config file is hardcoded.
2. The path is assumed to be `/etc/ufw/applications.d`

# Changelog

check [CHANGELOG.md](CHANGELOG.md)
