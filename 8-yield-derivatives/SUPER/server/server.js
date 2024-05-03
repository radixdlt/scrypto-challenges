
/*region Set up */

const express = require('express');
const cors = require('cors');
const {connectToDatabase} = require('./goose')
require('dotenv').config();

const app = express();
const port = process.env.PORT || 8080;

app.use(cors());

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

/*endregion Set up */

connectToDatabase().then(() => {
    const SaleRouter = require('./routes/SaleRoutes');
    const NftRouter = require('./routes/NftRoutes');

    app.use('/sale', SaleRouter);
    app.use('/nft', NftRouter);

    app.listen(port, () => {
        console.log(`Server listening on port ${port}`);
    });
}).catch(error => {
    console.error('Database connection failed', error);
    process.exit(1);
});













/*
const express = require('express')
const {connectToDatabase} = require("./goose");
const mongoose = require("mongoose");

const {NewSuper} = require("./models/newSuperModel");
const {StartSale} = require("./models/StartSaleModel");

//initialize
const server = express();

//connect to db
let db = connectToDatabase();

server.listen(8080, () => {
    console.log('Connected to port8080')
})

server.get('/', (req, res) => {
    res.redirect('https://floww.fi');
});

// Redirect from subdomain
server.get('/assets', (req, res) => {
    res.redirect('https://assets.floww.fi');
    res.sendFile('./assets.html');
});


//region routes
server.get('/BeginSale/:id/:super_t_addy/:start_time_unix/:end_time_unix/:start_time_utc/:end_time_utc', (req, res) => {

    const id = req.params.id;
    const super_t_addy = req.params.super_t_addy;
    const start_time_unix = req.params.start_time_unix;
    const end_time_unix = req.params.end_time_unix;

    const encoded_start_time_utc = req.params.start_time_utc;
    const encoded_end_time_utc = req.params.end_time_utc;

    const start_time_utc = decodeURIComponent(encoded_start_time_utc);
    const end_time_utc = decodeURIComponent(encoded_end_time_utc);


fetch('/BeginSale', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({"yo":"data"})
})
    .then(response => response.json())
    .then(data => {
        console.log('Success:', data);
    })
    .catch((error) => {
        console.error('Error:', error);
    });

    NewSuper.findByIdAndUpdate(id, {
        super_t_raddy: 'super_t_addy'
    })
        .then((result) => {
            res.send(result)
        })
        .catch((err) => {
            console.log(err);
        });

    const start_sale = new StartSale ({
        sale_started: true,
        sale_completed: false,

        sale_start_time_unix: sale_start_time_unix,
        sale_start_time_utc: {type: Date, required: false},

        sale_end_time_unix: sale_end_time_unix,
        sale_end_time_utc: {type: Date, required: false}
    });

server.get('/newSuper', (req, res) => {

    const new_super = new NewSuper ({
        package_addy: 'account_tdx_2_12yrlyd0yadgw8nlfvwgrhel5wxxfezaj0tslxe7kj4mz5d9fzea8pe',

        // Components Addresses
        dapp_definition_caddy: 'account_tdx_2_12yrlyd0yadgw8nlfvwgrhel5wxxfezaj0tslxe7kj4mz5d9fzea8pe',
        component_caddy: 'component_tdx_2_1cq7n7wqm47mcl4nu45xexxx3pywm4gnkn8hw2j8rrpf68h4zvrrgvc',
        pool_caddy: 'pool_tdx_2_1csrl9haq2shf25lz7n8gxhk80yyctlhl0wtd8ewldu2z257mmw9qnh',

        // Badge Resource Addresses
        owner_badge_raddy: 'resource_tdx_2_1ntjjduregcjk4ch3ctqmw30ez8sc526zdkyqmfucf5tp6shrjnjaw5',
        component_badge_raddy: 'resource_tdx_2_1nfxxxxxxxxxxglcllrxxxxxxxxx002350006550xxxxxxxxxqtcnwk',
        db_updater_raddy: 'resource_tdx_2_1thz443q0kflk80v20afw8d0ww5z4j8e3kywrc4m05cyqfmetqduuc2',

        // Token/NFT Resource Addresses
        super_raddy: 'resource_tdx_2_1tk8jtmkfzh4w9rfhk703hk4tp895fga3sfhkkry97mgq3j86gsp8xz',
        super_y_raddy: 'resource_tdx_2_1thuzt0p45cwkfspa2wvwdm7uj2ywjpgm7fcyx696qj29dlszy24h9v',
        super_t_raddy: 'NA',
        yield_nft_raddy: 'resource_tdx_2_1nglptnhm8ua3fqfdph6x5vfltqgymhrwv3kc0k68crzextqd6r42t5',

        // Token Sale Timings and Status
        sale_started: false,
        sale_completed: false,
    });

    new_super.save()
        .then((results) => {
            res.send(results);
        })
        .catch((err) => {
            console.log(err);
        })

    res.json({message: 'Hello World!'})
})})


*/