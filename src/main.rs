use std::io::BufRead;

fn read_test_data<P: AsRef<std::path::Path>>(path: P) -> Vec<String> {
    let data_file = std::fs::File::open(path).unwrap();
    let data_reader = std::io::BufReader::new(data_file);
    let mut data: Vec<String> = Vec::new();
    for line in data_reader.lines() {
        data.push(line.unwrap());
    }
    data

}

fn parse_test_data<S: AsRef<str>>(data: &[S]) -> Vec<u32> {
    let mut depths = Vec::<u32>::new();
    for value in data {
        let parsed_value = value.as_ref().parse::<u32>().unwrap();
        depths.push(parsed_value);
    }
    depths
}

fn count_increasing_depths(depths: &[u32]) -> u32 {
    let mut count: u32 = 0;
    for value in depths.windows(2){
        if value[0] < value[1]{
            count += 1;
        }
    }
    count
}

fn main() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let data_str: Vec<String> = read_test_data(data_dir.join("1.txt"));
    let data: Vec<u32> = parse_test_data(&data_str);
    let count: u32 = count_increasing_depths(&data);
    println!("number of increasing depths: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 10] = [
        "199",
        "200",
        "208",
        "210",
        "200",
        "207",
        "240",
        "269",
        "260",
        "263",
    ];

    #[test]
    fn test1() {
        let test_data = parse_test_data(&TEST_DATA);
        let count = count_increasing_depths(&test_data);
        assert_eq!(count, 7);
    }
}