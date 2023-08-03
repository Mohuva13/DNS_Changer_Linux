pub mod create_cash_dns{
    use std::io::Write;

    pub fn create_cash_dns(){
        let mut file = std::fs::File::create("./all_dns.txt").expect("create failed");
        let _ = file.write_all("IR,Shecan,178.22.122.100,185.51.200.2\n".as_ref());
        let _ = file.write_all("US,Google Public DNS,8.8.8.8,8.8.4.4\n".as_ref());
        let _ = file.write_all("US,OpenDNS,208.67.222.222,208.67.220.220\n".as_ref());
        let _ = file.write_all("US,OpenDNS,2,208.67.222.220,208.67.220.222\n".as_ref());
        let _ = file.write_all("RU,Yandex,77.88.8.1,77.88.8.8\n".as_ref());
        let _ = file.write_all("AU,Cloudflare,1.1.1.1,1.0.0.1\n".as_ref());
        let _ = file.write_all("US,Norton ConnectSafe Basic,199.85.126.10,199.85.127.10\n".as_ref());
        let _ = file.write_all("US,Comodo Secure,8.26.56.26,8.20.247.20\n".as_ref());
        let _ = file.write_all("US,Dyn,216.146.35.35,216.146.36.36\n".as_ref());
        let _ = file.write_all("US,Norton DNS,198.153.192.1,198.153.194.1\n".as_ref());
        let _ = file.write_all("US,Comodo,156.154.70.22,156.154.71.22\n".as_ref());
        let _ = file.write_all("US,VeriSign Public DNS,64.6.64.6,64.6.65.6\n".as_ref());
        let _ = file.write_all("US,Qwest,205.171.3.65,205.171.2.65\n".as_ref());
        let _ = file.write_all("US,Sprint,204.97.212.10,204.117.214.10\n".as_ref());
        let _ = file.write_all("DK,Censurfridns,89.233.43.71,91.239.100.100\n".as_ref());
        let _ = file.write_all("RU,Safe DNS,195.46.39.39,195.46.39.40\n".as_ref());
        let _ = file.write_all("DE,DNS WATCH,84.200.69.80,84.200.70.40\n".as_ref());
        let _ = file.write_all("AT,FreeDNS,37.235.1.174,37.235.1.177\n".as_ref());
        let _ = file.write_all("US,Sprintlink,199.2.252.10,204.97.212.10\n".as_ref());
        let _ = file.write_all("US,UltraDNS,204.69.234.1,204.74.101.1\n".as_ref());
        let _ = file.write_all("GB,Zen Internet,212.23.8.1,212.23.3.1\n".as_ref());
        let _ = file.write_all("GB,Orange DNS,195.92.195.94,195.92.195.95\n".as_ref());
        let _ = file.write_all("NL,Freenom World,80.80.80.80,80.80.81.81\n".as_ref());
        let _ = file.write_all("FR,FDN,80.67.169.12,80.67.169.40\n".as_ref());
        let _ = file.write_all("US,Neustar 1,156.154.70.1,156.154.71.1\n".as_ref());
        let _ = file.write_all("US,Neustar 2,156.154.70.5,156.154.71.5\n".as_ref());
        let _ = file.write_all("RU,AdGuard,94.140.14.14,94.140.15.15\n".as_ref());
        let _ = file.write_all("US,Quad9 Security,9.9.9.9,149.112.112.112\n".as_ref());
        let _ = file.write_all("US,Quad9 No Security,9.9.9.10,149.112.112.10\n".as_ref());
        let _ = file.write_all("BG,MegaLan,95.111.55.251,95.111.55.250\n".as_ref());
        let _ = file.write_all("US,OpenDNS Family,208.67.222.123,208.67.220.123\n".as_ref());
        let _ = file.write_all("US,Norton ConnectSafe Family,199.85.126.30,199.85.127.30\n".as_ref());
        let _ = file.write_all("RU,Yandex Family,77.88.8.7,77.88.8.3\n".as_ref());
        let _ = file.write_all("AU,Cloudflare Malware + Adult Blocking,1.1.1.3,1.0.0.3\n".as_ref());
        let _ = file.write_all("IE,CleanBrowsing Family,185.228.168.168,185.228.169.168\n".as_ref());
        let _ = file.write_all("IE,CleanBrowsing Adult Filter,185.228.168.10,185.228.169.11\n".as_ref());
        let _ = file.write_all("US,Neustar Family Secure,156.154.70.3,156.154.71.3\n".as_ref());
        let _ = file.write_all("US,Neustar Business Secure,156.154.70.4,156.154.71.4\n".as_ref());
        let _ = file.write_all("RU,AdGuard Family,94.140.14.15,94.140.15.16\n".as_ref());
    }
}