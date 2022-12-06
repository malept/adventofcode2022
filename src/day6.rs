use std::collections::HashSet;

pub fn find_start_of_packet_marker(datastream: &str) -> Option<usize> {
    let count = datastream.len();
    let chars: Vec<char> = datastream.chars().collect();
    for (idx, c) in chars.iter().enumerate() {
        if idx < 3 {
            continue;
        }
        if idx + 3 >= count {
            break;
        }
        let unique: HashSet<char> = HashSet::from([
            c.clone(),
            chars[idx + 1].clone(),
            chars[idx + 2].clone(),
            chars[idx + 3].clone(),
        ]);
        if unique.len() == 4 {
            return Some(idx + 1);
        }
    }

    None
}

mod tests {
    const SIGNAL1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SIGNAL2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SIGNAL3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SIGNAL4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SIGNAL5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_find_start_of_packet_marker() {
        assert_eq!(super::find_start_of_packet_marker(SIGNAL1), Some(7));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL2), Some(5));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL3), Some(6));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL4), Some(10));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL5), Some(11));
    }
}
