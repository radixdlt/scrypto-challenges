const NewSuperModel = require('../models/SaleModel');
const StartSaleModel = require('../models/StartModel');

const preHandleNewSuper = (request, reply, done) => {
    NewSuperModel.count({}, (err, count) => {
        if (err) {
            console.error(err);
            reply.code(500).send('Error!');
        }
        reply.header('Content-Range', `notes 0-10}/${count}`);
        done();
    });
};


const preHandleStartSale = (request, reply, done) => {
    StartSaleModel.count({}, (err, count) => {
        if (err) {
            console.error(err);
            reply.code(500).send('Error!');
        }
        reply.header('Content-Range', `notes 0-10}/${count}`);
        done();
    });
};

module.exports = {preHandleNewSuper, preHandleStartSale};