// Import required modules
const mongoose = require('mongoose');
const Schema = mongoose.Schema;
const Decimal = mongoose.Types.Decimal128; // Import Decimal128 type for precise decimal values

// Define the NFT schema
const nftSchema = new Schema({

     // Unique identifier for the NFT
     _id: Number,

     // The hour the NFT was minted
     hour_of_mint: Number,

     // Number of SUPER minted
     n_super_minted: Number,

     // Amount of trust tokens minted, using Decimal128 for precision
     n_trust_minted: Decimal

}, { timestamps: true }); // Automatically add createdAt and updatedAt timestamps

// Create the NFT model from the schema
const nftModel = mongoose.model("NFT", nftSchema);

// Export the NFT model
module.exports = nftModel;