
const express = require('express');
const router = express.Router();
const NftModel = require('../models/NftModel');
const goose = require('mongoose');
const SaleModel = require("../models/SaleModel");
const assert = require("assert");

const parseNFTBody = async (body) => {
    console.log("parsing", body);
        return new NftModel(
            {
                _id: parseInt(body.nft_id),
                hour_of_mint: parseInt(body.hour_of_mint),
                n_super_minted: parseInt(body.n_super_minted),
                n_trust_minted: parseFloat(body.n_trust_minted),
            }
    );
}


/*#region GET */


// GET all items
router.route('/').get((req, res) => {
    NftModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json('Error: ', err));
});


// GET a single item by ID
router.route('/:id').get((req, res) => {
    NftModel.findById(req.params.id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});




/*#endregion GET */



/*#region POST */


// POST a new item
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




// SPLIT NFT
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

    // Convert string IDs to numbers if necessary
    const burntNftId = parseInt(burnt_nft_id);
    const firstNftId = parseInt(first_nft_id);
    const restNftIdsConverted = rest_nft_ids.map(id => parseInt(id));

    let saveOperations = [];

    // Update the burnt NFT by setting n_super_minted and n_trust_minted to zero
    saveOperations.push(NftModel.findByIdAndUpdate(burntNftId, {
        "n_super_minted": 0,
        "n_trust_minted": 0,
        "BURNT": true,
        "__v": 1
    }));

    // Create new NFT for the first split part
    const firstNft = new NftModel({
        _id: firstNftId,
        hour_of_mint: parseInt(first_nft_data.hour_of_mint),
        n_super_minted: parseInt(first_nft_data.n_super_minted),
        n_trust_minted: first_nft_data.n_trust_minted,
    });
    saveOperations.push(firstNft.save());

    // Create new NFTs for the rest of the split parts
    // Since all data is identical, no need to access by index
    restNftIdsConverted.forEach(nft_id => {
        const newNft = new NftModel({
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


// UPDATE an item (Not really used)
router.route('/update/:id').post((req, res) => {
    NftModel.findById(req.params.nft_id)
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




// DELETE an item (Not really used)
router.route('/:id').delete((req, res) => {
    NftModel.findByIdAndDelete(req.params.nft_id)
        .then(() => res.json('Item deleted.'))
        .catch(err => res.status(400).json(err));
});



module.exports = router;