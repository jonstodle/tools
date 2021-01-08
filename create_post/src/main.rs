use chrono::prelude::*;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "create-post")]
struct Opt {
    #[structopt(name = "NAME")]
    name: String,

    #[structopt(name = "TAGS")]
    tags: Vec<String>,

    #[structopt(short, long)]
    editor: Option<String>,
}

fn main() {
    let opts: Opt = Opt::from_args();

    if opts.name.len() == 0 {
        println!("Name of post must be at least one character");
        std::process::exit(1);
    }

    if opts.tags.is_empty() {
        println!("The post must have at least one tag");
        std::process::exit(1);
    }

    let file_name = format!(
        "{}.md",
        opts.name
            .to_lowercase()
            .replace(&[' ', '\''][..], "-")
            .replace('.', "")
    );

    let front_matter = format!(
        r#"+++
title = "{}"
date = {}
[taxonomies]
tags = [{}]
+++
"#,
        opts.name,
        Local::now().format("%Y-%m-%d"),
        opts.tags
            .iter()
            .map(|t| format!(r#""{}""#, t))
            .collect::<Vec<String>>()
            .join(", ")
    );

    File::create(&file_name)
        .expect("Unable to create file")
        .write_all(front_matter.as_bytes())
        .expect("Unable to write front matter to file");

    let editor = opts.editor.or(std::env::var("EDITOR").ok());

    if let Some(editor) = editor {
        std::process::Command::new(editor)
            .arg(&file_name)
            .status()
            .expect("Failed to launch editor");
    } else {
        println!("Unable to resolve editor. File created: {}", file_name);
    }
}
