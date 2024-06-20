const express = require('express');
const router = express.Router();
const saleModel = require('../models/saleModel.js');
const goose = require('mongoose');
const {jsParser, jsonParser} = require("config/parser");

/**
 * Parses the request body and creates a new SaleModel instance.
 *
 * @param {Object} body - The request body containing sale data.
 * @returns {Promise<Object>} - A promise that resolves to a new SaleModel instance.
 */
const parseSaleBody = async (body) => {
    const model = new saleModel(
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


/* #region GET */

/**
 * GET all sale items.
 * Responds with a JSON array of all sale items in the database.
 */
router.route('/').get((req, res) => {
    saleModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json('Error in GET all items: ', err));
});

/**
 * GET the latest sale item.
 * Responds with the most recently updated sale item in the database.
 */
router.route('/latest').get((req, res) => {
    saleModel.find()
        .sort({ updatedAt: -1 })
        .limit(1)
        .then(item => res.json(item))
        .catch(err => res.status(400).json('Error in latest: ',err));
});

/**
 * GET a single sale item by ID.
 * Responds with the sale item document corresponding to the provided ID.
 */
router.route('/:id').get((req, res) => {
    saleModel.findById(req.params.id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});

/* #endregion GET */

/* #region POST */

/**
 * POST a new sale item.
 * Creates a new sale item document in the database using the request body data.
 */
router.route('/add').post(async (req, res) => {
    const newItem = await parseSaleBody(req.body);

    console.log("Sending Data: ", newItem);

    newItem.save()
        .then(() => res.status(200).json(`done adding ${newItem}`))
        .catch(err => res.status(400).json(err.message));
});

/**
 * UPDATE the latest sale item.
 * Updates the most recently updated sale item in the database with the provided data.
 */
router.route('/update/latest').post((req, res) => {
    const parsePromise = parseSaleBody(req.body);

    // Start querying MongoDB asynchronously
    const mongoQueryPromise = saleModel.find()
        .sort({ updatedAt: -1 })
        .limit(1)
        .exec();

    Promise.all([parsePromise, mongoQueryPromise])
        .then(results => {
            const newModel = results[0];
            const latestItem = results[1][0];

            if (!latestItem) {
                return res.status(404).json('Sale item not found');
            }

            console.log("Sending Data: ", latestItem);
            // Save the updated item

            // Save the updated item
            latestItem.save()
                .then(() => res.json('Sale item updated!'))
                .catch(err => res.status(400).json('Error updating sale item: ' + err));
        })
           .catch(err => {
               console.error("Error handling request: ", err);
               res.status(400).json(err);
           });
});

/**
 * UPDATE a sale item by ID.
 * Updates an existing sale item document in the database with the provided data.
 */
router.route('/update/:id').post((req, res) => {
    try {
        const parsePromise = parseSaleBody(req.body);

        // Start querying MongoDB asynchronously
        const mongoQueryPromise = saleModel.findById(req.params.id).exec();

        Promise.all([parsePromise, mongoQueryPromise])
               .then(results => {
                   const newModel = results[0];
                   const item = results[1];

                   if (!item) {
                       return res.status(404).json('Sale item not found');
                   }

                   // Save the updated item
                   item.save()
                       .then(() => res.json('Sale item updated!'))
                       .catch(err => res.status(400).json('Error updating sale item: ' + err));
               })
               .catch(err => {
                   console.error("Error handling request: ", err);
                   res.status(400).json(err);
               });
    } catch (err) {
        res.status(400).json('Error parsing sale item: ' + err.message);
    }
});

/* #endregion POST */

/* #region DELETE */

/**
 * DELETE a sale item by ID.
 * Removes the sale item document corresponding to the provided ID from the database.
 */
router.route('/:id').delete((req, res) => {
    saleModel.findByIdAndDelete(req.params.id)
        .then(() => res.json('Item deleted.'))
        .catch(err => res.status(400).json(err));
});

/* #endregion DELETE */

module.exports = router;