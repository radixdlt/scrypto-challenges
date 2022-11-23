BRed='\033[1;31m'          # Red
BGreen='\033[1;32m'        # Green
Red='\033[0;31m'          # Red
Green='\033[0;32m'        # Green
Yellow='\033[0;33m'       # Yellow
BYellow='\033[1;33m'       # Yellow
Blue='\033[1;34m'         # Blue
Purple='\033[0;35m'       # Purple
LCyan='\033[0;36m'        # Green
Cyan='\033[1;36m'         # Cyan
NC='\033[0m'              # No Color

logbr () {
    >&2 echo -e "$BRed$@ $NC"
}

logbg () {
    >&2 echo -e "$BGreen$@ $NC"
}

logr () {
    >&2 echo -e "$Red$@ $NC"
}

logg () {
    >&2 echo -e "$LCyan$@ $NC"
}

logc () {
    >&2 echo -e "$BYellow=== $@ ===$NC"
}

logp () {
    >&2 echo -e "$Purple$@ $NC"
}

logy () {
    >&2 echo -e "$Yellow$@ $NC"
}

logb () {
    >&2 echo -e "$Blue$@ $NC"
}

completed () {
    >&2 echo -e "$BGreen========================$NC"
    >&2 echo -e "$BGreen====== COMPELETED ======$NC"
    >&2 echo -e "$BGreen========================$NC"
}
