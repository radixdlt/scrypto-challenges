"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var _1 = require(".");
var t = new _1.HTTPTransport("http://localhost:3333");
var c = new _1.Client(new _1.RequestManager([t]));
c.request({ method: "addition", params: [2, 2] }).then(function (result) {
    console.log('addition result: ', result); // tslint:disable-line
});
