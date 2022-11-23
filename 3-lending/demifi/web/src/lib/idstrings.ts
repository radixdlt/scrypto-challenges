

export function shorten(str: string) {
    if (!str) return "";
    const MAXLEN: int = 8;
    if (str.length < MAXLEN) { return str; }
    
    // Just remove all leading zeros if there's enough of them
    let zerosplit = str.match(/^(0*)(.+)$/);
    if (str.length - zerosplit[1].length <= MAXLEN) {
	return zerosplit[2];
    }
    
    // Otherwise keep a few leading and trailing digits
    return str.substring(0, MAXLEN / 2) + '…' + str.substring(str.length - MAXLEN / 2);
 }

