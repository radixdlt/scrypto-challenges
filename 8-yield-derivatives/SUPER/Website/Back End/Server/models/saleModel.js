// Import required modules
const mongoose = require('mongoose');
const Schema = mongoose.Schema;

// Define the Sale Detail schema
const saleDetailSchema = new Schema({

    // Components Addresses
        // Component addresses - DApp definition address
        dapp_definition_caddy: { type: String, required: true },
        // Component addresses - Component address
        component_caddy: { type: String, required: true },
        // Component addresses - Pool
        pool_caddy: { type: String, required: true },

    // Badge Resource Addresses
        // Resource Addresses - Owner badge
        owner_badge_raddy: { type: String, required: true },
        // Resource Addresses - Component badge
        component_badge_raddy: { type: String, required: true },
        // Resource Addresses - DB updater badge
        db_updater_raddy: { type: String, required: true },

    // Token/NFT
        // Resource Addresses - SUPER token
        super_raddy: { type: String, required: true },
        // Resource Addresses - SUPER Yield token
        super_y_raddy: { type: String, required: true },
        // Resource Addresses - SUPER Trust token
        super_t_raddy: { type: String, required: true },
        // Resource Addresses - Yield NFT
        yield_nft_raddy: { type: String, required: true },

    // Token Sale Status
        // Sale status - Started
        sale_started: { type: Boolean, required: true },
        // Sale status - Completed
        sale_completed: { type: Boolean, required: true },

    // Sale time details
        // Unix time - Start
        sale_start_time_unix: { type: Number, required: false },
        // UTC time - Start
        sale_start_time_utc: { type: String, required: false },
        // Sale time - End time in Unix timestamp
        sale_end_time_unix: { type: Number, required: false },
        // Sale time - End time in UTC string
        sale_end_time_utc: { type: String, required: false }

}, { timestamps: true }); // Automatically add createdAt and updatedAt timestamps

const saleModel = mongoose.model('Sale', saleDetailSchema);

module.exports = saleModel;
