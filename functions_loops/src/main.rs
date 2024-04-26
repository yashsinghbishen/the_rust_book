fn main() {
    println!("Varibale with conditions");
    run_loop(1,1,"loop");
    println!("Loop");
    run_loop(1,5,"loop");
    println!("While");
    run_loop(1,5,"while");
    println!("For");
    run_loop(1,5,"for");
    println!("Reverse For");
    run_loop(1,5,"for_rev");
}

fn run_loop(mut start:u32, mut end:u32, loop_type:&str) -> &str {
    // If condition for assignments
    end = if end > start {end} else {start + 1};

    if loop_type == "loop" {
        // loop
        loop{
            if start == end {
                break;
            }
            println!("{start}");
            start+=1;
        }
    } else if loop_type == "while" {
        // while loop
        while start < end {
            println!("{start}");
            start+=1;
        }
    } else if loop_type == "for" {
        // for loop
        for i in start .. end {
            println!("{i}");
        }
    }else if loop_type == "for_rev" {
        // for loop
        for i in (start .. end).rev() {
            println!("{i}");
        }
    }
    loop_type
}