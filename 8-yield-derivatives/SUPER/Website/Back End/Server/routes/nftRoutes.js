// Import required modules
const express = require('express');
const router = express.Router();
const nftModel = require('../models/nftModel');
const goose = require('mongoose');
const saleModel = require("../models/saleModel");
const assert = require("assert");

/**
 * Parses the request body and creates a new NftModel instance.
 *
 * @param {Object} body - The request body containing NFT data.
 * @returns {Promise<Object>} - A promise that resolves to a new nftModel instance.
 */
const parseNFTBody = async (body) => {
    console.log("parsing", body);
        return new nftModel(
            {
                _id: parseInt(body.nft_id),
                hour_of_mint: parseInt(body.hour_of_mint),
                n_super_minted: parseInt(body.n_super_minted),
                n_trust_minted: parseFloat(body.n_trust_minted),
            }
    );
}


/*#region GET */


/**
 * GET all NFTs.
 * Responds with a JSON array of all NFTs in the database.
 */
router.route('/').get((req, res) => {
    nftModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json('Error: ', err));
});

/**
 * GET a single NFT by ID.
 * Responds with the NFT document corresponding to the provided ID.
 */
router.route('/:id').get((req, res) => {
    nftModel.findById(req.params.id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});


/*#endregion GET */


/*#region POST */


/**
 * POST a new NFT.
 * Creates a new NFT document in the database using the request body data.
 */
router.route('/buy').post(async (req, res) => {
    try {
        const newNft = await parseNFTBody(req.body);
        await newNft.save();
        res.status(200).json("done");
    } catch (err) {
        console.error("Error saving the NFT:", err);
        res.status(400).json(err.message);
    }
});


/**
 * POST to split an NFT.
 * Handles splitting an NFT into multiple parts and updates the database accordingly.
 */
router.route('/split').post(async (req, res) => {
    const {
        burnt_nft_id, burnt_nft_data, first_nft_id, first_nft_data, rest_nft_ids, rest_nft_data
    } = req.body;

    console.log("burnt");
    console.log(burnt_nft_id);
    console.log(burnt_nft_data);

    console.log("first_nft");
    console.log(first_nft_id);
    console.log(first_nft_data);

    console.log("rest_nfts");
    console.log(rest_nft_ids);
    console.log(rest_nft_data);

    let total_super = first_nft_data.n_super_minted + (rest_nft_ids.length * rest_nft_data.n_super_minted);
    console.log(`Burning: ${burnt_nft_data.n_super_minted} , Minting: ${total_super}`);

    // Convert string IDs to numbers
    const burntNftId = parseInt(burnt_nft_id);
    const firstNftId = parseInt(first_nft_id);
    const restNftIdsConverted = rest_nft_ids.map(id => parseInt(id));

    let saveOperations = [];

    // Update the burnt NFT by setting n_super_minted and n_trust_minted to zero
    saveOperations.push(nftModel.findByIdAndUpdate(burntNftId, {
        "n_super_minted": 0,
        "n_trust_minted": 0,
        "BURNT": true,
        "__v": 1
    }));

    // Create new NFT for the first split part
    const firstNft = new nftModel({
        _id: firstNftId,
        hour_of_mint: parseInt(first_nft_data.hour_of_mint),
        n_super_minted: parseInt(first_nft_data.n_super_minted),
        n_trust_minted: first_nft_data.n_trust_minted,
    });
    saveOperations.push(firstNft.save());

    // Create new NFTs for the rest of the split parts
    // Since all data is identical, no need to access by index
    restNftIdsConverted.forEach(nft_id => {
        const newNft = new nftModel({
            _id: nft_id,
            hour_of_mint: parseInt(rest_nft_data.hour_of_mint),
            n_super_minted: parseInt(rest_nft_data.n_super_minted),
            n_trust_minted: rest_nft_data.n_trust_minted,
        });
        saveOperations.push(newNft.save());
    });

    // Execute all operations
    await Promise.all(saveOperations);

    res.status(200).json("NFT split and burn status updated successfully");
});


/**
 * UPDATE an NFT.
 * Updates an existing NFT document in the database with the provided data.
 * Note: This endpoint is not currently used.
 */
router.route('/update/:id').post((req, res) => {
    nftModel.findById(req.params.nft_id)
        .then(item => {
            item.field1 = req.body.field1; // Assuming 'field1' is a field of your model
            item.field2 = req.body.field2; // Replace with actual fields

            item.save()
                .then(() => res.json('Item updated!'))
                .catch(err => res.status(400).json(err));
        })
        .catch(err => res.status(400).json(err));
});


/*#endregion POST */




/**
 * DELETE an NFT by ID.
 * Removes the NFT document corresponding to the provided ID from the database.
 * Note: This endpoint is not currently used.
 */
router.route('/:id').delete((req, res) => {
    nftModel.findByIdAndDelete(req.params.nft_id)
        .then(() => res.json('Item deleted.'))
        .catch(err => res.status(400).json(err));
});



module.exports = router;