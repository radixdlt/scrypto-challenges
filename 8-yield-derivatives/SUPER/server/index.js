
//import fastify & mongoose
const fastify = require('fastify');
const {connectToDatabase} = require('goose')
const newSuperRoutes = require('./routes/SaleRoutes');
const StartSuperRoutes = require('./routes/StartRoutes');
const {preHandleNewSuper, preHandleStartSale} = require('./hooks/contentRangeHook');

//initialized Fastify App
const app = fastify();

//connected fastify to mongoose
connectToDatabase();

app.addHook('preHandler', preHandleNewSuper);
app.addHook('preHandler', preHandleStartSale);

newSuperRoutes(app);
StartSuperRoutes(app);

//handle root route
app.get('/', (request, reply) => {
    try{
        reply.send("Hello world!");
    } catch(e) {
        console.error(e);
    }
})

//set application listening on port 5000 of localhost
app.listen({port: 8000}, (err, address) => {
    if (err) {
        throw err;
    }
    console.log(`Server running on ${address}`);
});