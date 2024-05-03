const NewSuper = require('../models/SaleModel');

module.exports = {
    //# create a newSuper
    create: async (request, reply) => {
        try {
            const NewSuper = request.body;
            const newNewSuper = await NewSuper.create(NewSuper);
            reply.code(201).send(newNewSuper);
        } catch (err) {
            reply.code(500).send(err.message);
        }
    },

    //#get the list of newSupers
    fetch: async (request, reply) => {
        try {
            const NewSuperId = request.params.id;
            const newSuper = await NewSuper.findById(NewSuperId);
            reply.code(200).send(newSuper);
        } catch (e) {
            reply.code(500).send(e);
        }
    },

    get: async (request, reply) => {
        try {
            const NewSuperId = request.params.id;
            const newSuper = await NewSuper.findById(NewSuperId);
            reply.code(200).send(newSuper);
        } catch (e) {
            reply.code(500).send(e);
        }
    },

    //#update a newSuper
    update: async (request, reply) => {
        try {
            const NewSuperId = request.params.id;
            const updates = request.body;
            await NewSuper.findByIdAndUpdate(NewSuperId, updates);
            const newSuperToUpdate = await NewSuper.findById(NewSuperId);
            reply.code(200).send({ data: newSuperToUpdate });
        } catch (e) {
            reply.code(500).send(e);
        }
    },

    //#delete a newSuper
    delete: async (request, reply) => {
        try {
            const NewSuperId = request.params.id;
            const newSuperToDelete = await NewSuper.findById(NewSuperId);
            await NewSuper.findByIdAndDelete(NewSuperId);
            reply.code(200).send({ data: newSuperToDelete });
        } catch (e) {
            reply.code(500).send(e);
        }
    },
};