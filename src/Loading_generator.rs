pub mod loading_writer{
    use std::fs::File;
    use std::io::prelude::*;
    use std::env;

    pub fn write_loading(){
        let home_dir = env::var("HOME").unwrap();
        let mut home_dir = home_dir + "/.config/DNS_Changer_Linux";
        let mut file = File::create(home_dir + "/loading_.sh").expect("create failed");

        file.write_all(b"function progress {
    bar=''
    clear
    for (( x=0; x <= $1; x++ )); do
        sleep 0.05
        bar=\"${bar} \"

        echo -ne \"\r\"
        echo -ne \"\\e[43m$bar\\e[0m\"

        local left=\"$(( 100 - $x ))\"
        printf \" %${left}s\"
        echo -n \"${x}%\"
    done
    echo -e \"\r\r\"
}
progress $1").expect("write failed");

    }
}