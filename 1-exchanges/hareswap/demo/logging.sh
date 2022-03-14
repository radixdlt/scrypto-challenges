RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color
H1="${RED}===${NC}"
PRE="${BLUE}>>>${NC}"
POST="${BLUE}<<<${NC}"

log () {
    >&2 echo -e "$H1 $@"
}

xlogp () {
    (>&2 echo -e "$PRE $@")
    $@ || exit 1;
}

xlog () {
    (>&2 echo -e "$PRE $@")
    $@ || exit 1;
    (>&2 echo -e "$POST")
}

alias resim="xlog resim"
alias _resim="xlogp resim"

success () {
    >&2 echo -e "${GREEN}=====================${NC}"
    >&2 echo -e "${GREEN}====== SUCCESS ======${NC}"
    >&2 echo -e "${GREEN}=====================${NC}"
}
