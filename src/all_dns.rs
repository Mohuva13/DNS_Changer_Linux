pub mod dns_loader{
    pub fn load_dns() -> Vec<(&'static str, &'static str, &'static str, &'static str)>{
        let mut dns_list: Vec<(&'static str, &'static str, &'static str, &'static str)> = Vec::new();

        dns_list.push(("IR","Shecan","178.22.122.100","185.51.200.2"));
        dns_list.push(("US","Google Public DNS","8.8.8.8","8.8.4.4"));
        dns_list.push(("US","OpenDNS","208.67.222.222","208.67.220.220"));
        dns_list.push(("US","OpenDNS,2","208.67.222.220","208.67.220.222"));
        dns_list.push(("RU","Yandex","77.88.8.1","77.88.8.8"));
        dns_list.push(("AU","Cloudflare","1.1.1.1","1.0.0.1"));
        dns_list.push(("US","Norton ConnectSafe Basic","199.85.126.10","199.85.127.10"));
        dns_list.push(("US","Comodo Secure","8.26.56.26","8.20.247.20"));
        dns_list.push(("US","Dyn","216.146.35.35","216.146.36.36"));
        dns_list.push(("US","Norton DNS","198.153.192.1","198.153.194.1"));
        dns_list.push(("US","Comodo","156.154.70.22","156.154.71.22"));
        dns_list.push(("US","VeriSign Public DNS","64.6.64.6","64.6.65.6"));
        dns_list.push(("US","Qwest","205.171.3.65","205.171.2.65"));
        dns_list.push(("US","Sprint","204.97.212.10","204.117.214.10"));
        dns_list.push(("DK","Censurfridns","89.233.43.71","91.239.100.100"));
        dns_list.push(("RU","Safe DNS","195.46.39.39","195.46.39.40"));
        dns_list.push(("DE","DNS WATCH","84.200.69.80","84.200.70.40"));
        dns_list.push(("AT","FreeDNS","37.235.1.174","37.235.1.177"));
        dns_list.push(("US","Sprintlink","199.2.252.10","204.97.212.10"));
        dns_list.push(("US","UltraDNS","204.69.234.1","204.74.101.1"));
        dns_list.push(("GB","Zen Internet","212.23.8.1","212.23.3.1"));
        dns_list.push(("GB","Orange DNS","195.92.195.94","195.92.195.95"));
        dns_list.push(("US","Hurricane Electric","74.82.42.42",""));
        dns_list.push(("NL","Freenom World","80.80.80.80","80.80.81.81"));
        dns_list.push(("FR","FDN","80.67.169.12","80.67.169.40"));
        dns_list.push(("US","Neustar 1","156.154.70.1","156.154.71.1"));
        dns_list.push(("US","Neustar 2","156.154.70.5","156.154.71.5"));
        dns_list.push(("RU","AdGuard","94.140.14.14","94.140.15.15"));
        dns_list.push(("US","Quad9 Security","9.9.9.9","149.112.112.112"));
        dns_list.push(("US","Quad9 No Security","9.9.9.10","149.112.112.10"));
        dns_list.push(("BG","MegaLan","95.111.55.251","95.111.55.250"));
        dns_list.push(("US","OpenDNS Family","208.67.222.123","208.67.220.123"));
        dns_list.push(("US","Norton ConnectSafe Family","199.85.126.30","199.85.127.30"));
        dns_list.push(("RU","Yandex Family","77.88.8.7","77.88.8.3"));
        dns_list.push(("AU","Cloudflare Malware + Adult Blocking","1.1.1.3","1.0.0.3"));
        dns_list.push(("IE","CleanBrowsing Family","185.228.168.168","185.228.169.168"));
        dns_list.push(("IE","CleanBrowsing Adult Filter","185.228.168.10","185.228.169.11"));
        dns_list.push(("US","Neustar Family Secure","156.154.70.3","156.154.71.3"));
        dns_list.push(("US","Neustar Business Secure","156.154.70.4","156.154.71.4"));
        dns_list.push(("RU","AdGuard Family","94.140.14.15","94.140.15.16"));

        dns_list
    }
}