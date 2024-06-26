> [!NOTE]
> This is documentation for the server used for the SUPER DApp
> - [How do I run the DApp?](../../README.md)
> - [What are you making?](../../../README.md)
> - [Scrypto Docs](../../../Smart%20Contract/README.md)
> - [Front End Docs](../../Front%20End/README.md)
> - [Try out the DApp on Stokenet](https://testnet.floww.fi)

# SUPER Back End (_Express_)

## Introduction

Welcome to the backend server of the SUPER DApp, built with Express.js. 
This server acts as the intermediary between the frontend and MongoDB, 
handling HTTP requests to manage data related to the SUPER DApp. This 
documentation provides a comprehensive guide to the server configuration.

## Table of Contents

1. [Folder Structure](#folder-structure)
2. [Server Setup and Configuration](#server-setup-and-configuration)
3. [Database Connection](#database-connection)
4. [Models](#models)
5. [Routes](#routes)
6. [License](#license)

## Folder Structure

The folder structure of the backend server is as follows:
```
.
├── models
│   ├── saleModel.js
│   └── nftModel.js
├── routes
│   ├── saleRoutes.js
│   └── nftRoutes.js
├── .env
├── app.js
└── goose.js
```

## Server Setup and Configuration

### Environment Variables [(`.env`)](.env)

- **`PORT`**: The port number on which the server will listen.
- **`ENV_ATLAS_URI`**: The MongoDB connection URI.

This setup ensures that the server is properly configured to handle requests, connect to the database, and route traffic to the appropriate handlers.

### Main Server File [(`app.js`)](app.js)
This file sets up the Express server, configures middleware, connects to the MongoDB database, and defines the main application routes.

#### Overview

- Import required modules
- Initialize Express app
- Configure CORS (Cross-Origin Resource Sharing)
- Middleware to parse JSON and URL-encoded data
- Connect to the database and start the server

#### Detailed Explanation

##### Import required modules

```javascript
const express = require('express');
const cors = require('cors');
const { connectToDatabase } = require('./goose'); // Import database connection function
require('dotenv').config(); // Load environment variables from .env file
```

The necessary modules are imported, including `express` for creating the server, `cors` for handling Cross-Origin Resource Sharing, and `dotenv` for loading environment variables.

##### Initialize Express app

```javascript
const app = express();
const port = process.env.PORT || 8080;
```

An Express application instance is created and the server port is set to the value specified in the environment variable `PORT` or 8080 by default.

##### Configure CORS

```javascript
const corsOptions = {
// Allow any subdomain of floww.fi, floww.fi, and any port on localhost
origin: [/\.floww\.fi$/, 'https://floww.fi', /^http:\/\/localhost:\d+$/],
};
app.use(cors(corsOptions));
```

CORS is configured to allow requests from any subdomain of `floww.fi`, the main `floww.fi` domain, and any port on `localhost`.

##### Middleware to parse JSON and URL-encoded data

```javascript
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
```

Express middleware is set up to parse JSON and URL-encoded data in incoming requests.

##### Connect to the database and start the server

```javascript
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
```

- **`connectToDatabase()`**: Attempts to connect to the MongoDB database using the `connectToDatabase` function from `goose.js`.
- **`.then()`**: If the connection is successful, the sale and NFT routes are imported and set up.
   - `app.use('/sale', saleRouter)`: Registers the sale routes with the `/sale` path.
   - `app.use('/nft', nftRouter)`: Registers the NFT routes with the `/nft` path.
- **`.catch()`**: If the connection fails, an error message is logged and the process exits with a failure code.
- **`app.listen(port)`**: Starts the server and listens on the specified port.

[Back to Table of Contents](#table-of-contents)

## Database Connection

### Mongoose [(`goose.js`)](goose.js)

This file handles the connection to the MongoDB database using Mongoose.

#### Overview

- Import required modules
- Define the `connectToDatabase` function to handle the connection
- Export the `connectToDatabase` function

#### Detailed Explanation

##### Define the `connectToDatabase` function

```javascript
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
```

- **connectToDatabase()**: An asynchronous function that connects to the MongoDB database using Mongoose.
   - **uri**: Retrieves the MongoDB connection URI from environment variables.
   - **try**:
      - Checks if there is already an active connection to MongoDB.
         - **mongoose.connection.readyState === 1**: If true, logs that the connection already exists.
      - If not already connected, attempts to connect using `mongoose.connect(uri)`.
      - Logs a success message if the connection is successful.
   - **catch**: Logs an error message if the connection attempt fails.

##### Export the `connectToDatabase` function

```javascript
// Export the connectToDatabase function
module.exports = { connectToDatabase };
```

The `connectToDatabase` function is exported for use in other parts of the application, allowing the database connection to be established when needed.

This setup ensures that the MongoDB database is properly connected using Mongoose, handling both initial connection attempts and reconnections if already connected.

[Back to Table of Contents](#table-of-contents)

[Back to Table of Contents](#table-of-contents)

## Models

### NFT Model [(`nftModel.js`)](./models/nftModel.js)
This file defines the schema and model for NFTs using Mongoose.

#### Overview

- Define the NFT schema with relevant fields and types
- Create and export the NFT model

```javascript
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
```

[Back to Table of Contents](#table-of-contents)

### Sale Model [(`saleModel.js`)](models/saleModel.js)

This file defines the schema and model for Sale details using Mongoose.  
This model was specifically defined to closely follow the structure of the ScryptoEvent `SaleDetailEvent`.
By doing so, it made my life 10x easier to send this to the database. I didn't have to restructure things 
to follow a specific pattern, it goes out of the smart contract receipt, through the frontend and into 
the backend without having to edit the event itself.

#### Overview

- Define the Sale schema with relevant fields and types
- Create and export the Sale model

```javascript
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

```

[Back to Table of Contents](#table-of-contents)

## Routes

### NFT Routes [(`nftRoutes.js`)](routes/nftRoutes.js)

This file defines the API endpoints for handling NFT-related operations.

#### `parseNFTBody` Function
This function is used in the POST routes (`/buy` and `/split`) to create a new NFT instance from the request body data
before saving it to the database.

The `parseNFTBody` function is an asynchronous function that takes in a request body containing NFT data and returns a new instance of the `nftModel` with the parsed data. This function is used to ensure the correct data types are assigned to each field of the NFT model before saving it to the database.

```javascript
/**
* Parses the request body and creates a new NftModel instance.
*
* @param {Object} body - The request body containing NFT data.
* @returns {Promise<Object>} - A promise that resolves to a new nftModel instance.
  */
  const parseNFTBody = async (body) => {
  console.log("parsing", body);
  return new nftModel({
  _id: parseInt(body.nft_id),
  hour_of_mint: parseInt(body.hour_of_mint),
  n_super_minted: parseInt(body.n_super_minted),
  n_trust_minted: parseFloat(body.n_trust_minted),
  });
  }
```

##### Explanation
- **_id**: Parsed as an integer from `body.nft_id`.
- **hour_of_mint**: Parsed as an integer from `body.hour_of_mint`.
- **n_super_minted**: Parsed as an integer from `body.n_super_minted`.
- **n_trust_minted**: Parsed as a floating-point number from `body.n_trust_minted`.

The function logs the parsed body to the console for debugging purposes and then creates a new `nftModel` instance with the parsed values.

##### Integration
You can find this function at the top of the `nftRoutes.js` file, and it's used in various POST routes to handle incoming NFT data.

#### NFT API Endpoints

```javascript
/* #region GET */

/**
 * GET all NFTs.
 * Responds with a JSON array of all NFTs in the database.
 */
router.route('/').get((req, res) => {
    nftModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json('Error: ', err));
});

/**
 * GET a single NFT by ID.
 * Responds with the NFT document corresponding to the provided ID.
 */
router.route('/:id').get((req, res) => {
    nftModel.findById(req.params.id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});

/* #endregion GET */

/* #region POST */

/**
 * POST a new NFT.
 * Creates a new NFT document in the database using the request body data.
 */
router.route('/buy').post(async (req, res) => {
    try {
        const newNft = await parseNFTBody(req.body);
        await newNft.save();
        res.status(200).json("done");
    } catch (err) {
        console.error("Error saving the NFT:", err);
        res.status(400).json(err.message);
    }
});

/**
 * POST to split an NFT.
 * Handles splitting an NFT into multiple parts and updates the database accordingly.
 */
router.route('/split').post(async (req, res) => {
    const {
        burnt_nft_id, burnt_nft_data, first_nft_id, first_nft_data, rest_nft_ids, rest_nft_data
    } = req.body;

    console.log("burnt");
    console.log(burnt_nft_id);
    console.log(burnt_nft_data);

    console.log("first_nft");
    console.log(first_nft_id);
    console.log(first_nft_data);

    console.log("rest_nfts");
    console.log(rest_nft_ids);
    console.log(rest_nft_data);

    let total_super = first_nft_data.n_super_minted + (rest_nft_ids.length * rest_nft_data.n_super_minted);
    console.log(`Burning: ${burnt_nft_data.n_super_minted} , Minting: ${total_super}`);

    // Convert string IDs to numbers
    const burntNftId = parseInt(burnt_nft_id);
    const firstNftId = parseInt(first_nft_id);
    const restNftIdsConverted = rest_nft_ids.map(id => parseInt(id));

    let saveOperations = [];

    // Update the burnt NFT by setting n_super_minted and n_trust_minted to zero
    saveOperations.push(nftModel.findByIdAndUpdate(burntNftId, {
        "n_super_minted": 0,
        "n_trust_minted": 0,
        "BURNT": true,
        "__v": 1
    }));

    // Create new NFT for the first split part
    const firstNft = new nftModel({
        _id: firstNftId,
        hour_of_mint: parseInt(first_nft_data.hour_of_mint),
        n_super_minted: parseInt(first_nft_data.n_super_minted),
        n_trust_minted: first_nft_data.n_trust_minted,
    });
    saveOperations.push(firstNft.save());

    // Create new NFTs for the rest of the split parts
    // Since all data is identical, no need to access by index
    restNftIdsConverted.forEach(nft_id => {
        const newNft = new nftModel({
            _id: nft_id,
            hour_of_mint: parseInt(rest_nft_data.hour_of_mint),
            n_super_minted: parseInt(rest_nft_data.n_super_minted),
            n_trust_minted: rest_nft_data.n_trust_minted,
        });
        saveOperations.push(newNft.save());
    });

    // Execute all operations
    await Promise.all(saveOperations);

    res.status(200).json("NFT split and burn status updated successfully");
});

/**
 * UPDATE an NFT.
 * Updates an existing NFT document in the database with the provided data.
 * Note: This endpoint is not currently used.
 */
router.route('/update/:id').post((req, res) => {
    nftModel.findById(req.params.nft_id)
        .then(item => {
            item.field1 = req.body.field1; // Assuming 'field1' is a field of your model
            item.field2 = req.body.field2; // Replace with actual fields

            item.save()
                .then(() => res.json('Item updated!'))
                .catch(err => res.status(400).json(err));
        })
        .catch(err => res.status(400).json(err));
});

/* #endregion POST */

/**
 * DELETE an NFT by ID.
 * Removes the NFT document corresponding to the provided ID from the database.
 * Note: This endpoint is not currently used.
 */
router.route('/:id').delete((req, res) => {
    nftModel.findByIdAndDelete(req.params.nft_id)
        .then(() => res.json('Item deleted.'))
        .catch(err => res.status(400).json(err));
});

module.exports = router;

```
[Back to Table of Contents](#table-of-contents)

### Sale Routes [(`saleRoutes.js`)](routes/saleRoutes.js)

This file defines the API endpoints for handling sale-related operations.

#### `parseSaleBody` Function
This function is used in the POST routes (`/add`, `/update/latest`, and `/update/:id`) to create a new sale instance from the request body data before saving it to the database.

The `parseSaleBody` function is an asynchronous function that takes in a request body containing sale data and returns a new instance of the `saleModel` with the parsed data. This function is used to ensure the correct data types are assigned to each field of the sale model before saving it to the database.

```javascript
/**
 * Parses the request body and creates a new SaleModel instance.
 *
 * @param {Object} body - The request body containing sale data.
 * @returns {Promise<Object>} - A promise that resolves to a new SaleModel instance.
 */
const parseSaleBody = async (body) => {
    const model = new saleModel({
        dapp_definition_caddy: body.dapp_definition_caddy,
        component_caddy: body.component_caddy,
        pool_caddy: body.pool_caddy,
        owner_badge_raddy: body.owner_badge_raddy,
        component_badge_raddy: body.component_badge_raddy,
        db_updater_raddy: body.db_updater_raddy,
        super_raddy: body.super_raddy,
        super_y_raddy: body.super_y_raddy,
        super_t_raddy: body.super_t_raddy,
        yield_nft_raddy: body.yield_nft_raddy,
        sale_started: body.sale_started === true,
        sale_completed: body.sale_completed === true,
        sale_start_time_unix: Number(body.sale_start_time_unix),
        sale_start_time_utc: body.sale_start_time_utc,
        sale_end_time_unix: Number(body.sale_end_time_unix),
        sale_end_time_utc: body.sale_end_time_utc,
    });
    return model;
}
```
##### Explanation
- **dapp_definition_caddy**: String, the DApp definition address.
- **component_caddy**: String, the component address.
- **pool_caddy**: String, the pool address.
- **owner_badge_raddy**: String, the owner badge resource address.
- **component_badge_raddy**: String, the component badge resource address.
- **db_updater_raddy**: String, the DB updater badge resource address.
- **super_raddy**: String, the SUPER token resource address.
- **super_y_raddy**: String, the SUPER Yield token resource address.
- **super_t_raddy**: String, the SUPER Trust token resource address.
- **yield_nft_raddy**: String, the Yield NFT resource address.
- **sale_started**: Boolean, indicates if the sale has started.
- **sale_completed**: Boolean, indicates if the sale is completed.
- **sale_start_time_unix**: Number, the sale start time in Unix timestamp.
- **sale_start_time_utc**: String, the sale start time in UTC string.
- **sale_end_time_unix**: Number, the sale end time in Unix timestamp.
- **sale_end_time_utc**: String, the sale end time in UTC string.

The function logs the parsed body to the console for debugging purposes and then creates a new `saleModel` instance with the parsed values.

##### Integration
You can find this function at the top of the `saleRoutes.js` file, and it's used in various POST routes to handle incoming sale data.

#### Sale API Endpoints

```javascript
/* #region GET */

/**
 * GET all sale items.
 * Responds with a JSON array of all sale items in the database.
 */
router.route('/').get((req, res) => {
    saleModel.find()
        .then(items => res.json(items))
        .catch(err => res.status(400).json('Error in GET all items: ', err));
});

/**
 * GET the latest sale item.
 * Responds with the most recently updated sale item in the database.
 */
router.route('/latest').get((req, res) => {
    saleModel.find()
        .sort({ updatedAt: -1 })
        .limit(1)
        .then(item => res.json(item))
        .catch(err => res.status(400).json('Error in latest: ', err));
});

/**
 * GET a single sale item by ID.
 * Responds with the sale item document corresponding to the provided ID.
 */
router.route('/:id').get((req, res) => {
    saleModel.findById(req.params.id)
        .then(item => res.json(item))
        .catch(err => res.status(400).json(err));
});

/* #endregion GET */

/* #region POST */

/**
 * POST a new sale item.
 * Creates a new sale item document in the database using the request body data.
 */
router.route('/add').post(async (req, res) => {
    const newItem = await parseSaleBody(req.body);

    console.log("Sending Data: ", newItem);

    newItem.save()
        .then(() => res.status(200).json(`done adding ${newItem}`))
        .catch(err => res.status(400).json(err.message));
});

/**
 * UPDATE the latest sale item.
 * Updates the most recently updated sale item in the database with the provided data.
 */
router.route('/update/latest').post((req, res) => {
    const parsePromise = parseSaleBody(req.body);

    // Start querying MongoDB asynchronously
    const mongoQueryPromise = saleModel.find()
        .sort({ updatedAt: -1 })
        .limit(1)
        .exec();

    Promise.all([parsePromise, mongoQueryPromise])
        .then(results => {
            const newModel = results[0];
            const latestItem = results[1][0];

            if (!latestItem) {
                return res.status(404).json('Sale item not found');
            }

            console.log("Sending Data: ", latestItem);
            // Save the updated item
            latestItem.save()
                .then(() => res.json('Sale item updated!'))
                .catch(err => res.status(400).json('Error updating sale item: ' + err));
        })
        .catch(err => {
            console.error("Error handling request: ", err);
            res.status(400).json(err);
        });
});

/**
 * UPDATE a sale item by ID.
 * Updates an existing sale item document in the database with the provided data.
 */
router.route('/update/:id').post((req, res) => {
    try {
        const parsePromise = parseSaleBody(req.body);

        // Start querying MongoDB asynchronously
        const mongoQueryPromise = saleModel.findById(req.params.id).exec();

        Promise.all([parsePromise, mongoQueryPromise])
            .then(results => {
                const newModel = results[0];
                const item = results[1];

                if (!item) {
                    return res.status(404).json('Sale item not found');
                }

                // Save the updated item
                item.save()
                    .then(() => res.json('Sale item updated!'))
                    .catch(err => res.status(400).json('Error updating sale item: ' + err));
            })
            .catch(err => {
                console.error("Error handling request: ", err);
                res.status(400).json(err);
            });
    } catch (err) {
        res.status(400).json('Error parsing sale item: ' + err.message);
    }
});

/* #endregion POST */

/* #region DELETE */

/**
 * DELETE a sale item by ID.
 * Removes the sale item document corresponding to the provided ID from the database.
 */
router.route('/:id').delete((req, res) => {
    saleModel.findByIdAndDelete(req.params.id)
        .then(() => res.json('Item deleted.'))
        .catch(err => res.status(400).json(err));
});

/* #endregion DELETE */

module.exports = router;
```

[Back to Table of Contents](#table-of-contents)

## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.