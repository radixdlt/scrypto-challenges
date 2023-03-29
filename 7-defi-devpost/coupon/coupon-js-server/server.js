const http = require('http');

const hostname = '127.0.0.1';
const port = 3002;

const server = require('./route.js');

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
