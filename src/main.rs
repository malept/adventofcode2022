mod day1;
mod util;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let path = std::path::PathBuf::from(args[1].as_str()).canonicalize()?;
        let list = util::read_file(path)?;
        println!("Most calories: {}", day1::most_calories(list.as_str()));
    } else {
        println!("USAGE: {} [filename]", args[0]);
    }

    Ok(())
}
