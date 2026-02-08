use gbemulator::system::ram::RAM;
use gbemulator::system::sm83::snapshot::SM83Snapshot;
use gbemulator::system::System;
use json::{self, JsonValue};
use std::fs;

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

fn check_ram(json_ram: &JsonValue, ram: &RAM) -> Result<(), String> {
    let mut result = String::new();
    for json_ram_item in json_ram.members() {
        let address = json_ram_item[0].as_u16().unwrap();
        let value = json_ram_item[1].as_u8().unwrap();
        let actual_value = ram.get_at(address).unwrap().to_owned();
        if value != actual_value {
            result += format!(
                "Address {} / 0x{:X}: value {} / 0x{:x}, expected value {} / 0x{:x}",
                address, address, actual_value, actual_value, value, value
            )
            .as_str();
        }
    }
    if result.is_empty() {
        Ok(())
    } else {
        Err(result)
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
    let mut ram = RAM::new(None);
    fill_ram(&json_state["ram"], &mut ram);
    return (snapshot, ram);
}

fn read_test_case(json_content: &json::JsonValue) -> Result<(), ()> {
    assert!(json_content.has_key("name"));
    assert!(json_content.has_key("initial"));
    assert!(json_content.has_key("final"));
    assert!(json_content.has_key("cycles"));
    println!("{}", json_content["name"]);
    if
    /*json_content["name"].as_str().unwrap().contains("27 ") ||*/
    json_content["name"].as_str().unwrap().contains("FB ") {
        return Ok(());
    }
    let initial_state = &json_content["initial"];
    let final_state = &json_content["final"];
    let (initial_snapshot, intial_ram) = read_sm83_state(initial_state);
    let mut simulator = System::from_ram_snapshot(intial_ram, initial_snapshot, true);
    //for cycle_json in cycles {
    simulator.next();
    //}
    let (mut final_snapshot, _) = read_sm83_state(final_state);
    if !json_content["name"].as_str().unwrap().contains("76 ")
        || json_content["name"].as_str().unwrap().contains("CB ")
    {
        let new_pc = if final_snapshot.pc < u16::MAX {
            final_snapshot.pc + 1
        } else {
            0
        };
        final_snapshot = final_snapshot.with_pc(new_pc);
    }
    let result = simulator.to_snapshot().compare(&final_snapshot);
    if result.is_err() {
        //println!("{}", result.err().unwrap());
        panic!("{}", result.err().unwrap());
        return Err(());
    }
    let ram_result = check_ram(&final_state["ram"], &simulator.get_ram());
    if ram_result.is_err() {
        //println!("{}", ram_result.err().unwrap());
        panic!("{}", ram_result.err().unwrap());
        return Err(());
    }
    Ok(())
}
fn main() {
    let jsons_path = "./sm83/v1/";
    let succesful_tests_path = "./sm83/succesful_tests.txt";
    let mut succesful_tests_file_contents = fs::read_to_string(succesful_tests_path).unwrap();
    // let mut succesful_tests_names: Vec<&str> = Vec::<&str>::new();
    // for line in succesfult_tests_file.split("\n") {
    //     succesful_tests_names.push(line);
    // }
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
                if succesful_tests_file_contents.contains(path.to_str().unwrap()) {
                    continue;
                }
                println!("{} {}/{}", path.display(), index, total_paths);
                let content = fs::read_to_string(path.clone());
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
                let mut total = 0;
                let mut total_passed = 0;
                for (test_index, test_case) in tests_list.into_iter().enumerate() {
                    println!("test case {}", test_index);
                    let res = read_test_case(test_case);
                    total += 1;
                    if res.is_ok() {
                        total_passed += 1;
                    }
                }
                println!("{total_passed}/{total}");
                if total == total_passed {
                    succesful_tests_file_contents += path.to_str().unwrap();
                    succesful_tests_file_contents += "\n";
                    fs::write(succesful_tests_path, succesful_tests_file_contents.clone()).unwrap();
                }
            }
        }
        Err(..) => panic!("unable to find files in path: {}", jsons_path),
    };
}
