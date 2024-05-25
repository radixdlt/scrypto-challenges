import axios from 'axios';

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL;


export const UpdateSaleDetailsToMongo = async (SaleDetailEvent) => {
    const saleDetails = SaleDetailEvent[0];
    console.log("Sending SaleDetailEvent to MongoDB: ", saleDetails);
    const response = await axios.post(`${backendBaseUrl}/sale/add`, saleDetails);
    console.log(response);
}

export const NewSaleDetailsToMongo = async (SaleDetailEvent) => {
    //console.log("got: ", SaleDetailEvent);
    const saleDetails = SaleDetailEvent[0];
    console.log("Sending SaleDetailEvent to MongoDB: ", saleDetails);
    const response = await axios.post(`${backendBaseUrl}/sale/add`, saleDetails);
    console.log(response);
}



export const SendNewNftToMongo = async (CreateYieldNftEvent) => {
    console.log("Sending NFTCreationEvent to MongoDB: ", CreateYieldNftEvent[0]);
    try {
        const response = await axios.post(`${backendBaseUrl}/nft/buy`, CreateYieldNftEvent[0]);
        console.log(response);
    } catch (error) {
        console.error('Error fetching sale details:', error.message);
        throw error; // Throw the error so it can be caught by the caller
    }
}

