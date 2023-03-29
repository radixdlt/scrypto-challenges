export const handleRequest = async (url: string, method: string, data: {}) => {
    const dataRes = await fetch(url, {
        method,
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    })
    const res = await dataRes.json();
    return res;
}

export const METHODS = Object.freeze({
    POST:"POST",
    PUT:"PUT"
})