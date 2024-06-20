// Import required modules
const mongoose = require('mongoose');
require('dotenv').config(); // Load environment variables from .env file

/**
 * Connects to the MongoDB database using Mongoose.
 *
 * @returns {Promise<void>} A promise that resolves when the connection is successful.
 */
const connectToDatabase = async () => {
    // MongoDB connection URI from environment variables
    const uri = process.env.ENV_ATLAS_URI;

    try {
        // Check if already connected to MongoDB
        if (mongoose.connection.readyState === 1) {
            console.log('Already Connected to MongoDB using Mongoose');
        } else {
            // Connect to MongoDB
            await mongoose.connect(uri);
            console.log('Connected to MongoDB using Mongoose');
        }
    } catch (error) {
        console.error('Failed to connect to MongoDB', error);
    }
}

// Export the connectToDatabase function
module.exports = { connectToDatabase };
