"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var _1 = require(".");
var RequestManager_1 = __importDefault(require("./RequestManager"));
var EventEmitterTransport_1 = __importDefault(require("./transports/EventEmitterTransport"));
var events_1 = require("events");
var eventEmitter_1 = require("./__mocks__/eventEmitter");
var requestData_1 = require("./__mocks__/requestData");
describe("client-js", function () {
    it("can be constructed", function () {
        var emitter = new events_1.EventEmitter();
        var c = new _1.Client(new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]));
        expect(!!c).toEqual(true);
    });
    it("has a request method that returns a promise", function () {
        var emitter = new events_1.EventEmitter();
        var c = new _1.Client(new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]));
        expect(typeof c.request).toEqual("function");
        expect(typeof c.request({ method: "my_method" }).then).toEqual("function");
    });
    it("has a notify method that returns a promise", function () {
        var emitter = new events_1.EventEmitter();
        var c = new _1.Client(new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]));
        expect(typeof c.request).toEqual("function");
        expect(typeof c.notify({ method: "my_method" }).then).toEqual("function");
    });
    it("can recieve notifications", function (done) {
        var emitter = new events_1.EventEmitter();
        var c = new _1.Client(new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]));
        eventEmitter_1.addMockServerTransport(emitter, "from1", "to1://asdf/rpc-notification");
        c.onNotification(function () { return done(); });
        emitter.emit("to1", JSON.stringify(requestData_1.generateMockNotificationRequest("foo", ["bar"])));
    });
    it("can register error and subscription handlers", function () {
        var emitter = new events_1.EventEmitter();
        var c = new _1.Client(new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]));
        // tslint:disable-next-line:no-empty
        c.onError(function (err) { });
        // tslint:disable-next-line:no-empty
        c.onNotification(function (data) { });
    });
    describe("startBatch", function () {
        it("calls startBatch", function () {
            var emitter = new events_1.EventEmitter();
            var rm = new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]);
            var c = new _1.Client(rm);
            c.startBatch();
            //      expect(mockedRequestManager.mock.instances[0].startBatch).toHaveBeenCalled();
        });
    });
    describe("can call stopBatch", function () {
        var emitter = new events_1.EventEmitter();
        var rm = new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]);
        var c = new _1.Client(rm);
        c.startBatch();
        c.stopBatch();
    });
    describe("can close", function () {
        var emitter = new events_1.EventEmitter();
        var rm = new RequestManager_1.default([new EventEmitterTransport_1.default(emitter, "from1", "to1")]);
        var c = new _1.Client(rm);
        c.close();
    });
});
