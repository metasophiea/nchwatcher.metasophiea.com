use std::path::Path;

use time::OffsetDateTime;

mod collect_data;
use collect_data::collect_data;
mod constants;
mod event;
use event::Event;
mod generate_missing_diffs;
use generate_missing_diffs::generate_missing_diffs;
mod utils;

const COLLECTED_DATA_DIRECTORY:&str = "web/collected_data";
const DIFF_DIRECTORY:&str = "web/diff";
const MANIFEST_FILE_NAME:&str = "manifest.json";

fn main() {
    //collect new data if necessary
        let now = OffsetDateTime::now_utc();
        let now_date_string = now.date().to_string();

        if Path::new(&format!("{COLLECTED_DATA_DIRECTORY}/{now_date_string}")).exists() {
            println!("> data already collected for today");
        } else {
            collect_data(&now_date_string);
        }

    //ensure presence of all diffs
        generate_missing_diffs();
}