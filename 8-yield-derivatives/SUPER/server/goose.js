const mongoose = require('mongoose');
require('dotenv').config();

const connectToDatabase = async () => {

    const uri = process.env.ENV_ATLAS_URI;

    try {
        if (mongoose.connection.readyState === 1) {
            console.log('Already Connected to MongoDB using Mongoose');
        } else {
            await mongoose.connect(uri);
            console.log('Connected to MongoDB using Mongoose');
        }
    } catch (error) {
        console.error('Failed to connect to MongoDB', error);
    }
}

module.exports = {connectToDatabase};
