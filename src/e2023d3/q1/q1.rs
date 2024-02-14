use std::env;
use std::path::Path;
use std::fs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn ascii_is_num(c: u8) -> bool {
    if (c >= 48) && (c <= 57) {
        return true;
    }
    return false;
}

fn ascii_is_alpha(c: u8) -> bool {
    if (c >= 65 && c <= 90) || (c >= 97 && c <= 122) {
        return true;
    }
    return false;
}

fn ascii_is_special(c: u8) -> bool {
    if ascii_is_num(c) {
        return false;
    } else {
        if c == 46 {
            return false;
        } else {
            return true;
        }
    }
}

fn main () {
    let mut verbose: bool = false;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        // Check if the user is asking for help
        if args[1].contains("help") || args[1].contains("-h") {
            println!("Usage: q1.exe <path_to_input_text_file> [verbose]");
            println!("Example: q1.exe input.txt verbose");
            println!("Example: q1.exe input.txt -v");
            println!("Example: q1.exe help");
            println!("Example: q1.exe -h");
            return;
        }
        println!("Processing file: {}", args[1]);
        // Check if the user provided a text file
        if args[1].contains(".txt") {
            println!("It's a text file!");
        } else {
            println!("Please provide a text file with .txt filetype!");
            return;
        }
        // Check if the file exists
        if !Path::new(&args[1]).exists() {
            println!("File does not exist!");
            return;
        } else {
            println!("File exists!");
        }
        // Check verbose mode
        if args.len() > 2 {
            if args[2].contains("verbose") || args[2].contains("-v") {
                println!("Verbose mode is on!");
                verbose = true;
            } else {
                println!("Verbose mode is off!");
                verbose = false;
            }
        }
    }

    println!("Reading file ...");
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    if verbose { println!("With text:\n{}", contents); }

    println!("Splitting file ...");
    let parts = contents.lines().collect::<Vec<_>>();
    let mut line_info = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let line_count = i + 1;
        let char_count = part.chars().count();
        line_info.push((line_count, char_count));
        if verbose { println!("Line {}: {} characters", line_count, char_count); }
    }
    // Accessing the first character of the first line
    //println!("{}", parts[0]);
    // Accessing type of the first line
    //print_type_of(&parts[0]);
    // Accessing the first character of the first line as a char
    //println!("{}", parts[0].as_bytes()[0] as char);
    // Accessing the first character of the first line as a byte (ASCII value)
    //println!("{}", parts[0].as_bytes()[0]);
    // Accessing whatever in line_info
    //println!("{:?}", line_info);
    // Accessing first element of vec and its first element (vec = [(x)])
    //println!("{}", line_info[0].0);
    // Accessing the length of vec
    //println!("{}", line_info.len());

    let max_line_cnt = line_info.len();
    let max_char_cnt = line_info[max_line_cnt - 1].1;
    println!("(Max line count, Max char count): ({}, {})", max_line_cnt, max_char_cnt);

    println!("Converting to ascii 2D array ...");
    let mut ascii_arr = vec![vec![0; max_line_cnt]; max_char_cnt];
    for (i, part) in parts.iter().enumerate() {
        for (j, c) in part.chars().enumerate() {
            ascii_arr[i][j] = c as u8;
        }
        if verbose { println!("{:?}", ascii_arr[i]); }
    }

    println!("Solving first question ...");
    let mut num_count = 0;
    let mut valid_num_count = 0;
    let mut num_flag: bool = false;
    let mut break_flag: bool = false;
    let mut num_tmp: Vec<char> = Vec::new();
    let mut num_tmp_len = 0;
    let mut num_tmp_str: String = String::new();
    let mut valid_num: Vec<String> = Vec::new();
    let mut invalid_num: Vec<String> = Vec::new();
    let mut all_num: Vec<String> = Vec::new();
    let mut search_x_index: Vec<usize> = Vec::new();
    let mut search_y_index: Vec<usize> = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        for (j, _) in part.chars().enumerate() {
            if ascii_is_num(ascii_arr[i][j]) {
                num_flag = true;
                if verbose { println!("Found number at idx (line, column) = ({}, {})", i, j); }
                num_tmp.push(ascii_arr[i][j] as char);
            } else {
                num_flag = false;
            }
            num_tmp_len = num_tmp.len();
            if num_flag == false && num_tmp_len > 0 {
                num_count += 1;
                if verbose { println!("Number in Vec: {:?}", num_tmp); }
                for k in 0..num_tmp_len {
                    num_tmp_str.push(num_tmp[k]);
                }
                if verbose { println!("Number in String: {}", num_tmp_str); }
                // Search for special characters around the number
                // lower limit is 0, upper limit is max_char_cnt - 1
                if verbose { println!("Processing idx (i, j) = ({}, {})", i, j); }
                if j <= 0 + num_tmp_len {
                    if verbose { println!("case 1"); }
                    if verbose { println!("min = {}", 0); }
                    if verbose { println!("max = {}", j); }
                    for l in 0..(j+1) {
                        search_x_index.push(l);
                    }
                } else if j >= (max_char_cnt - 1) {
                    if verbose { println!("case 2"); }
                    if verbose { println!("min = {}", j-num_tmp_len-1); }
                    if verbose { println!("max = {}", j); }
                    for l in ((j-num_tmp_len-1)..(j+1)).rev() {
                        search_x_index.push(l);
                    }
                } else {
                    if verbose { println!("case 3"); }
                    if verbose { println!("min = {}", j-num_tmp_len-1); }
                    if verbose { println!("max = {}", j); }
                    for l in ((j-num_tmp_len-1)..(j+1)).rev() {
                        search_x_index.push(l);
                    }
                }
                if verbose { println!("X index to search: {:?}", search_x_index); }
                // lower limit is 0, upper limit is max_line_cnt - 1
                if i == 0 {
                    search_y_index.push(i);
                    search_y_index.push(i+1);
                } else if i == (max_line_cnt - 1) {
                    search_y_index.push(i-1);
                    search_y_index.push(i);
                } else {
                    search_y_index.push(i-1);
                    search_y_index.push(i);
                    search_y_index.push(i+1);
                }
                if verbose { println!("Y index to search: {:?}", search_y_index); }

                for (m, _) in search_y_index.iter().enumerate() {
                    for (n, _) in search_x_index.iter().enumerate() {
                        if ascii_is_special(ascii_arr[search_y_index[m]][search_x_index[n]]) {
                            if verbose { println!("Special character found at idx (line, column) = ({}, {})", search_x_index[n], search_y_index[m]); }
                            valid_num_count += 1;
                            valid_num.push(num_tmp_str.clone());
                            if verbose { println!("VALID\n"); }
                            break_flag = true;
                            break;
                        }
                    }
                }
                if break_flag == false {
                    invalid_num.push(num_tmp_str.clone());
                    if verbose { println!("INVALID\n"); }
                }
                all_num.push(num_tmp_str.clone());
                num_tmp.clear();
                num_tmp_str.clear();
                search_x_index.clear();
                search_y_index.clear();
                break_flag = false;
            }
        }
    }
    println!("Valid/Invalid/Total number count: {}/{}/{}", valid_num_count, num_count - valid_num_count, num_count);
    println!("Valid numbers: {:?}", valid_num);
    println!("Invalid numbers: {:?}", invalid_num);
    println!("All numbers: {:?}", all_num);

    println!("Summing up valid numbers ...");
    let mut sum: u32 = 0;
    for (i, _) in valid_num.iter().enumerate() {
        sum += valid_num[i].parse::<u32>().unwrap();
    }
    println!("Sum of valid numbers: {}", sum);
}
