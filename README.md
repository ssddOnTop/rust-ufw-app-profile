![Visitor Badge](https://visitor-badge.laobi.icu/badge?page_id=rust-ufw-app-profile)
![Crates Badge](https://img.shields.io/crates/v/ufwprofile)
![Crates Downloads](https://img.shields.io/crates/d/ufwprofile)

# About Project
UFW app profile written in pure rust.

# Example
```rust
    let mut x = ufwprofile::config::UFWConf::default();
    x.append_ports("80", "")
        .append_ports("81:82", "tcp")
        .append_ports("84", "udp")
        .append_ports("83", "")
        .append_ports("8000", "tcp")
        .init("Foo", "Alo", "Alo").unwrap();
    println!("{}",x.try_adding_to_ufw(true).unwrap());
```