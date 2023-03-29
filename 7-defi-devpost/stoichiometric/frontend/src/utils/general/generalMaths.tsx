function randomIntFromInterval(min: GLfloat, max: GLfloat) { // min and max included 
    return Math.floor(Math.random() * (max - min + 1) + min)
}

function twoDecimals(n: number) {
    const log10 = n ? Math.floor(Math.log10(n)) : 0,
        div = log10 < 0 ? Math.pow(10, 1 - log10) : 100;

    return Math.round(n * div) / div;
}

function formatToString(n: number) {
    const x = twoDecimals(n);
    if (x < 0) {
        const s = x.toLocaleString("en-US");
        return s.slice(1, s.length)
    }
    if (isNaN(x)) return "?";
    else return x.toLocaleString("en-US", { maximumFractionDigits: 9 })
}

function formatToString2(n: number) {
    return parseFloat(Math.abs(n).toFixed(2)).toLocaleString("en-US");
}

export { randomIntFromInterval, formatToString, formatToString2, twoDecimals };