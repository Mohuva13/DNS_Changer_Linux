pub mod apply {
    pub fn run_loading(time: &str){
        let mut running = std::process::Command::new("sh")
            .arg("./loading_.sh")
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
        let mut cmd = std::process::Command::new("cp");
        cmd.arg("-r");
        cmd.arg("./chosen_dns.txt");
        cmd.arg("/etc/resolv.conf");
        let output = cmd.output().expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}