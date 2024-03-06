use std::fs::{File, read_dir};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use time::OffsetDateTime;

mod constants;
mod event;
use event::Event;
mod utils;

const COLLECTED_DATA_DIRECTORY:&str = "web/collected_data";
const DIFF_DIRECTORY:&str = "web/diff";
const MANIFEST_FILE_NAME:&str = "manifest.json";

fn generate_diff(diff:(String, String)) {
    println!(">>> generating diff for {} > {}", diff.0, diff.1);

    //gather previous data
        let file_a = PathBuf::from(&format!("{}/{COLLECTED_DATA_DIRECTORY}/{}", env!("CARGO_MANIFEST_DIR"), diff.0));
        let previous_data_string = std::fs::read_to_string(file_a).unwrap();
        let previous_events = serde_json::from_str::<Vec<Event>>(&previous_data_string).unwrap();

    //gather next data
        let file_b = PathBuf::from(format!("{}/{COLLECTED_DATA_DIRECTORY}/{}", env!("CARGO_MANIFEST_DIR"), diff.1));
        let next_data_string = std::fs::read_to_string(file_b).unwrap();
        let next_events = serde_json::from_str::<Vec<Event>>(&next_data_string).unwrap();

    //compare
        //go through the new list, and if anything appears on the old list remove it
        let mut events:Vec<Event> = next_events.into_iter().filter(|event| !previous_events.iter().any(|previous_event| event == previous_event)).collect();

    //sort
        events.sort_by(|a, b| a.get_time().partial_cmp(b.get_time()).unwrap());
        events.sort_by(|a, b| a.get_date().partial_cmp(b.get_date()).unwrap());

    //print
        let output = PathBuf::from(format!("{}/{DIFF_DIRECTORY}/{}>{}", env!("CARGO_MANIFEST_DIR"), diff.0, diff.1));
        let json_events:String = serde_json::to_string_pretty(&events).unwrap();
        let mut file = File::create(output).unwrap();
        file.write_all(json_events.as_bytes()).unwrap();
}

fn collect_data(now_date_string:&str) {
    println!("> collecting fresh data");

    println!(">> gathering all pages");
        let all_pages = utils::gather_all_pages();

    println!(">> extracting events");
        let mut events:Vec<Event> = all_pages.into_iter().flat_map(|page| Event::extract_events_from_html_document(&page)).collect();

    println!(">> correct event data");
        events.iter_mut().for_each(|event| {
            if let Some(url) = event.get_image_url() {
                if !url.starts_with("http") {
                    event.set_image_url(
                        format!("https://nch.ie{url}")
                    );
                }
            }
        });

    println!(">> saving data to file");
        let json_events:String = serde_json::to_string_pretty(&events).unwrap();
        let mut file = File::create(format!("{COLLECTED_DATA_DIRECTORY}/{now_date_string}")).unwrap();
        file.write_all(json_events.as_bytes()).unwrap();

    println!(">> updating manifest");
        let mut file_names:Vec<String> = read_dir( &format!("{}/{COLLECTED_DATA_DIRECTORY}", env!("CARGO_MANIFEST_DIR")) )
            .unwrap()
            .into_iter()
            .map(|path| path.unwrap().path().file_name().unwrap().to_str().unwrap().to_string())
            .filter(|name| name != MANIFEST_FILE_NAME)
            .collect();

        file_names.sort();

        let json_events:String = serde_json::to_string_pretty(&file_names).unwrap();
        let mut file = File::create(&format!("{}/{COLLECTED_DATA_DIRECTORY}/{MANIFEST_FILE_NAME}", env!("CARGO_MANIFEST_DIR"))).unwrap();
        file.write_all(json_events.as_bytes()).unwrap();
}

fn generate_missing_diffs() {
    println!("> generating missing diffs");

    println!(">> get list of all data files");
        let text = std::fs::read_to_string(&format!("{}/{COLLECTED_DATA_DIRECTORY}/{MANIFEST_FILE_NAME}", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let file_names = serde_json::from_str::<Vec<String>>(&text).unwrap();

    println!(">> create list of required diffs");
        let mut required_diffs:Vec<(String, String)> = vec![];
        for index in 0..file_names.len() - 1 {
            required_diffs.push((
                file_names[index].clone(),
                file_names[index+1].clone()
            ));
        }

    println!(">> compare this list to the list of generated diffs");
        let text = std::fs::read_to_string(&format!("{}/{DIFF_DIRECTORY}/{MANIFEST_FILE_NAME}", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let diff_list = serde_json::from_str::<Vec<String>>(&text).unwrap();
        let ungenerated_diffs:Vec<(String, String)> = required_diffs.into_iter().filter(|diff| {
            !diff_list.contains(
                &format!("{}>{}", diff.0, diff.1)
            )
        })
        .collect();

    println!(">> generate missing diffs");
        if ungenerated_diffs.is_empty() { println!(">>> none to generate"); }
        ungenerated_diffs.into_iter().for_each(generate_diff);

    println!(">> updating manifest");
        let mut file_names:Vec<String> = read_dir( &format!("{}/{DIFF_DIRECTORY}", env!("CARGO_MANIFEST_DIR")) )
            .unwrap()
            .into_iter()
            .map(|path| path.unwrap().path().file_name().unwrap().to_str().unwrap().to_string())
            .filter(|name| name != MANIFEST_FILE_NAME)
            .collect();

        file_names.sort();

        let json_events:String = serde_json::to_string_pretty(&file_names).unwrap();
        let mut file = File::create(&format!("{}/{DIFF_DIRECTORY}/{MANIFEST_FILE_NAME}", env!("CARGO_MANIFEST_DIR"))).unwrap();
        file.write_all(json_events.as_bytes()).unwrap();
}

fn main() {
    //collect new data if necessary
        let now = OffsetDateTime::now_utc();
        let now_date_string = now.date().to_string();

        if !Path::new(&format!("{COLLECTED_DATA_DIRECTORY}/{now_date_string}")).exists() {
            collect_data(&now_date_string);
        } else {
            println!("> data already collected for today");
        }

    //ensure presence of all diffs
        generate_missing_diffs();
}