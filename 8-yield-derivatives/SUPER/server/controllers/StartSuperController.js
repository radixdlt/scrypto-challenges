const StartSale = require('../models/StartModel');

module.exports = {
    //# create a note
    create: async (request, reply) => {
        try {
            const startSale = request.body;
            const newStartSale = await StartSale.create(startSale);
            reply.code(201).send(newStartSale);
        } catch (err) {
            reply.code(500).send(err.message);
        }
    },

    //#get the list of notes
    fetch: async (request, reply) => {
        try {
            const startSaleId = request.params.id;
            const startSale = await StartSale.findById(startSaleId);
            reply.code(200).send(startSale);
        } catch (e) {
            reply.code(500).send(e);
        }
    },

    get: async (request, reply) => {
        try {
            const startSaleId = request.params.id;
            const startSale = await StartSale.findById(startSaleId);
            reply.code(200).send(startSale);
        } catch (e) {
            reply.code(500).send(e);
        }
    },

    //#update a newSuper
    update: async (request, reply) => {
        try {
            const startSaleId = request.params.id;
            const updates = request.body;
            await StartSale.findByIdAndUpdate(startSaleId, updates);
            const StartSaleToUpdate = await StartSale.findById(startSaleId);
            reply.code(200).send({ data: StartSaleToUpdate });
        } catch (e) {
            reply.code(500).send(e);
        }
    },

    //#delete a newSuper
    delete: async (request, reply) => {
        try {
            const startSaleId = request.params.id;
            const startSaleToDelete = await StartSale.findById(startSaleId);
            await StartSale.findByIdAndDelete(startSaleId);
            reply.code(200).send({ data: startSaleToDelete });
        } catch (e) {
            reply.code(500).send(e);
        }
    },
};