mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let path = std::path::PathBuf::from(args[1].as_str()).canonicalize()?;
        let data = util::read_file(path)?;
        println!(
            "Overlapping pairs count: {}",
            day5::top_stacked_crates(data)
        );
    } else {
        println!("USAGE: {} [filename]", args[0]);
    }

    Ok(())
}
