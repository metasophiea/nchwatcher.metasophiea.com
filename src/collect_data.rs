use std::fs::{File, read_dir};
use std::io::prelude::*;

use super::{
    utils,
    Event,
    COLLECTED_DATA_DIRECTORY,
    MANIFEST_FILE_NAME
};

pub fn collect_data(now_date_string:&str) {
    println!("> collecting fresh data");

    println!(">> gathering all pages");
        let all_pages = utils::gather_all_pages();

    println!(">> extracting events");
        let mut events:Vec<Event> = all_pages.into_iter().flat_map(|page| Event::extract_events_from_html_document(&page)).collect();

    println!(">> correct event data");
        for event in &mut events {
            if let Some(url) = event.get_image_url() {
                if !url.starts_with("http") {
                    event.set_image_url(
                        format!("https://nch.ie{url}")
                    );
                }
            }
        }

    println!(">> saving data to file");
        let json_events:String = serde_json::to_string_pretty(&events).unwrap();
        let mut file = File::create(format!("{COLLECTED_DATA_DIRECTORY}/{now_date_string}")).unwrap();
        file.write_all(json_events.as_bytes()).unwrap();

    println!(">> updating manifest");
        let mut file_names:Vec<String> = read_dir( format!("{}/{COLLECTED_DATA_DIRECTORY}", env!("CARGO_MANIFEST_DIR")) )
            .unwrap()
            .map(|path| path.unwrap().path().file_name().unwrap().to_str().unwrap().to_string())
            .filter(|name| name != MANIFEST_FILE_NAME)
            .collect();

        file_names.sort();

        let json_events:String = serde_json::to_string_pretty(&file_names).unwrap();
        let mut file = File::create( format!("{}/{COLLECTED_DATA_DIRECTORY}/{MANIFEST_FILE_NAME}", env!("CARGO_MANIFEST_DIR")) ).unwrap();
        file.write_all(json_events.as_bytes()).unwrap();
}
