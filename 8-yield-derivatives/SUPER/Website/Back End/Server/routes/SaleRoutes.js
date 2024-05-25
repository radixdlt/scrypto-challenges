
const express = require('express');
const router = express.Router();
const SaleModel = require('../models/SaleModel.js');
const goose = require('mongoose');
const {jsParser, jsonParser} = require("config/parser");


const parseSaleBody = async (body) => {
    const model = new SaleModel(
        {
            dapp_definition_caddy: body.dapp_definition_caddy,
            component_caddy: body.component_caddy,
            pool_caddy: body.pool_caddy,
            owner_badge_raddy: body.owner_badge_raddy,
            component_badge_raddy: body.component_badge_raddy,
            db_updater_raddy: body.db_updater_raddy,
            super_raddy: body.super_raddy,
            super_y_raddy: body.super_y_raddy,
            super_t_raddy: body.super_t_raddy,
            yield_nft_raddy: body.yield_nft_raddy,
            sale_started: body.sale_started === true,
            sale_completed: body.sale_completed === true,
            sale_start_time_unix: Number(body.sale_start_time_unix),
            sale_start_time_utc: body.sale_start_time_utc,
            sale_end_time_unix: Number(body.sale_end_time_unix),
            sale_end_time_utc: body.sale_end_time_utc,
        });
    return model;
}

/*#region get */

// GET all items
router.route('/').get((req, res) => {
    SaleModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json('Error in GET all items: ', err));
});

router.route('/latest').get((req, res) => {
    SaleModel.find()
        .sort({ updatedAt: -1 })
        .limit(1)
        .then(item => res.json(item))
        .catch(err => res.status(400).json('Error in latest: ',err));
});

// GET a single item by ID
router.route('/:id').get((req, res) => {
    SaleModel.findById(req.params.id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});


/*#endregion get */

/*#region post */

// POST a new item
router.route('/add').post(async (req, res) => {
    const newItem = await parseSaleBody(req.body);

    console.log("Sending Data: ", newItem);

    newItem.save()
        .then(() => res.status(200).json(`done adding ${newItem}`))
        .catch(err => res.status(400).json(err.message));
});



// UPDATE an item
router.route('/update/latest').post((req, res) => {
    const parsePromise = parseSaleBody(req.body);

    // Start querying MongoDB asynchronously
    const mongoQueryPromise = SaleModel.find()
        .sort({ updatedAt: -1 })
        .limit(1)
        .exec();

    Promise.all([parsePromise, mongoQueryPromise])
        .then(results => {
            const newModel = results[0];
            const latestItem = results[1][0];

            console.log("Sending Data: ", latestItem);
            res.status(200).json({ newModel, latestItem });
        })
        .catch(err => {
            console.error("Error handling request: ", err);
            res.status(400).json(err);
        });
});


// UPDATE an item
router.route('/update/:id').post((req, res) => {
    SaleModel.findById(req.params.id)
        .then(item => {
            item.field1 = req.body.field1; // Assuming 'field1' is a field of your model
            item.field2 = req.body.field2; // Replace with actual fields

            item.save()
                .then(() => res.json('Item updated!'))
                .catch(err => res.status(400).json(err));
        })
        .catch(err => res.status(400).json(err));
});

/*#endregion post */


















// DELETE an item
router.route('/:id').delete((req, res) => {
    SaleModel.findByIdAndDelete(req.params.id)
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