mod day1;
mod day2;
mod day3;
mod day4;
mod util;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let path = std::path::PathBuf::from(args[1].as_str()).canonicalize()?;
        let lines = util::lines_for_file(path)?;
        println!(
            "Pairs subset count: {}",
            day4::full_subset_assignment_pairs_count(&lines)
        );
    } else {
        println!("USAGE: {} [filename]", args[0]);
    }

    Ok(())
}
