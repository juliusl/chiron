pub mod config;
pub mod metadata;

use serde::{Deserialize};
use rocksdb::{IteratorMode, Options, DB, ColumnFamily, DBWithThreadMode, SingleThreaded};
use std::{str, ops::Deref};

/// Locates all objects in the column family: `object_name`, stored in the db at `path`
pub fn get_db_objects<O>(path: &str, object_name: &str) -> Option<Vec<O>>
where 
    for<'a> O: Deserialize<'a> + Default {
    let db_opts = Options::default();
    let cfs = DB::list_cf(&db_opts, &path).unwrap();

    let cf_opts = Options::default();
    {
        let cf = DB::open_cf_for_read_only(&cf_opts, &path, &cfs, false);

        match cf {
            Ok(opened) => {
                let mut response = vec![];
                let handle = opened.cf_handle(object_name).unwrap();

                opened.iterator_cf(handle, IteratorMode::Start).for_each(|(_, v)| {

                    let value = serde_json::from_slice(&v);

                    let value: O = match value {
                        Ok(v) => v, 
                        Err(err) => {
                            let raw_json = str::from_utf8(&v).unwrap_or_default(); 
                            let property = raw_json[..err.column()]
                                .split("\"")
                                .collect::<Vec<&str>>().iter()
                                    .rev()
                                    .skip(1)
                                    .next()
                                    .unwrap()
                                    .deref()
                                    .to_owned();

                            eprintln!("{}\nproperty_key: {}", err, property);
                            O::default()
                        }
                    };

                    response.push(value);
                });

                return Some(response)
            },
            Err(_) => None 
        }
    }
}

fn scan(opened: &DBWithThreadMode<SingleThreaded>, handle: &ColumnFamily) {
    opened.iterator_cf(handle, IteratorMode::Start).for_each(|(_, v)| {

        println!("{:?}", str::from_utf8(&v));

    });
}