mod day1;
mod day2;
mod day3;
mod util;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let path = std::path::PathBuf::from(args[1].as_str()).canonicalize()?;
        let lines = util::lines_for_file(path)?;
        println!(
            "Duplicate priority sum: {}",
            day3::duplicate_priority_sum(lines)
        );
    } else {
        println!("USAGE: {} [filename]", args[0]);
    }

    Ok(())
}
