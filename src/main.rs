use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn is_digit_str(contents: &String, i: usize, x: &str) -> bool {
    if (i + x.len()) >= contents.len() {
        return false;
    }
    let slice = &contents[i..(i + x.len())];
    return slice == x;
}

fn main() {
    //day_1().ok();
    //day_2().ok();
    day_2_part_2().ok();
}

fn day_2_part_2() -> std::io::Result<()> {
    let file_path = "2.txt";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut result = 0;

    let lines = contents.lines();

    for game_line in lines {
        let mut game_totals = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let parts: Vec<&str> = game_line.split(":").collect();
        let game_str = parts[0].replace("Game ", "");
        let game_int = game_str.parse::<i32>().unwrap();

        println!("Game: {}", game_int);

        let draws: Vec<&str> = parts[1].split(";").collect();

        for draw in draws {

            println!("  - Draw: {}", draw);

            let groups: Vec<&str> = draw.split(",").collect();

            for group in groups {
                let group_detail: Vec<&str> = group.trim().split(" ").collect();

                let group_number = group_detail[0].parse::<i32>().unwrap();
                let group_colour = group_detail[1];

                println!("      - Color: {}, Number: {}", group_colour, group_number);

                game_totals.entry(group_colour).and_modify(|e| *e = i32::max(*e, group_number));
            }
        }

        let game_total =
            game_totals.get("red").unwrap() *
                game_totals.get("green").unwrap() *
                game_totals.get("blue").unwrap();

        result += game_total;

        println!("    >>> Game: {}, Total: {}", game_int, game_total);
    }

    println!("Result: {}", result);

    Ok(())
}

fn day_2() -> std::io::Result<()> {
    let file_path = "2.txt";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut result = 0;

    let lines = contents.lines();

    let max_cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for game_line in lines {
        //println!("{}", line);

        let parts: Vec<&str> = game_line.split(":").collect();
        let game_str = parts[0].replace("Game ", "");
        let game_int = game_str.parse::<i32>().unwrap();

        println!("Game: {}", game_int);

        let draws: Vec<&str> = parts[1].split(";").collect();

        let mut is_valid_game = true;

        for draw in draws {

            let mut draw_totals = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ]);

            println!("  - Draw: {}", draw);

            let groups: Vec<&str> = draw.split(",").collect();

            for group in groups {
                //println!("    - Group: {}", group);

                let group_detail: Vec<&str> = group.trim().split(" ").collect();

                let group_number = group_detail[0].parse::<i32>().unwrap();
                let group_colour = group_detail[1];

                println!("      - Color: {}, Number: {}", group_colour, group_number);

                draw_totals.entry(group_colour).and_modify(|e| *e += group_number);
            }

            for game_total in draw_totals {
                let max_cube_colour = max_cubes.get(&game_total.0).unwrap();
                if &game_total.1 > max_cube_colour {
                    is_valid_game = false;
                    println!("    >>>>>>>>>>>>>> INVALID Game: {} - {} > {}", game_int, game_total.1, max_cube_colour);
                }
            }
        }

        if is_valid_game {
            println!("    >>> VALID Game: {}", game_int);
            result += game_int
        }
        else {
            println!("    >>> INVALID Game: {}", game_int);
        }
    }

    println!("Result: {}", result);

    Ok(())
}

fn day_1() -> std::io::Result<()> {
    let file_path = "1.txt";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total = 0;
    let mut first: i32= -1;
    let mut last: i32 = -1;

     for (i, chr) in contents.chars().enumerate() {
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
