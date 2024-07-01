import axios from 'axios';

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL;

/**
 * Sends updated sale details to the backend, which then processes and sends them to MongoDB.
 *
 * @param {Array} SaleDetailEvent - The sale detail event array.
 */
export const UpdateSaleDetailsToMongo = async (SaleDetailEvent) => {

    const saleDetails = SaleDetailEvent[0];
    console.log("Sending SaleDetailEvent to MongoDB: ", saleDetails);

    // Make a POST request to the backend to update sale details
    const response = await axios.post(`${backendBaseUrl}/sale/add`, saleDetails);

    console.log(response);
}

/**
 * Sends new sale details to the backend, which then processes and sends them to MongoDB.
 *
 * @param {Array} SaleDetailEvent - The sale detail event array.
 */
export const NewSaleDetailsToMongo = async (SaleDetailEvent) => {

    const saleDetails = SaleDetailEvent[0];
    console.log("Sending SaleDetailEvent to MongoDB: ", saleDetails);

    // Make a POST request to the backend to add new sale details
    const response = await axios.post(`${backendBaseUrl}/sale/add`, saleDetails);

    console.log(response);
}

/**
 * Sends new NFT creation event to the backend, which then processes and sends it to MongoDB.
 *
 * @param {Array} CreateYieldNftEvent - The NFT creation event array.
 */
export const SendNewNftToMongo = async (CreateYieldNftEvent) => {

    console.log("Sending NFTCreationEvent to MongoDB: ", CreateYieldNftEvent[0]);

    try {
        // Make a POST request to the backend to add new NFT data
        const response = await axios.post(`${backendBaseUrl}/nft/buy`, CreateYieldNftEvent[0]);
        console.log(response);
    } catch (error) {
        console.error('Error fetching sale details:', error.message);
        throw error; // Throw the error so it can be caught by the caller
    }
}

