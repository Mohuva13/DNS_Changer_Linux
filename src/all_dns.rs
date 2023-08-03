pub mod dns_loader{
    use std::io::BufRead;

    pub fn load_dns() -> Vec<(String, String, String, String)> {
        let mut dns_list: Vec<(String, String, String, String)> = Vec::new();

        let file = std::fs::File::open("./all_dns.txt").expect("open failed");
        let reader = std::io::BufReader::new(&file);
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.clone(); // create a copy of line

            let line = line.split(",");
            let line: Vec<&str> = line.collect::<Vec<&str>>();

            if line.len() == 4 {
                let line_tuple = (line[0].to_owned(), line[1].to_owned(),
                                  line[2].to_owned(), line[3].to_owned());
                dns_list.push(line_tuple);
            }
        }
        dns_list

    }
}