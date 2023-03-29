const http = require('http');
const url = require('url');

module.exports = http.createServer((req, res) => {

  var giftcode = require('./coupon-controller.js');
  const reqUrl =  url.parse(req.url, true);
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'OPTIONS, GET');
  res.setHeader('Access-Control-Max-Age', 2592000); // 30 days

  // POST route
  if (req.method === 'POST' && req.url == '/giftcode') {
    console.log('request type: ' + req.method + ' endpoint: ' + req.url);
    giftcode.generate(req, res);
  }
  // POST route
  else if (req.method === 'POST' && req.url == '/redeem/code') {
    console.log('request type: ' + req.method + ' endpoint: ' + req.url);
    giftcode.decode(req, res);
  }
  // invalid URL
  else {
    console.log('request type: ' + req.method + ' endpoint: ' + req.url);
    giftcode.invalid(req, res);
  }
})
