use std::{fs, thread::sleep, time::Duration};
use xshell::cmd;

pub static PACS: &[(&str, &str)] = &[
    ("PY32T020xx", "thumbv6m-none-eabi"),
    ("PY32c640xx", "thumbv6m-none-eabi"),
    ("PY32f002axx", "thumbv6m-none-eabi"),
    ("PY32f030xx", "thumbv6m-none-eabi"),
    ("PY32f071cxx", "thumbv6m-none-eabi"),
    ("PY32f303xx", "thumbv6m-none-eabi"),
    ("PY32m070xx", "thumbv6m-none-eabi"),
    ("PY32L020xx", "thumbv6m-none-eabi"),
    ("PY32c641xx", "thumbv6m-none-eabi"),
    ("PY32f002bxx", "thumbv6m-none-eabi"),
    ("PY32f031xx", "thumbv6m-none-eabi"),
    ("PY32f071xx", "thumbv6m-none-eabi"),
    ("PY32f403xx", "thumbv6m-none-eabi"),
    ("PY32c610xx", "thumbv6m-none-eabi"),
    ("PY32c670xx", "thumbv6m-none-eabi"),
    ("PY32f002xxx", "thumbv6m-none-eabi"),
    ("PY32f040cxx", "thumbv6m-none-eabi"),
    ("PY32f072cxx", "thumbv6m-none-eabi"),
    ("PY32m010xx", "thumbv6m-none-eabi"),
    ("PY32c611xx", "thumbv6m-none-eabi"),
    ("PY32f001xx", "thumbv6m-none-eabi"),
    ("PY32f003xx", "thumbv6m-none-eabi"),
    ("PY32f040xx", "thumbv6m-none-eabi"),
    ("PY32f072xx", "thumbv6m-none-eabi"),
    ("PY32m030xx", "thumbv6m-none-eabi"),
];

pub fn install_tools() {
    // Install embedded Rust targets.
    let mut targets = PACS.iter().map(|(_, target)| *target).collect::<Vec<_>>();
    targets.sort();
    targets.dedup();

    cmd!("rustup target add {targets...}").run().unwrap();

    // Install meta-dependencies used for generating the crates.
    let toml = fs::read_to_string("Cargo.toml").unwrap();
    let mut metadeps = Vec::new();
    for line in toml.lines() {
        if line.starts_with("# @") {
            let parts = line[3..].split('=').collect::<Vec<_>>();
            match &*parts {
                [name, version] => {
                    metadeps.push((name.trim(), version.trim()));
                }
                _ => panic!("malformed metadep line: {}", line),
            }
        }
    }

    for (name, version) in metadeps {
        println!("installing {} {}", name, version);
        cmd!("cargo install {name} --version {version}")
            .run()
            .unwrap();
    }
}

pub fn generate() {
    install_tools();

    for (pac, _target) in PACS {
        let svd_path = format!("svds/{}.svd", pac);
        let crate_dir = format!("pacs/{}-pac", pac);
        fs::create_dir_all(&crate_dir).unwrap();

        cmd!("svd2rust -i {svd_path} -o {crate_dir}").run().unwrap();
        cmd!("form -i {crate_dir}/lib.rs -o {crate_dir}/src")
            .run()
            .unwrap();
        fs::remove_file(format!("{}/lib.rs", crate_dir)).unwrap();
    }

    cmd!("cargo fmt").run().unwrap();
}

pub fn is_git_clean() -> bool {
    cmd!("git status --porcelain").read().unwrap().is_empty()
}

fn file_replace(path: &str, from: &str, to: &str, dry_run: bool) {
    let old_contents = fs::read_to_string(path).unwrap();
    let new_contents = old_contents.replacen(from, to, 1);
    if old_contents == new_contents {
        panic!("failed to replace `{}` -> `{}` in `{}`", from, to, path);
    }

    if !dry_run {
        fs::write(path, new_contents).unwrap();
    }
}

pub fn publish(dry_run: bool) {
    // Pre-publish checks.

    generate();

    if !dry_run {
        assert!(
            !is_git_clean(),
            "working directory clean, please leave it dirty, containing the desired version bump"
        );
    }

    let toml = fs::read_to_string("Cargo.toml").unwrap();
    let needle = "version = \"";
    let version_pos = toml.find(needle).unwrap() + needle.len();
    let version_rest = &toml[version_pos..];
    let end_pos = version_rest.find('"').unwrap();
    let version = &version_rest[..end_pos];

    // Bump the changelog first, also check that it isn't empty.
    let changelog_path = "CHANGELOG.md";
    let changelog = fs::read_to_string(changelog_path).unwrap();

    if changelog.contains(&format!("## [{}]", version)) {
        println!(
            "changelog already contains section for version {}, not updating",
            version
        );
    } else {
        // (ignore empty changelog when this is a dry_run, since that runs in normal CI)
        assert!(
            dry_run || !changelog.contains("(no changes)"),
            "changelog contains `(no changes)`; please fill it"
        );

        println!("bumping changelog to {}", version);

        // Prepend empty "Unreleased" section, promote the current one.
        let from = String::from("## Unreleased");
        let to = format!("## Unreleased\n\n(no changes)\n\n## [{}]", version);
        file_replace(changelog_path, &from, &to, dry_run);

        // Append release link at the end.
        let mut changelog = fs::read_to_string(changelog_path).unwrap();
        changelog.push_str(&format!(
            "[{vers}]: https://github.com/hysonglet/py32-pacs/releases/tag/v{vers}\n",
            vers = version
        ));
        if !dry_run {
            fs::write(changelog_path, changelog).unwrap();
        }
    }

    if !dry_run {
        let message = format!("Release v{}", version);
        let tag = format!("v{}", version);
        cmd!("git commit -am {message}").run().unwrap();
        cmd!("git tag -a {tag} -m {tag}").run().unwrap();
    }

    // Run `cargo publish` on each crate.
    for (pac, target) in PACS {
        let mut cmd =
            cmd!("cargo publish --manifest-path pacs/{pac}-pac/Cargo.toml --target {target}");
        if dry_run {
            cmd = cmd.arg("--dry-run").arg("--no-verify");
        }
        cmd.run().unwrap();
        sleep(Duration::from_secs(10));
    }
}
