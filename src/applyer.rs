pub mod apply {
    use std::env;

    pub fn run_loading(time: &str){
        let home_dir = env::var("HOME").unwrap();
        let mut home_dir = home_dir + "/.config/DNS_Changer_Linux";

        // chmod +x loading_.sh
        let mut cmd = std::process::Command::new("chmod");
        cmd.arg("+x");
        cmd.arg(&home_dir + "/loading_.sh");
        let output = cmd.output().expect("failed to execute process");

        let mut running = std::process::Command::new("sh")
            .arg(home_dir + "/loading_.sh")
            .arg(time)
            .spawn().expect("");

        running.wait().expect("");
    }

    pub fn chattr_cmd(access_opt: &str){
        let mut cmd = std::process::Command::new("chattr");
        cmd.arg(access_opt);
        cmd.arg("/etc/resolv.conf");
        let output = cmd.output().expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    pub fn copy_dns_file(){
        let home_dir = env::var("HOME").unwrap();
        let mut home_dir = home_dir + "/.config/DNS_Changer_Linux";
        let mut cmd = std::process::Command::new("cp");
        cmd.arg("-r");
        cmd.arg(home_dir + "/chosen_dns.txt");
        cmd.arg("/etc/resolv.conf");
        let output = cmd.output().expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}