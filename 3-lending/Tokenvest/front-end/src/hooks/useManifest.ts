import { CMS_API, CMS_PRODUCTS, CMS_PRODUCTS_REF, CMS_UPLOAD, DEFAULT_RAISED_AMOUNT, IMAGE_FIELD } from "@/constants/cms";
import { GATEWAY_URL, GATEWAY_URL_RESOURCES, GATEWAY_URL_DETAILS, PACKAGE_ID, TRANSACTION_SUCCESSFUL } from "@/constants/radix";
import { IProduct } from "@/interfaces/cmsInterface";
import { ITransactionRes } from "@/interfaces/radixInterface";
import { handleRequest, METHODS } from "@/utils/handleRequest";
import { createProductManifest, investManifest, withdrawManifest } from "@/utils/manifest";
import { useState } from "react";
import { useAccounts } from "./useAccounts";
import { useSendTransaction } from "./useSendTransaction";

/* 
useManifest is a custom hook which uses useAccounts, useSendTransaction and UseState hooks
and returns three functions and a variable: createProduct, invest, withdraw and isLoading
 */
export const useManifest = () => {
    const accounts = useAccounts();
    const sendTransaction = useSendTransaction();
    const [isLoading, setLoading] = useState(false);
    /*
    createProduct is a function which takes four arguments: title(product title), description(product description) and raiseAmount(the amount that should be raised for the product), image(blob)
    */
    const createProduct = async (title: string, description: string, raiseAmount: string, imageFile: Blob) => {
        setLoading(true);
        /* 
        1. Call sendTransaction method with createProductManifest
        sendTranscation is a method that takes manifest string as an argument and calls it using the sendTransaction() method provided by radix-dapp-toolkit
        */
        const transactionRes = await sendTransaction(createProductManifest(PACKAGE_ID, raiseAmount, title, accounts[0].address));
        /*
        2. Call handleRequest with gateway url of the nebunet with the response from the previous call
        By doing this we receive information about the transaction, using which we can make sure if the transaction was successful or not
        */
        const transactionInfo = await handleRequest(GATEWAY_URL, METHODS.POST, {
            "transaction_identifier": {
                "type": "intent_hash",
                "value_hex": (transactionRes as ITransactionRes).value.transactionIntentHash
            }
        });
        /*
        3. Check transaction status 
        */
        if (transactionInfo.transaction.transaction_status === TRANSACTION_SUCCESSFUL) {
            /*
            4. Call handleRequest with our CMS API url
            After making sure the transaction was successful, post request product data to our CMS, which includes
            title,description,raiseAmount: arguments for the function
            raisedAmount: the amount that has already been raised for the produect(default 0)
            componentId: component address we receive from transaction information
            ownerAddress: the account address of the user who created the product
            ownerResource: the owner badge resource address
            complete: the state of the product(default false)
            */
            const postRes = await handleRequest(`${CMS_API}${CMS_PRODUCTS}`, METHODS.POST, {
                "data": {
                    "title": title,
                    "description": description,
                    "raiseAmount": raiseAmount,
                    "raisedAmount": DEFAULT_RAISED_AMOUNT,
                    "componentId": transactionInfo.details.referenced_global_entities[0],
                    "ownerAddress": accounts[0].address,
                    "ownerResource": transactionInfo.details.referenced_global_entities[1],
                    "complete": false
                }
            });
            const id = postRes.data.id;
            if (postRes.data) {
                if (imageFile) {
                    const formData = new FormData();
                    formData.append("ref", CMS_PRODUCTS_REF);
                    formData.append("refId", id);
                    formData.append("field", IMAGE_FIELD);
                    formData.append("files", imageFile);
                    const postRes = await fetch(`${CMS_API}${CMS_UPLOAD}`, {
                        method: METHODS.POST,
                        body: formData
                    });
                    if (postRes.ok) {
                        setLoading(false);
                        alert("Product successfully created!!!");
                    } else {
                        setLoading(false);
                        alert("There was a problem with image")
                    }
                } else {
                    setLoading(false);
                    alert("Product successfully created!!!");
                }
            } else {
                setLoading(false);
                alert("Something went wrong!!!");
            }
        } else {
            setLoading(false);
        }
    };

    /*
    invest is a function which takes two arguments: investAmount(the amount to invest), product(product information)
    */
    const invest = async (investAmount: string, product: IProduct) => {
        setLoading(true);
        /*
        1. Call handleRequest with nebunet resources url
        By sendind the account address, we receive information about account resources
        */
        const resourcesData = await handleRequest(GATEWAY_URL_RESOURCES, METHODS.POST, {
            "address": accounts[0].address
        })
        /*
        2. Filter fungible resources
        */
        const resources = resourcesData.fungible_resources.items;
        await resources.forEach(async (item: { address: string }) => {
            /*
            3. Call handleRequest with nebunet details url
            By sending the resource address of every resource, we receive more details about the resources
            */
            const resourceDetails = await handleRequest(GATEWAY_URL_DETAILS, METHODS.POST, {
                "address": item.address
            });
            resourceDetails.metadata.items.forEach(async (item: { value: string }) => {
                /*
                4. Check if the resource is the Radix resource
                */
                if (item.value === "Radix") {
                    const radixAddress = resourceDetails.address;
                    /*
                    5. Call sendTransaction method with investManifest
                    */
                    const transactionRes = await sendTransaction(investManifest(accounts[0].address, investAmount, radixAddress, product.componentId));
                    /*
                    6. Call handleRequest with gateway url of the nebunet with the response from the previous call
                    By doing this we receive information about the transaction, using which we can make sure if the transaction was successful or not
                    */
                    const transactionInfo = await handleRequest(GATEWAY_URL, METHODS.POST, {
                        "transaction_identifier": {
                            "type": "intent_hash",
                            "value_hex": (transactionRes as ITransactionRes).value.transactionIntentHash
                        }
                    });
                    /*
                    7. Get the total raised amount using the transcation information 
                    */
                    const amount = transactionInfo.details.receipt.output[3].data_json;
                    /*
                    8. Check transaction status 
                    */
                    if (transactionInfo.transaction.transaction_status === TRANSACTION_SUCCESSFUL) {
                        /*
                        9. Call handleRequest with our CMS API url using the product id and the PUT method
                        to update data in our CMS.
                        Using the total amount from above we update the raisedAmount value 
                        */
                        const putRes = await handleRequest(`${CMS_API}${CMS_PRODUCTS}/${product.id}`, METHODS.PUT, {
                            "data": {
                                "raisedAmount": amount,
                            }
                        })
                        if (putRes.data) {
                            setLoading(false);
                            alert(`Successfully invested ${investAmount}!!!`);
                        } else {
                            setLoading(false);
                            alert("Something went wrong!!!");
                        }
                    } else {
                        setLoading(false);
                    }
                }
            })
        })
    };

    /*
    withdraw is a function which takes one argument: product(product information)
    */
    const withdraw = async (product: IProduct) => {
        setLoading(true);
        /*
        1. Call sendTransaction with withdrawManifest
        */
        const transactionRes = await sendTransaction(withdrawManifest(accounts[0].address, product.ownerResource, product.componentId));
        /*
        2. Call handleRequest with gateway url of the nebunet with the response from the previous call         
        By doing this we receive information about the transaction, using which we can make sure if the transaction was successful or not
        */
        const transactionInfo = await handleRequest(GATEWAY_URL, METHODS.POST, {
            "transaction_identifier": {
                "type": "intent_hash",
                "value_hex": (transactionRes as ITransactionRes).value.transactionIntentHash
            }
        });
        const updatedSubstates = transactionInfo.details.receipt.state_updates.updated_substates;
        /*
        3. Checking the amount of updated substates
        If the amount of the updated substates is one, the withdraw is not completely done
        If the amount of the updated substates is two, the withdraw is done successfully 
        That is because if the amount of updated substates is one, it means that an empty bucket is being send.
        In contrast if the amount of updates substates is two, it means that the withdraw amount is being received and sent to the account.
        */
        if (updatedSubstates.length === 1) {
            setLoading(false);
            alert("Not enough to withdraw !!!");
        } else {
            /*
            4. Call handleRequest with our CMS API url using the product id and the PUT method
            to update data in our CMS.
            Using the confirmation we get from above we update the complete status to true
            */
            const putRes = await handleRequest(`${CMS_API}${CMS_PRODUCTS}/${product.id}`, METHODS.PUT, {
                "data": {
                    "complete": true,
                }
            })
            if (putRes.data) {
                setLoading(false);
                alert(`Success!!!`);
            } else {
                setLoading(false);
                alert("Something went wrong!!!");
            }
        }

    };

    return { createProduct, invest, withdraw, isLoading };
};