extern crate fs_extra;
extern crate pulldown_cmark;
extern crate ramhorns;
extern crate regex;
extern crate toml;
#[macro_use]
extern crate die;

use fs_extra::dir;
use pulldown_cmark::{html, Parser};
use ramhorns::{Ramhorns, Template};
use regex::{Captures, Regex};

use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, read_dir, read_to_string, write};
use std::path::{Path, PathBuf};

static TEMPLATE: &str = "{{{body}}}";

fn main() -> Result<(), Box<dyn Error>> {
    let content_regex = Regex::new(
        r#"\[\[(?P<content>(((\.\.?/)|([.a-zA-Z0-9_/\-\\]))+(\.[a-zA-Z0-9]+)?))(?P<template> +(((\.\.?/)|([.a-zA-Z0-9_/\-\\]))+(\.[a-zA-Z0-9]+)?))?\]\]"#,
    )?;
    let mut template_cache = {
        let mut template_cache = HashMap::new();
        template_cache.insert("base".to_string(), Template::new(TEMPLATE)?);
        template_cache
    };

    let template_files = read_dir("theme")?
        .into_iter()
        .filter_map(|x| {
            let path = x.ok()?.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    create_dir_all("build")?;
    for target in ["media", "theme/static"] {
        dir::copy(target, "build/", &{
            let mut c = dir::CopyOptions::new();
            c.overwrite = true;
            c
        })?;
    }

    let mut templates = Ramhorns::from_folder("theme")?;

    template_files.iter().for_each(|path| {
        let tpl = templates
            .from_file(&path.strip_prefix("theme").unwrap().display().to_string())
            .unwrap();

        if let Err(e) = tpl.render_to_file(
            &PathBuf::from("build").join(&path.strip_prefix("theme").unwrap()),
            &(),
        ) {
            die!("failed to render to file: {}", e);
        }
    });

    let built = read_dir("build")?
        .into_iter()
        .filter_map(|f| {
            let x = f.ok().map(|x| x.path())?;
            let s = read_to_string(&x).ok()?;
            Some((x, s))
        })
        .collect::<Vec<(PathBuf, String)>>();

    for (path, contents) in built.iter() {
        let processed = content_regex.replace_all(contents, |caps: &Captures| {
            let content = caps["content"].to_owned();
            let path = Path::new(&content);
            let mut files = match path {
                p if !p.exists() => die!("path does not exist: {}", p.display()),
                p if p.is_file() => vec![p.to_owned()],
                p if p.is_dir() => read_dir(p)
                    .unwrap_or_else(|_| die!("could not read directory {}", p.display()))
                    .filter_map(|x| x.ok().map(|x| dbg!(x.path())))
                    .filter(|x| {
                        dbg!(x.is_file()) && dbg!(x.to_str().unwrap_or_default().ends_with(".md"))
                    })
                    .inspect(|x| {
                        dbg!(x);
                    })
                    .collect::<Vec<PathBuf>>(),
                p => die!("invalid path: {}", p.display()),
            };

            files.sort();

            dbg!(&files);

            let mut s = String::new();

            println!("{}", caps.len());
            for f in files {
                let content = match read_to_string(dbg!(f)) {
                    Ok(s) => s,
                    Err(e) => die!("failed to read file {}: {}", path.display(), e),
                };

                let tpl_name = caps
                    .name("template")
                    .map(|x| x.as_str().trim())
                    .unwrap_or("base")
                    .to_string();

                let v: Vec<&str> = content.splitn(2, "\n\n").collect();
                let (head, body);
                match v.len() {
                    1 => {
                        body = v[0].trim();
                        head = "".to_string();
                    }
                    _ => {
                        head = v[0].trim().to_string();
                        body = v[1].trim();
                    }
                }
                dbg!(&head);
                dbg!(&body);

                let body = {
                    let mut h = String::new();
                    html::push_html(&mut h, Parser::new(body));
                    h
                };

                let data = match toml::from_str::<HashMap<String, String>>(&head) {
                    Ok(mut s) => {
                        s.insert("body".into(), body);
                        s
                    }
                    Err(_) => {
                        let mut h = HashMap::new();
                        h.insert("body".to_string(), body);
                        h
                    }
                };

                let tpl =
                    template_cache
                        .entry(tpl_name.clone())
                        .or_insert_with(|| match read_to_string(&tpl_name) {
                            Ok(s) => Template::new(s)
                                .unwrap_or_else(|_| die!("this template is invalid")),
                            Err(e) => die!("failed to make template from file {}: {}", tpl_name, e),
                        });

                s.push_str(&tpl.render(&data));
            }
            s
        });

        if let Err(e) = write(path, processed.to_string()) {
            die!("failed to write to file {}, error: {}", path.display(), e)
        }
    }
    Ok(())
}
