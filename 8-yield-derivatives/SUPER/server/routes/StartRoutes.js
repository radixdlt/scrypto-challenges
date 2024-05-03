const express = require('express');
const router = express.Router();
let StartModel = require('../models/StartModel.js');

// GET all items
router.route('/').get((req, res) => {
    StartModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json(err));
});

// GET a single item by ID
router.route('/:id').get((req, res) => {
    StartModel.findById(req.params.id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});

// POST a new item
router.route('/add').post((req, res) => {
    const newItem = new StartModel(req.body);

    newItem.save()
        .then(() => res.json('Item added!'))
        .catch(err => res.status(400).json(err));
});

// UPDATE an item
router.route('/update/:id').post((req, res) => {
    StartModel.findById(req.params.id)
        .then(item => {
            // Update each field with new values from req.body
            Object.assign(item, req.body);

            item.save()
                .then(() => res.json('Item updated!'))
                .catch(err => res.status(400).json(err));
        })
        .catch(err => res.status(400).json(err));
});

// DELETE an item
router.route('/:id').delete((req, res) => {
    StartModel.findByIdAndDelete(req.params.id)
        .then(() => res.json('Item deleted.'))
        .catch(err => res.status(400).json(err));
});

router.route('/latest').get((req, res) => {
    StartModel.findOne() // Use findOne to get a single document
        .sort({ createdAt: -1 }) // Sort by createdAt in descending order
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});


module.exports = router;