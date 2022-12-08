#![feature(option_result_contains)]

use scraper::{Html, Selector};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn    let arg    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            r#"You must provide a search query, e.g. "php_1337x_scraper [query] [destination folder]""#
        );
        std::process::exit(0);
    }

    let query = &args[1];
    let dest_dir = &args[2];

    let cachestring = dest_dir.to_string();
    let cachedir = Path::new(&cachestring);
    let filename = format!(r"{}\1337x_Cache.json", cachedir.display());
    let filepath = Path::new(&filename);

    if filepath.exists() {
        println!(r"Clearing previous cache.");
        fs::remove_file(filepath).expect("Couldn't delete config file for re-caching");
    }

    fs::create_dir_all(cachedir).expect("Couldn't create config directory");
    File::create(filepath).expect("Couldn't create config file");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(filepath)
        .expect("Opening file failed");

    file.write_all(
        br#"{
    "response": [
"#,
    )
    .expect("Couldn't write bytes to file");

    let url = format!("https://1337x.to/category-search/{}/Games/1/", query);

    let body = reqwest::blocking::get(url)
        .expect("GET Request failed.")
        .text()
        .expect("Couldn't output HTML body as text.");

    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"tbody > tr > td > a"#).expect("Couldn't parse list");

    let mut results: Vec<String> = vec![];

    for title in document.select(&selector) {
        if title.html().contains("/torrent/") && results.len() < 6 {
            results.push(format!(
                "https://1337x.to{}",
                title.value().attr("href").unwrap()
            ));
        }
    }

    let mut count = 0;

    for entry in results {
        if count < 5 {
            scan_page(entry, cachedir, 0);
        } else {
            scan_page(entry, cachedir, 1);
        }

        count += 1;
    }

    file.write_all(
        br#"    ]
}"#,
    )
    .expect("Couldn't write bytes to file");
}

fn scan_page(url: String, dest_dir: &Path, loopcount: u8) {
    let body = reqwest::blocking::get(url)
        .expect("GET Request failed.")
        .text()
        .expect("Couldn't output HTML body as text.");
    let document = Html::parse_document(&body);
    let title_selector = Selector::parse(r#"h1"#).expect("Couldn't parse title");
    let magnet_selector = Selector::parse(r#"ul > li > a"#).expect("Couldn't parse magnets");

    let mut titles: Vec<String> = vec![];
    let mut magnets: Vec<String> = vec![];

    for title in document.select(&title_selector) {
        titles.push(title.inner_html());
    }

    for magnet in document.select(&magnet_selector) {
        if magnet.inner_html().contains("Magnet Download") {
            magnets.push(
                magnet
                    .value()
                    .attr("href")
                    .expect("Couldn't push magnets to vector")
                    .to_string(),
            )
        }
    }

    let title = &titles[0];
    let magnet = &magnets[0];

    let jsoncontent = format!(
        r#"        {{ "Title": "{}", "URL1": ["{}"], "URL2": [], "URL3": [], "URL4": [] }},
"#,
        title, magnet
    );
    let last_jsoncontent = format!(
        r#"        {{ "Title": "{}", "URL1": ["{}"], "URL2": [], "URL3": [], "URL4": [] }}
"#,
        title, magnet
    );

    println!("Caching: {}", title);
    if loopcount == 0 {
        write_to_json(dest_dir, jsoncontent);
    } else {
        write_to_json(dest_dir, last_jsoncontent);
    }
}

fn write_to_json(dest_dir: &Path, jsoncontent: String) {
    );

   let file_path = Path::new(&file_string);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .expect("Opening file failed");

    file.write_all(jsoncontent.as_bytes())
        .expect("Couldn't write bytes to file");
}
ile_path)
        .expect("Opening file failed");

    file.write_all(jsoncontent.as_bytes())
        .expect("Couldn't write bytes to file");
}

