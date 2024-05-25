
/*region Set up */

const express = require('express');
const cors = require('cors');
const {connectToDatabase} = require('./goose')
require('dotenv').config();

const app = express();
const port = process.env.PORT || 8080;

app.use(cors({ origin: 'https://api.floww.fi' })); // Set CORS origin

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cors())
/*endregion Set up */

connectToDatabase(process.env.ATLAS_URL).then(() => {
    const SaleRouter = require('./routes/SaleRoutes');
    const NftRouter = require('./routes/NftRoutes');

    app.use('/sale', SaleRouter);
    app.use('/nft', NftRouter);

    app.listen(port, () => {
        console.log(`Server listening on port ${port}`);
    });
}).catch(error => {
    console.log('Database connection failed', error);
    process.exit(1);
});




