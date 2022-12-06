mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let path = std::path::PathBuf::from(args[1].as_str()).canonicalize()?;
        let data = util::read_file(path)?;
        println!(
            "Characters to process: {}",
            day6::find_start_of_packet_marker(&data).expect("Cannot find marker")
        );
    } else {
        println!("USAGE: {} [filename]", args[0]);
    }

    Ok(())
}
