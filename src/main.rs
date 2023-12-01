use std::fs::File;
use std::io::Read;

fn is_digit_str(contents: &String, i: usize, x: &str) -> bool {
    if (i + x.len()) >= contents.len() {
        return false;
    }
    let slice = &contents[i..(i + x.len())];
    return slice == x;
}

fn main() -> std::io::Result<()> {
    let file_path = "1.txt";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total = 0;
    let mut first: i32= -1;
    let mut last: i32 = -1;

    for (i, chr) in contents.chars().into_iter().enumerate() {
        let mut digit = -1;

        if is_digit_str(&contents, i, "nine") {
            digit = 9
        } else if is_digit_str(&contents, i, "eight") {
            digit = 8
        } else if is_digit_str(&contents, i, "seven") {
            digit = 7
        } else if is_digit_str(&contents, i, "six") {
            digit = 6
        } else if is_digit_str(&contents, i, "five") {
            digit = 5
        } else if is_digit_str(&contents, i, "four") {
            digit = 4
        } else if is_digit_str(&contents, i, "three") {
            digit = 3
        } else if is_digit_str(&contents, i, "two") {
            digit = 2
        } else if is_digit_str(&contents, i, "one") {
            digit = 1
        } else if chr.is_digit(10) {
            digit = chr.to_digit(10).unwrap() as i32;
        }

        if digit != -1 {
            if first == -1 {
                first = digit * 10;
            }
            last = digit;
        } else if chr.is_ascii_control() {
            if first != -1 && last != -1 {
                let row_total = first + last;
                total += row_total;
                first = -1;
                last = -1;
            }
        }
    }
    println!("Sum {}", total);
    Ok(())
}
