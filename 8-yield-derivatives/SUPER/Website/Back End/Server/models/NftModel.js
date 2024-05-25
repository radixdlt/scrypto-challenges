const mongoose = require('mongoose');
const Schema = mongoose.Schema;
const Decimal = mongoose.Types.Decimal128

const NftSchema = new Schema({
    _id: Number,
    hour_of_mint: Number,
    n_super_minted: Number,
    n_trust_minted: Decimal
}, { timestamps: true });

const NftModel = mongoose.model("NFT", NftSchema);

module.exports = NftModel;

