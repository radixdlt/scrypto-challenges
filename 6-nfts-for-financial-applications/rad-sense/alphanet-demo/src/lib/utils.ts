export function shortenAddress(address: String, startCharCount: number, endCharCount: number): string {
    return address.slice(0, startCharCount) + "..." + address.slice(-endCharCount);
}

export function parseAddress(sborJson: { type: string, value: String }): string {
    console.log("Parsing address: " + JSON.stringify(sborJson));
    return sborJson.value
        .replace(`${sborJson.type}("`, '')
        .replace('")', '');
}