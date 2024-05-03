const mongoose = require('mongoose');
const Schema = mongoose.Schema;

const StartSchema = new Schema({
    super_t_raddy: String,
    // Token Sale Timings and Status
    sale_started: Boolean,
    sale_completed: Boolean,

    sale_start_time_unix: {type: Number, required: false},
    sale_start_time_utc: {type: String, required: false},

    sale_end_time_unix: {type: Number, required: false},
    sale_end_time_utc: {type: String, required: false}

}, { timestamps: true });

const StartModel = mongoose.model("dawg", StartSchema);

module.exports = StartModel;

