mod file_loader;

fn main() {

    let dns_list = file_loader::dns_loader::load_dns();

    // Show the dns_list and user choose one of them

    println!("DNS List:");
    for (i, dns) in dns_list.iter().enumerate() {
        println!("{}. {} - {}", i, dns.0, dns.1);
    }
    println!("Choose one of them:");
    let mut dns_index = String::new();
    std::io::stdin()
        .read_line(&mut dns_index)
        .expect("Failed to read line");
    let dns_index: usize = dns_index.trim().parse().expect("Please type a number!");
    let dns = &dns_list[dns_index];
    println!("You choose: {}", dns.1);


}
