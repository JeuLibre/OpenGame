use std::collections::HashMap;
use std::process::{exit, Command};

use super::fedora;
use crate::log;

pub struct Installer {
    pub root: String,
    pub commands: HashMap<&'static str, Vec<String>>,
}

impl Installer {
    pub fn new() -> Installer {
        let roots_list = ["sudo", "doas", "su"];
        let mut rt = String::new();

        for root in roots_list {
            let res = Command::new("command").arg("-v").arg(root).status();

            match res {
                Ok(r) => {
                    rt = root.to_string();
                    break;
                }
                Err(_e) => {}
            }
        }

        Installer {
            root: rt,
            commands: HashMap::new(),
        }
    }

    pub fn add_command(mut self: Self, key: &'static str, value: Vec<String>) -> Self {
        self.commands.insert(key, value);
        self
    }

    pub fn find_all_commands(mut self: Self) -> Self {
        let res = Command::new("lsb_release").arg("-is").output();

        match res {
            Ok(r) => {
                log::success("Find distro id");
                let distro_utf8 = String::from_utf8(r.stdout).unwrap_or_default();
                let distro_name = &distro_utf8[..distro_utf8.len() - 1];

                match distro_name {
                    "Fedora" => {
                        self = self.add_command("all", fedora::all_commands());
                        self = self.add_command("lutris", fedora::lutris());
                        self = self.add_command("heroic", fedora::heroic_launcher());
                        self = self.add_command("overlay", fedora::overlay());
                    }
                    _ => {
                        log::error("Can't find a gaming dependencies for this distro");
                        exit(-1);
                    }
                }
            }
            Err(e) => log::error(&e.to_string()),
        }

        self
    }
}
