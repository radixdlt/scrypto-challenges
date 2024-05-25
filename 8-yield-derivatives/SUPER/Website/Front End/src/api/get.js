import axios from "axios";

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL;


export const getLatestSaleDetails = async () => {

    const response = await axios.get(`${backendBaseUrl}/sale/latest`);
    const data = response.data[0];
    console.log("From latest sale details", response.data[0]);
    //console.log("^data", data)

    return data; // Return the data from the response
}


export const getNftDataFromMongo = async (id) => {
    console.log(`Getting data for NFT ${id} MongoDB:`);
    try {
        const response = await axios.get(`${backendBaseUrl}/nft/${id}`);
        return response.data;
    } catch (error) {
        console.error('Error fetching sale details:', error.message);
        throw error; // Throw the error so it can be caught by the caller
    }
}