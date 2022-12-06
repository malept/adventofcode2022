use std::collections::HashSet;

const MARKER_LENGTH: usize = 14;

pub fn find_start_of_packet_marker(datastream: &str) -> Option<usize> {
    let count = datastream.len();
    let chars: Vec<char> = datastream.chars().collect();
    for (idx, c) in chars.iter().enumerate() {
        if idx + MARKER_LENGTH - 1 >= count {
            break;
        }
        let mut unique: HashSet<char> = HashSet::with_capacity(MARKER_LENGTH);
        unique.insert(c.clone());
        for n in 1..MARKER_LENGTH {
            unique.insert(chars[idx + n].clone());
        }
        if unique.len() == MARKER_LENGTH {
            return Some(idx + MARKER_LENGTH);
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
        assert_eq!(super::find_start_of_packet_marker(SIGNAL1), Some(19));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL2), Some(23));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL3), Some(23));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL4), Some(29));
        assert_eq!(super::find_start_of_packet_marker(SIGNAL5), Some(26));
    }
}
