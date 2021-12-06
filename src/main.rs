use std::fs;
use std::{num::ParseIntError};

// ------------------------------------------ Structures -------------------------------------------
struct Submarine {
    horizontal_pos: i32,
    depth_pos: i32,
    aim: i32,
}

// ------------------------------------- Utility Functionality -------------------------------------
// I'm jacking this from https://stackoverflow.com/questions/63876919/rust-convert-from-a-binary-string-representation-to-ascii-string
// pub fn decode_binary(s: Vec<&str>) -> Result<Vec<u8>, ParseIntError> {
//     return s.iter().map(|i| u8::from_str_radix(&s[i..i + 8], 2)).collect();
// }


fn parse_file_input_to_numeric_array(filename: &str) -> Vec<i32> {
    let data = fs::read_to_string(filename).expect("Unable to read file");
    // Iterate over the string, it should be delimited by new lines.
    //
    let string_array = data.split("\n").collect::<Vec<&str>>();
    // let array_size = string_array.len();
    let y: Result<Vec<i32>, _> = string_array.iter().map(|&x| x.parse::<i32>()).collect();
    let y = match y {
        Ok(y) => y,
        Err(_err) => {
            panic!("Ah fuck!")
        }
    };
    return y;
}


// ------------------------------------- Day by Day Functions --------------------------------------
fn day_one_part_one_solution() {
    let sonar_input = parse_file_input_to_numeric_array("src/day1input.txt");
    let mut counter: u32 = 0;
    for item in sonar_input.iter().enumerate() {
        let (i, _x): (usize, &i32) = item;
        if i == 0 {
            continue;
        }
        if sonar_input[i - 1] < sonar_input[i] {
            counter = counter + 1;
        }
    }
    println!("{}", counter);
}

fn calculate_sliding_window(index: usize, input_values: &Vec<i32>, window_size: usize) -> i32 {
    // We assume that we've caught an illegal index value at a higher level.
    // We simply get the next values in the vector for the size of the window, sum them, and
    // return the value.
    let mut result: i32 = 0;
    for i in index..index + window_size {
        result += input_values[i];
    }
    return result;
}

fn day_one_part_two_solution() {
    let sonar_input = parse_file_input_to_numeric_array("src/day1input.txt");
    // let sonar_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let mut counter: u32 = 0;
    for item in sonar_input.iter().enumerate() {
        let (i, _x): (usize, &i32) = item;
        if i == 0 {
            continue;
        }
        if i + 2 >= sonar_input.len() {
            continue;
        }
        let prior_window = calculate_sliding_window(i - 1, &sonar_input, 3);
        let current_window = calculate_sliding_window(i, &sonar_input, 3);
        // println!("Prior: {} Current: {}", prior_window, current_window);
        if current_window > prior_window {
            counter = counter + 1;
        }
    }
    println!("{}", counter);
}

fn day_two_part_one_solution() {
    let data = fs::read_to_string("src/day2.txt").expect("Unable to read file");
    let test_input = data.split("\n").collect::<Vec<&str>>();
    let mut sub = Submarine {
        horizontal_pos: 0,
        depth_pos: 0,
        aim: 0,
    };
    for command in test_input.iter(){
        let splt_cmd: Vec<&str> = command.split_whitespace().collect();
        // So we should have two elements here. If we don't, we throw a fit.
        if splt_cmd.len() != 2{
            panic!("Ah fuck!")
        }
        let command_type = splt_cmd[0];
        let command_value = splt_cmd[1].parse::<i32>().unwrap();
        match command_type {
            "forward" => sub.horizontal_pos = sub.horizontal_pos + command_value,
            "down" => sub.depth_pos = sub.depth_pos + command_value,
            "up" => sub.depth_pos = sub.depth_pos - command_value,
            _ => println!("Jeepers"),
        }
    }
    println!("Horizontal Pos: {}, Depth Pos: {}", sub.horizontal_pos, sub.depth_pos);
    println!("Multiplied Solution: {}", sub.depth_pos * sub.horizontal_pos);
}

fn day_two_part_two_solution() {
    let data = fs::read_to_string("src/day2.txt").expect("Unable to read file");
    let test_input = data.split("\n").collect::<Vec<&str>>();
    // let test_input = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
    let mut sub = Submarine {
        horizontal_pos: 0,
        depth_pos: 0,
        aim: 0,
    };
    for command in test_input.iter(){
        let splt_cmd: Vec<&str> = command.split_whitespace().collect();
        // So we should have two elements here. If we don't, we throw a fit.
        if splt_cmd.len() != 2{
            panic!("Ah fuck!")
        }
        let command_type = splt_cmd[0];
        let command_value = splt_cmd[1].parse::<i32>().unwrap();
        match command_type {
            "forward" => {
                sub.horizontal_pos = sub.horizontal_pos + command_value;
                sub.depth_pos = sub.depth_pos + (sub.aim * command_value);
            },
            "down" => sub.aim = sub.aim + command_value,
            "up" => sub.aim = sub.aim - command_value,
            _ => println!("Jeepers"),
        }
        println!("Horizontal Pos: {}, Aim: {}, Depth: {}", sub.horizontal_pos, sub.aim, sub.depth_pos);
    }
    println!("Horizontal Pos: {}, Depth Pos: {}", sub.horizontal_pos, sub.depth_pos);
    println!("Multiplied Solution: {}", sub.depth_pos * sub.horizontal_pos);
}

fn day_three_part_one_solution(){
    let mut test_input = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];
    // We take the corresponding bit fields and find the most common number in each of the positions
    // to calculate the Gamma.
    let bitsize = test_input[0].len();
    let totality = test_input.len();
    let mut result_string: String = "".to_owned();
    for i in 0..bitsize {
        let mut bitresult = 0;
        for j in 0..totality-1 {
            println!("Index Value into Strin: {}", j);
            println!("Count Value of Input String: {}", test_input[i].chars().count());
            let thing = test_input[i].chars().nth(j).unwrap();
            println!("{}",  test_input[i].chars().nth(j).unwrap());
            // bitresult = bitresult + test_input[i].chars().nth(j).unwrap().to_digit(10).unwrap()
        }
        // if bitresult > (totality - 1).try_into().unwrap(){
        //     result_string.push_str("1")
        // }
        // else {
        //     result_string.push_str("0")
        // }
    }
    // println!("{}", result_string);
    // A couple wrinkles here.
    // Those are 5bit numbers. So, we could always pad them with zeros, but once you do the not to
    // those fields you're kind of fucked.
    // However, you could always do a leftwise bitshift after the conversion to drop the lefthand
    // numbers off of the string and then do a righwise bitshift to "correct", essentially.
    // for input in test_input.iter().enumerate(){
    //     let (i, x): (usize, &&str) = item;
    //     test_input[i] = &*format!("{:0>8}", x)
    // }
    // let binary_numbers = decode_binary(test_input).unwrap();
    //

}


fn main() {
    // day_one_solution()
    // day_one_part_two_solution()
    // day_two_part_one_solution()
    // day_two_part_two_solution();
    day_three_part_one_solution();
}
