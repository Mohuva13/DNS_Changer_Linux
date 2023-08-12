use nix::unistd::Uid;

mod applyer;
use applyer::apply::run_loading as run_loading;
use applyer::apply::chattr_cmd as chattr_cmd;
use applyer::apply::copy_dns_file as copy_dns_file;
use crate::cash_creator::create_cash_dns::export_chosen_dns;

mod Loading_generator;
mod all_dns;
mod cash_creator;



fn main() {

    // Create config directory if not exist
    cash_creator::create_cash_dns::create_config_directory();

    // Create cash file if not exist
    cash_creator::create_cash_dns::create_cash_dns();

    // Create shell script login file
    Loading_generator::loading_writer::write_loading();




    // Check user has linux root permission or not. If not, exit the program.
    if !Uid::effective().is_root() {
        println!("Please run as root!");
        std::process::exit(1);
    }


    let args: Vec<String> = std::env::args().collect();

    // -h or --help
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("Usage: sudo ./DNS_Changer_Linux [OPTIONS] [DNS1] [OPTIONS] [DNS2]");
        println!("OPTIONS:");
        println!("-h, --help\t\t\tShow help");
        println!("-n1\t\t\t\tSet DNS1");
        println!("-n2\t\t\t\tSet DNS2");
        println!("\n");
        println!("Example: sudo ./DNS_Changer_Linux -n1 1.1.1.1 -n2 1.0.0.1");
        println!("Or usually run : sudo ./dns_changer_linux ");
        std::process::exit(0);
    }

    // Fast apply dns with args
    if args.len() == 5 {
        if args[1] == "-n1" && args[3] == "-n2" {
            // check dns is valid or not
            let mut dns1 = args[2].clone();
            let mut dns2 = args[4].clone();
            if dns1.split(".").collect::<Vec<&str>>().len() == 4 && dns2.split(".").collect::<Vec<&str>>().len() == 4 {

                // chattr -i /etc/resolv.conf
                chattr_cmd("-i");

                export_chosen_dns(dns1.clone(), dns2.clone());
                copy_dns_file();

                {
                    run_loading("100");
                }

                println!("\n Done!");
                println!("\n GitHub: Mohuva13");
                std::process::exit(0);

            }else {
                println!("Invalid DNS!");
                std::process::exit(1);
            }

        }
    }

    let dns_list = all_dns::dns_loader::load_dns();


    // Show list DNS or find fastest DNS. (Ping test)
    println!("Do you want to find fastest DNS? (y/n)");
    let mut find_fastest = String::new();
    std::io::stdin()
        .read_line(&mut find_fastest)
        .expect("Failed to read line");
    let find_fastest = find_fastest.trim();
    if find_fastest == "y" {
        println!("Finding fastest DNS...");
        println!("\n");
        let mut fastest_dns: (String, String, String, String) = (String::new(), String::new(), String::new(), String::new());
        let mut fastest_time = 1000000.0;



        run_loading("10");

        // ping test
        for dns in dns_list {
            let mut cmd = std::process::Command::new("ping");
            cmd.arg("-c");
            cmd.arg("1");
            cmd.arg("-W");
            cmd.arg("1");
            cmd.arg(dns.2.clone());
            let output = cmd.output().expect("failed to execute process");
            let output = String::from_utf8_lossy(&output.stdout);
            let output = output.split("time=").collect::<Vec<&str>>();
            if output.len() > 1 {
                let output = output[1].split(" ").collect::<Vec<&str>>();
                let output = output[0].parse::<f32>().unwrap();
                if output < fastest_time {
                    fastest_time = output;
                    fastest_dns = (dns.0, dns.1, dns.2, dns.3 );
                }
            }
        }
        {
            run_loading("50");
            run_loading("85");
            run_loading("100");
        }

        // Test failed
        if fastest_time == 1000000.0 {
            println!("Test failed! Please try again and check your internet connection.");
            std::process::exit(1);
        }


        println!("Fastest DNS is: {} - {} - {} - {} - {}ms", fastest_dns.0, fastest_dns.1, fastest_dns.2, fastest_dns.3, fastest_time);
        println!("Do you want to use this DNS? (y/n)");
        let mut use_fastest = String::new();
        std::io::stdin()
            .read_line(&mut use_fastest)
            .expect("Failed to read line");
        let use_fastest = use_fastest.trim();
        if use_fastest == "y" {
            // Export choosen dns
            cash_creator::create_cash_dns::export_chosen_dns(fastest_dns.2.clone(), fastest_dns.3.clone());

            //set dns forever or only this session
            println!("Do you want to set DNS forever? (y/n)");
            let mut forever = String::new();
            std::io::stdin()
                .read_line(&mut forever)
                .expect("Failed to read line");
            let forever = forever.trim();
            if forever == "y" {
                // chattr -i /etc/resolv.conf
                chattr_cmd("-i");

                //export choosen dns
                cash_creator::create_cash_dns::export_chosen_dns(fastest_dns.2.clone(), fastest_dns.3.clone());

                // change dns
                copy_dns_file();

                {
                    run_loading("100");
                }
                println!("\n DNS changed to {}", fastest_dns.1);

                // chattr +i /etc/resolv.conf
                chattr_cmd("+i");

                println!("Done!");
                println!("\n GitHub: Mohuva13");
            } else {
                // chattr -i /etc/resolv.conf
                chattr_cmd("-i");


                //export choosen dns
                cash_creator::create_cash_dns::export_chosen_dns(fastest_dns.2.clone(), fastest_dns.3.clone());

                // change dns
                copy_dns_file();
                {
                    run_loading("100");
                }
                println!("\n DNS changed to {}", fastest_dns.1);

                println!("DNS will change to default after reboot!");
                println!("Done!");
                println!("\n GitHub: Mohuva13");
            }
        }
    }
    else {
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

        // Check dns_index is out of range
        if dns_index > dns_list.len() - 1 {
            println!("Please choose a number in list!");
            std::process::exit(1);
        }


        let dns = &dns_list[dns_index];
        println!("You choose: {}", dns.1);

        // Test ping of chosen dns
        println!("Do you want to test ping? (y/n)");
        let mut test_ping = String::new();
        std::io::stdin()
            .read_line(&mut test_ping)
            .expect("Failed to read line");
        let test_ping = test_ping.trim();
        if test_ping == "y" {
            // Test ping

            run_loading("40");

            let mut cmd = std::process::Command::new("ping");
            cmd.arg("-c");
            cmd.arg("5");
            cmd.arg(&dns.2);
            let output = cmd.output().expect("failed to execute process");

            //Show ping result
            {
                let output = String::from_utf8_lossy(&output.stdout);

                // Check if ping failed
                if output.contains("100% packet loss") {
                    println!("Ping failed!");
                    println!("\n GitHub: Mohuva13");
                    std::process::exit(1);
                    }

                if output.contains("unknown host") {
                    println!("Ping failed!");
                    println!("\n GitHub: Mohuva13");
                    std::process::exit(1);
                }

                if output == ""{
                    println!("\n Ping failed!");
                    println!("\n Check your internet connection and try again later!");
                    println!("\n GitHub: Mohuva13");
                    std::process::exit(1);
                }

                let output = output.split("\n").collect::<Vec<&str>>();
                let output = output[output.len() - 2].split(" ").collect::<Vec<&str>>();
                let output = output[output.len() - 2].split("/").collect::<Vec<&str>>();
                run_loading("100");
                println!("Ping result:");
                println!("Min: {}ms", output[0]);
                println!("Max: {}ms", output[2]);
                println!("Average: {}ms", output[1]);
            }

            // Ask user set DNS or not
            println!("Do you want to set DNS? (y/n)");
            let mut set_dns = String::new();
            std::io::stdin()
                .read_line(&mut set_dns)
                .expect("Failed to read line");
            let set_dns = set_dns.trim();
            if set_dns == "n" {
                println!("Ok!");
                println!("\n GitHub: Mohuva13");
                return;
            }
        }


        // Export choosen dns
        cash_creator::create_cash_dns::export_chosen_dns(dns.2.clone(), dns.3.clone());

        // chattr -i /etc/resolv.conf
        chattr_cmd("-i");

        // Choose set DNS forever or only for this session
        println!("Do you want to set DNS forever? (y/n)");
        let mut forever = String::new();
        std::io::stdin()
            .read_line(&mut forever)
            .expect("Failed to read line");
        let forever = forever.trim();



        // change dns
        copy_dns_file();
        println!("DNS changed to {}", dns.1);


        if forever == "y" {

            run_loading("100");


            // chattr +i /etc/resolv.conf
            chattr_cmd("+i");
        } else {
            run_loading("100");
            println!("DNS will change to default after reboot!");
        }
        println!("Done!");
        println!("\n GitHub: Mohuva13");
    }
}

