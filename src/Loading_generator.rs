pub mod loading_writer{
    use std::fs::File;
    use std::io::prelude::*;
    use std::env;

    pub fn write_loading(){
        let home_dir = env::var("HOME").unwrap();
        let mut home_dir = home_dir + "/.config/DNS_Changer_Linux";
        let mut file = File::create(home_dir + "/loading_.sh").expect("create failed");;

        file.write_all(b"# Created by wujian on 2018/11/12\n
# Description: loading animation\n

    LR='\033[1;31m'\n
    LG='\033[1;32m'\n
    LY='\033[1;33m'\n
    LC='\033[1;36m'\n
    LW='\033[1;37m'\n
    NC='\033[0m'\n
    if [ \"${1}\" = \"0\" ]; then TME=$(date +\"%s\"); fi\n
    PRC=`printf \"%.0f\" ${1}`\n
    SHW=`printf \"%3d\n\" ${PRC}`\n
    LNE=`printf \"%.0f\" $((${PRC}/2))`\n
    LRR=`printf \"%.0f\" $((${PRC}/2-12))`; if [ ${LRR} -le 0 ]; then LRR=0; fi;\n
    LYY=`printf \"%.0f\" $((${PRC}/2-24))`; if [ ${LYY} -le 0 ]; then LYY=0; fi;\n
    LCC=`printf \"%.0f\" $((${PRC}/2-36))`; if [ ${LCC} -le 0 ]; then LCC=0; fi;\n
    LGG=`printf \"%.0f\" $((${PRC}/2-48))`; if [ ${LGG} -le 0 ]; then LGG=0; fi;\n
    LRR_=\"\" \n
    LYY_=\"\" \n
    LCC_=\"\" \n
    LGG_=\"\" \n
    for ((i=1;i<=13;i++))\n
    do\n
    	DOTS=\"\"; for ((ii=${i};ii<13;ii++)); do DOTS=\"${DOTS}.\"; done\n
    	if [ ${i} -le ${LNE} ]; then LRR_=\"${LRR_}#\"; else LRR_=\"${LRR_}.\"; fi\n
    	echo -ne \"  ${LW}${SEC}  ${LR}${LRR_}${DOTS}${LY}............${LC}............${LG}............ ${SHW}%${NC}\r\"\n
    	if [ ${LNE} -ge 1 ]; then sleep .05; fi\n
    done\n
    for ((i=14;i<=25;i++))\n
    do\n
    	DOTS=\"\"; for ((ii=${i};ii<25;ii++)); do DOTS=\"${DOTS}.\"; done\n
    	if [ ${i} -le ${LNE} ]; then LYY_=\"${LYY_}#\"; else LYY_=\"${LYY_}.\"; fi\n
    	echo -ne \"  ${LW}${SEC}  ${LR}${LRR_}${LY}${LYY_}${DOTS}${LC}............${LG}............ ${SHW}%${NC}\r\"\n
    	if [ ${LNE} -ge 14 ]; then sleep .05; fi\n
    done\n
    for ((i=26;i<=37;i++))\n
    do\n
    	DOTS=\"\"; for ((ii=${i};ii<37;ii++)); do DOTS=\"${DOTS}.\"; done\n
    	if [ ${i} -le ${LNE} ]; then LCC_=\"${LCC_}#\"; else LCC_=\"${LCC_}.\"; fi\n
    	echo -ne \"  ${LW}${SEC}  ${LR}${LRR_}${LY}${LYY_}${LC}${LCC_}${DOTS}${LG}............ ${SHW}%${NC}\r\"\n
    	if [ ${LNE} -ge 26 ]; then sleep .05; fi\n
    done\n
    for ((i=38;i<=49;i++))\n
    do\n
    	DOTS=\"\"; for ((ii=${i};ii<49;ii++)); do DOTS=\"${DOTS}.\"; done\n
    	if [ ${i} -le ${LNE} ]; then LGG_=\"${LGG_}#\"; else LGG_=\"${LGG_}.\"; fi\n
    	echo -ne \"  ${LW}${SEC}  ${LR}${LRR_}${LY}${LYY_}${LC}${LCC_}${LG}${LGG_}${DOTS} ${SHW}%${NC}\r\"\n
    	if [ ${LNE} -ge 38 ]; then sleep .05; fi\n
    done").expect("write failed");

    }
}