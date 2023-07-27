pub mod dns_loader{
    // read from dns file split each line and each line split by "," in tuple. return Vec<(String, String, string, string)>
    pub fn load_dns() -> Vec<(String, String, String, String)>{
        let mut dns_list: Vec<(String, String, String, String)> = Vec::new();
        let file = std::fs::read_to_string("DNS").expect("Unable to read file");
        for line in file.lines(){
            let line = line.split(",").collect::<Vec<&str>>();
            dns_list.push((line[0].to_string(), line[1].to_string(), line[2].to_string(), line[3].to_string()));
        }
        dns_list
    }
}