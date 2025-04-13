use gbemulator::system::sm83::snapshot::SM83Snapshot;
use gbemulator::system::System;
use gbemulator::system::{ram::RAM, sm83::SM83};
use json::{self, JsonValue};
use std::{fs, io::Error};

fn read_u8_value(json_state: &JsonValue, key: &str) -> u8 {
    if json_state.has_key(key) {
        json_state[key].as_u8().unwrap()
    } else {
        0
    }
}

fn read_u16_value(json_state: &JsonValue, key: &str) -> u16 {
    if json_state.has_key(key) {
        json_state[key].as_u16().unwrap()
    } else {
        0
    }
}

fn read_bool_value(json_state: &JsonValue, key: &str) -> bool {
    if json_state.has_key(key) {
        json_state[key].as_u8().unwrap() > 0
    } else {
        false
    }
}

fn fill_ram(json_ram: &JsonValue, ram: &mut RAM) {
    for json_ram_item in json_ram.members() {
        let address = json_ram_item[0].as_u16().unwrap();
        let value = json_ram_item[1].as_u8().unwrap();
        ram.set_at(address, value);
    }
}

fn read_sm83_state(json_state: &JsonValue) -> (SM83Snapshot, RAM) {
    assert!(json_state.has_key("ram"));
    let snapshot = SM83Snapshot::new();
    let snapshot = snapshot
        .with_a(read_u8_value(json_state, "a"))
        .with_b(read_u8_value(json_state, "b"))
        .with_c(read_u8_value(json_state, "c"))
        .with_d(read_u8_value(json_state, "d"))
        .with_e(read_u8_value(json_state, "e"))
        .with_f(read_u8_value(json_state, "f"))
        .with_h(read_u8_value(json_state, "h"))
        .with_l(read_u8_value(json_state, "l"))
        //.with_ie(read_u8_value(json_state, "ie"))
        .with_ime(read_bool_value(json_state, "ime"))
        .with_pc(read_u16_value(json_state, "pc"))
        .with_sp(read_u16_value(json_state, "sp"));
    let mut ram = RAM::new();
    fill_ram(&json_state["ram"], &mut ram);
    return (snapshot, ram);
}

async fn read_test_case(json_content: &json::JsonValue) {
    assert!(json_content.has_key("name"));
    assert!(json_content.has_key("initial"));
    assert!(json_content.has_key("final"));
    assert!(json_content.has_key("cycles"));
    println!("{}", json_content["name"]);
    let initial_state = &json_content["initial"];
    let final_state = &json_content["final"];
    let (initial_snapshot, intial_ram) = read_sm83_state(initial_state);
    let mut simulator = System::from_ram_snapshot(1e3, intial_ram, initial_snapshot);
    let cycles: Vec<_> = json_content["cycles"].members().collect();
    //for cycle_json in cycles {
    simulator.next().await;
    //}
    let (final_snapshot, final_ram) = read_sm83_state(final_state);
    let new_pc = final_snapshot.pc + 1;
    let final_snapshot = final_snapshot.with_pc(new_pc);
    let result = simulator.to_snapshot().compare(&final_snapshot);
    println!("{:?}", result);
    if result.is_err() {
        panic!("{}", result.err().unwrap())
    }
}

#[tokio::main]
async fn main() {
    let jsons_path = "./sm83/v1/";
    let json_files_paths = fs::read_dir(jsons_path);
    match json_files_paths {
        Ok(paths) => {
            let paths: Vec<Result<fs::DirEntry, std::io::Error>> = paths.collect();
            let total_paths = paths.len();
            for (index, path) in paths.into_iter().enumerate() {
                if path.is_err() {
                    continue;
                }
                let path = path.unwrap().path();
                println!("{} {}/{}", path.display(), index, total_paths);
                let content = fs::read_to_string(path);
                if content.is_err() {
                    continue;
                }
                let content = content.unwrap();
                let json_content = json::parse(&content);
                if json_content.is_err() {
                    continue;
                }
                let tests_list = json_content.unwrap();
                assert!(tests_list.is_array());
                let tests_list = tests_list.members();
                for (test_index, test_case) in tests_list.into_iter().enumerate() {
                    println!("test case {}", test_index);
                    read_test_case(test_case).await;
                }
            }
        }
        Err(..) => panic!("unable to find files in path: {}", jsons_path),
    };
}
