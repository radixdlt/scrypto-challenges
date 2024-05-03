
const express = require('express');
const router = express.Router();
const NftModel = require('../models/NftModel');
const goose = require('mongoose');
const SaleModel = require("../models/SaleModel");

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


// GET all items
router.route('/').get((req, res) => {
    NftModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json('Error: ', err));
});


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




// GET a single item by ID
router.route('/:id').get((req, res) => {
    NftModel.findById(req.body.nft_id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});







// UPDATE an item
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




// DELETE an item
router.route('/:id').delete((req, res) => {
    NftModel.findByIdAndDelete(req.params.nft_id)
        .then(() => res.json('Item deleted.'))
        .catch(err => res.status(400).json(err));
});



module.exports = router;





/*
module.exports = (app) => {
    // create a note
    app.post('/api/newSuper', NewSuperController.create);

    // get the list of notes
    app.get('/api/newSuper', NewSuperController.fetch);

    // get a single note
    app.get('/api/newSuper/:id', NewSuperController.get);

    // update a note
    app.put('/api/newSuper/:id', NewSuperController.update);

    // delete a note
    app.delete('/api/newSuper/:id', NewSuperController.delete);
};*/