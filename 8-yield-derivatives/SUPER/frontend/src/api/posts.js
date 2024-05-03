import axios from 'axios';

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL || "http://localhost:8080";

export const getLatestSaleDetails = async () => {

    const response = await axios.get(`${backendBaseUrl}/sale/latest`);
    const data = response.data[0];
    //console.log("From latest sale details", response);
    //console.log("^data", data)

    return data; // Return the data from the response
}

export const UpdateSaleDetailsToMongo = async (SaleDetailEvent) => {
    console.log("Sending SaleDetailEvent to MongoDB: ", SaleDetailEvent[0]);
    const response = await axios.post(`${backendBaseUrl}/sale/update/latest`, SaleDetailEvent[0]);
    console.log(response);
}

export const NewSaleDetailsToMongo = async (SaleDetailEvent) => {
    console.log("Sending SaleDetailEvent to MongoDB: ", SaleDetailEvent[0]);
    const response = await axios.post(`${backendBaseUrl}/sale/add`, SaleDetailEvent[0]);
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

//[SendNewNftToMongo
//     {
//         "dapp_definition_caddy": "component_tdx_2_1cqem9dzvh87u2e3nfcjzx9wkczgrsd787589rv7e78uvttrjprvqx7",
//         "component_caddy": "component_tdx_2_1cqem9dzvh87u2e3nfcjzx9wkczgrsd787589rv7e78uvttrjprvqx7",
//         "pool_caddy": "pool_tdx_2_1c3dsgj9f8z8xm8w4t3qunkapyp8yx0eel30f8nfpzs8xuahkmqrrcq",
//         "owner_badge_raddy": "resource_tdx_2_1n23wfv60xncwpqeu7j8p7um39d9tztmhmcd4zluaeal0j4tppr2da5",
//         "component_badge_raddy": "resource_tdx_2_1nfxxxxxxxxxxglcllrxxxxxxxxx002350006550xxxxxxxxxqtcnwk",
//         "db_updater_raddy": "resource_tdx_2_1tkgurnk05qdm2ar3mycfv8znesj9mmmey2uujk02ha5mzmjgdfh8l0",
//         "super_raddy": "resource_tdx_2_1t40mngr3rczn6u3laecxendwmw497vyh4et9jm62q7f67k530sa9ul",
//         "super_y_raddy": "resource_tdx_2_1tke7dt5xzejrqugq7pguc2fc5ysu62t32vsql4d69vreaqgsmv4nww",
//         "super_t_raddy": "resource_tdx_2_1tke7dt5xzejrqugq7pguc2fc5ysu62t32vsql4d69vreaqgsmv4nww",
//         "yield_nft_raddy": "resource_tdx_2_1nt5x4k8dsgjmtx29wf5ew5vvm5cvp7nkxy0jp458ukdd926mgrgk4n",
//         "sale_started": false,
//         "sale_completed": false,
//         "sale_start_time_unix": "0",
//         "sale_start_time_utc": "Sale hasn't begun",
//         "sale_end_time_unix": "0",
//         "sale_end_time_utc": "Sale hasn't begun"
//     }
// ]