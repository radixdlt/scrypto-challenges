const mongoose = require('mongoose');
const Schema = mongoose.Schema;

const SaleDetailSchema = new Schema({
    // Components Addresses
    dapp_definition_caddy: {type: String, required: true},
    component_caddy: {type: String, required: true},
    pool_caddy: {type: String, required: true},

    // Badge Resource Addresses
    owner_badge_raddy: {type: String, required: true},
    component_badge_raddy: {type: String, required: true},
    db_updater_raddy: {type: String, required: true},

    // Token/NFT Resource Addresses
    super_raddy: {type: String, required: true},
    super_y_raddy: {type: String, required: true},
    super_t_raddy: {type: String, required: true},
    yield_nft_raddy: {type: String, required: true},

    // Token Sale Status
    sale_started: {type: Boolean, required: true},
    sale_completed: {type: Boolean, required: true},

    sale_start_time_unix: {type: Number, required: false},
    sale_start_time_utc: {type: String, required: false},

    sale_end_time_unix: {type: Number, required: false},
    sale_end_time_utc: {type: String, required: false}

}, { timestamps: true });

const SaleModel = mongoose.model('dawg', SaleDetailSchema);

module.exports = SaleModel;

