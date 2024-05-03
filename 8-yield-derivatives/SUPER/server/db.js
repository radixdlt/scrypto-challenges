// Create a MongoClient with a MongoClientOptions object to set the Stable API version
const {MongoClient, ServerApiVersion } = require("mongodb");

const username = 'nft_creator';
const pw = '0NShiIwcdkZO5arz';

const uri = `mongodb+srv://${username}:${pw}@yoo.brtac38.mongodb.net/nft_ids?retryWrites=true&w=majority`

const client = new MongoClient(uri, {
    serverApi: {
        version: ServerApiVersion.v1,
        strict: true,
        deprecationErrors: true,
    }
});

let dbConnection;

module.exports = {
    connectToDb: (cb) => {
        client.connect(uri)
            .then((client) => {
                dbConnection = client.db()
                return cb()
            })
            .catch((err) => {
                console.log(err)
                return cb(err)
            })
    },
    getDB: () => dbConnection
}