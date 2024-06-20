// Import required modules
const express = require('express');
const cors = require('cors');
const { connectToDatabase } = require('./goose'); // Import database connection function
require('dotenv').config(); // Load environment variables from .env file


// Initialize Express app
const app = express();
const port = process.env.PORT || 8080;


// Configure CORS (Cross-Origin Resource Sharing)
const corsOptions = {
    // Allow any subdomain of floww.fi, floww.fi, and any port on localhost
    origin: [/\.floww\.fi$/, 'https://floww.fi', /^http:\/\/localhost:\d+$/],
};
app.use(cors(corsOptions));


// Middleware to parse JSON and URL-encoded data
app.use(express.json());
app.use(express.urlencoded({ extended: true }));


// Connect to the database and start the server
connectToDatabase().then(() => {
    // Import routes after successful database connection
    const saleRouter = require('./routes/saleRoutes');
    const nftRouter = require('./routes/nftRoutes');

    // Use the routes
    app.use('/sale', saleRouter);
    app.use('/nft', nftRouter);

    // Start the server
    app.listen(port, () => {
        console.log(`Server listening on port ${port}`);
    });
}).catch(error => {
    // Exit the process with failure code
    console.log('Database connection failed', error);
    process.exit(1);
});