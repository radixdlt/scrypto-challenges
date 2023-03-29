export function shortenAddress(address: string, firstLength: number = 4, lastLength: number = 6): string {
    const firstPart = address.slice(0, firstLength)
    const lastPart = address.slice(-lastLength)
    return `${firstPart}...${lastPart}`
}
