use std::collections::{HashMap, VecDeque};

pub fn run() {
    let input = super::get_input(6, "");
    // let input = super::get_input(6, "test");
    // let input = super::get_input(6, "testb");

    let orbiter_to_orbitee = {
        let mut tmp = HashMap::new();
        for o in input {
            let mut splitted = o.split(')');
            let orbitee = splitted.next().unwrap().to_string();
            let orbiter = splitted.next().unwrap().to_string();
            tmp.insert(orbiter, orbitee);
        }
        tmp
    };

    let total_orbits = {
        let mut tmp = 0;
        for orbiter in orbiter_to_orbitee.keys() {
            let mut current = Some(orbiter);
            while let Some(new_current) = orbiter_to_orbitee.get(current.unwrap()) {
                current = Some(new_current);
                tmp += 1;
            }
        }
        tmp
    };

    println!("a: {}", total_orbits);

    let mut path_you = {
        let mut tmp = VecDeque::new();
        let mut current = Some("YOU");
        while let Some(new_current) = orbiter_to_orbitee.get(current.unwrap()) {
            tmp.push_front(new_current.as_str());
            current = Some(new_current);
        }
        tmp
    };
    let mut path_san = {
        let mut tmp = VecDeque::new();
        let mut current = Some("SAN");
        while let Some(new_current) = orbiter_to_orbitee.get(current.unwrap()) {
            tmp.push_front(new_current.as_str());
            current = Some(new_current);
        }
        tmp
    };

    // println!("{:?}", path_you);
    // println!("{:?}", path_san);

    let path_len = 'lp: loop {
        if let Some(node1) = path_you.pop_front() {
            if let Some(node2) = path_san.pop_front() {
                if node1 != node2 {
                    break 'lp path_san.len() + path_you.len() + 2;
                }
            } else {
                break 'lp path_you.len() + 1;
            }
        } else {
            break 'lp path_san.len();
        }
    };

    println!("b: {}", path_len);
}
