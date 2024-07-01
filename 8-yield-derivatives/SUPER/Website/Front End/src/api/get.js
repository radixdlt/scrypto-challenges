// Importing axios for making HTTP requests
import axios from "axios";

// Base URL for the backend, loaded from environment variables
const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL;

/**
 * Fetches the latest sale details from the backend.
 *
 * @returns {Promise<Object>} The latest sale details.
 */
export const getLatestSaleDetails = async () => {
    // Make a GET request to the backend to fetch the latest sale details
    const response = await axios.get(`${backendBaseUrl}/sale/latest`);
    const data = response.data[0];

    // Log the fetched sale details for debugging purposes
    console.log("From latest sale details", response.data[0]);

    return data; // Return the data from the response
}

/**
 * Fetches NFT data from the backend, which fetches it from MongoDB based on the given ID.
 *
 * @param {string} id - The ID of the NFT.
 * @returns {Promise<Object>} The NFT data.
 */
export const getNftDataFromMongo = async (id) => {

    // Log the ID of the NFT being fetched for debugging purposes
    console.log(`Getting data for NFT ${id} MongoDB:`);

    try {
        // Make a GET request to the backend to fetch NFT data
        const response = await axios.get(`${backendBaseUrl}/nft/${id}`);
        return response.data;
    } catch (error) {
        // Log the error message if the request fails
        console.error('Error fetching sale details:', error.message);

        throw error; // Throw the error so it can be caught by the caller
    }
}