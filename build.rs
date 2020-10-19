extern crate conan;

use std::path;

fn main() {
    let is_conan_success = if let Some(_) = conan::find_program() {
        try_conan()
    } else {
        false
    };

    if !is_conan_success {
        panic!("Conan failed.");
    }
}

fn try_conan() -> bool {
    let remote_list = conan::get_remote_list();
    let mut missing_remotes: Vec<conan::Remote> = vec![];

    let conan_center = conan::Remote {
        name: String::from("conan-center"),
        url: String::from("https://conan.bintray.com"),
    };

    let conan_transit = conan::Remote {
        name: String::from("conan-transit"),
        url: String::from("https://api.bintray.com/conan/conan/conan-transit"),
    };

    if let None = remote_list
        .iter()
        .find(|&remote| remote.url == conan_center.url)
    {
        missing_remotes.push(conan_center);
    }

    if let None = remote_list
        .iter()
        .find(|&remote| remote.url == conan_transit.url)
    {
        missing_remotes.push(conan_transit);
    }

    if !missing_remotes.is_empty() {
        let mut msg = String::from("The required packages are not found in the current remotes.\n");

        for remote in missing_remotes {
            msg.push_str(
                format!(
                    "note: execute the following command - \"conan remote add {} {}\"\n",
                    remote.name, remote.url
                )
                .as_str(),
            );
        }
        panic!(msg);
    }

    let install_command = conan::InstallCommandBuilder::new()
        .build_policy(conan::BuildPolicy::Missing)
        .recipe_path(path::Path::new("conanfile.txt"))
        .build();

    if let Some(build_info) = install_command.generate() {
        build_info.cargo_emit();
        return true;
    }

    false
}
