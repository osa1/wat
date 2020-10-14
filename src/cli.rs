use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub(crate) struct Args {
    pub(crate) files: Vec<String>,
}

pub(crate) fn parse() -> Args {
    let mut version = crate_version!().to_owned();
    let commit_hash = env!("GIT_HASH");
    if !commit_hash.is_empty() {
        version = format!("{} ({})", version, commit_hash);
    }

    let m = App::new(crate_name!())
        .version(version.as_str())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("files")
                .required(false)
                .multiple(true)
                .help("Files to process"),
        )
        .get_matches();

    Args {
        files: m
            .values_of("files")
            .map(|files| files.map(|s| s.to_owned()).collect())
            .unwrap_or_default(),
    }
}
