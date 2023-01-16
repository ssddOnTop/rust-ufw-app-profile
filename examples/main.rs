use ufwprofile::config::UFWConf;

#[cfg(test)]
mod tests{
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    #[test]
    fn foo(){
        let x = match File::create("foo.txt") {
            Ok(mut f) => {
                match f.write_all(b"alo") {
                    Ok(_) => true,
                    Err(_) => false
                }
            },
            Err(_) => false
        };
        println!("{x}");
    }

    #[test]
    fn bar(){
        let x = match File::create(format!("/etc/ufw/applications.d/foo").as_str()) {
            Ok(mut f) => {
                match f.write_all(b"alo") {
                    Ok(_) => {
                        let mut x = Command::new("ufw");
                        match x.arg("allow").arg("foo").output() {
                            Ok(_) => true,
                            Err(_) => false
                        }
                    },
                    Err(_) => false
                }
            },
            Err(_) => false
        };

    }

}

fn main() {
    println!("{}",UFWConf::check_permissions());
    /*let mut x = ufwprofile::config::UFWConf::default();
    x.append_ports("80", "")
        .append_ports("81:82", "tcp")
        .append_ports("84", "udp")
        .append_ports("83", "")
        .append_ports("8000", "tcp")
        .init("Foo", "Alo", "Alo").unwrap();
    println!("{}",x.try_adding_to_ufw(true).unwrap());*/
}