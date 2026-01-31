use anyhow::{anyhow, Context, Result};
use polars::prelude::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read(path: &Path) -> Result<DataFrame> {
    let file = File::open(path).context("Failed to open XML file")?;
    let reader = BufReader::new(file);

    let records = parse_xml(reader)?;

    if records.is_empty() {
        return Err(anyhow!("No records found in XML"));
    }

    records_to_dataframe(records)
}

fn parse_xml(reader: BufReader<File>) -> Result<Vec<HashMap<String, String>>> {
    let mut xml_reader = Reader::from_reader(reader);
    xml_reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut records = Vec::new();
    let mut current_record: HashMap<String, String> = HashMap::new();
    let mut current_element: Option<String> = None;
    let mut record_tag: Option<String> = None;
    let mut depth = 0;

    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                depth += 1;
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match depth {
                    2 => {
                        if record_tag.is_none() {
                            record_tag = Some(tag_name.clone());
                        }
                        if Some(&tag_name) == record_tag.as_ref() {
                            current_record = HashMap::new();
                        }
                    }
                    3 => current_element = Some(tag_name),
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if let Some(ref elem) = current_element {
                    let text = e.unescape().unwrap_or_default().to_string();
                    if !text.is_empty() {
                        current_record.insert(elem.clone(), text);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                if depth == 2
                    && Some(&tag_name) == record_tag.as_ref()
                    && !current_record.is_empty()
                {
                    records.push(current_record.clone());
                }
                if depth == 3 {
                    current_element = None;
                }
                depth -= 1;
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(anyhow!(
                    "Failed to read XML at position {}: {:?}",
                    xml_reader.error_position(),
                    e
                ))
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(records)
}

fn records_to_dataframe(records: Vec<HashMap<String, String>>) -> Result<DataFrame> {
    let columns = collect_columns(&records);

    let column_vec: Vec<Column> = columns
        .iter()
        .map(|col| {
            let values: Vec<Option<String>> = records
                .iter()
                .map(|record| record.get(col).cloned())
                .collect();

            Column::new(col.as_str().into(), values)
        })
        .collect();

    DataFrame::new(column_vec).context("Failed to create DataFrame from XML")
}

fn collect_columns(records: &[HashMap<String, String>]) -> Vec<String> {
    let mut columns = Vec::new();

    for record in records {
        for key in record.keys() {
            if !columns.contains(key) {
                columns.push(key.clone());
            }
        }
    }

    columns
}
