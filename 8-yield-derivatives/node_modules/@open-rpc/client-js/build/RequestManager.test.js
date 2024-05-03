"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var RequestManager_1 = __importDefault(require("./RequestManager"));
var EventEmitterTransport_1 = __importDefault(require("./transports/EventEmitterTransport"));
var events_1 = require("events");
var eventEmitter_1 = require("./__mocks__/eventEmitter");
describe("client-js", function () {
    it("can be constructed and connect", function () {
        var emitter = new events_1.EventEmitter();
        var transport = new EventEmitterTransport_1.default(emitter, "from1", "to1");
        var c = new RequestManager_1.default([transport]);
        expect(!!c).toEqual(true);
    });
    it("can close", function () {
        var emitter = new events_1.EventEmitter();
        var transport = new EventEmitterTransport_1.default(emitter, "from1", "to1");
        var c = new RequestManager_1.default([transport]);
        c.close();
    });
    it("can send a request", function () { return __awaiter(void 0, void 0, void 0, function () {
        var emitter, transport, c, result;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    emitter = new events_1.EventEmitter();
                    eventEmitter_1.addMockServerTransport(emitter, "from1", "to1://local/rpc-response");
                    transport = new EventEmitterTransport_1.default(emitter, "from1", "to1://local/rpc-response");
                    c = new RequestManager_1.default([transport]);
                    return [4 /*yield*/, c.request({ method: "foo", params: ["bar"] })];
                case 1:
                    result = _a.sent();
                    expect(result.method).toEqual("foo");
                    expect(result.params).toEqual(["bar"]);
                    return [2 /*return*/];
            }
        });
    }); });
    it("can error on error response", function () { return __awaiter(void 0, void 0, void 0, function () {
        var emitter, transport, c;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    emitter = new events_1.EventEmitter();
                    eventEmitter_1.addMockServerTransport(emitter, "from1", "to1://local/rpc-error");
                    transport = new EventEmitterTransport_1.default(emitter, "from1", "to1://local/rpc-error");
                    c = new RequestManager_1.default([transport]);
                    return [4 /*yield*/, expect(c.request({ method: "foo", params: ["bar"] })).rejects.toThrowError("Error message")];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    }); });
    it("can error on malformed response and recieve error", function () { return __awaiter(void 0, void 0, void 0, function () {
        var emitter, transport, c, unknownError, formatError;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    emitter = new events_1.EventEmitter();
                    eventEmitter_1.addMockServerTransport(emitter, "from1", "to1://local/rpc-garbage");
                    transport = new EventEmitterTransport_1.default(emitter, "from1", "to1://local/rpc-garbage");
                    c = new RequestManager_1.default([transport]);
                    unknownError = new Promise(function (resolve) {
                        c.requestChannel.on("error", function (d) {
                            resolve(d);
                        });
                    });
                    return [4 /*yield*/, expect(c.request({ method: "foo", params: ["bar"] }, false, 1000))
                            .rejects.toThrowError("Request timeout request took longer than 1000 ms to resolve")];
                case 1:
                    _a.sent();
                    return [4 /*yield*/, unknownError];
                case 2:
                    formatError = _a.sent();
                    expect(formatError.message).toContain("Bad response format");
                    return [2 /*return*/];
            }
        });
    }); });
    it("can error on batching a request", function () { return __awaiter(void 0, void 0, void 0, function () {
        var emitter, transport, c;
        return __generator(this, function (_a) {
            emitter = new events_1.EventEmitter();
            transport = new EventEmitterTransport_1.default(emitter, "from1", "to1");
            c = new RequestManager_1.default([transport]);
            expect(function () { return c.stopBatch(); }).toThrow();
            return [2 /*return*/];
        });
    }); });
    it("can return errors on batch requests", function () { return __awaiter(void 0, void 0, void 0, function () {
        var emitter, transport, c, requests;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    emitter = new events_1.EventEmitter();
                    eventEmitter_1.addMockServerTransport(emitter, "from1", "to1://local/rpc-error");
                    transport = new EventEmitterTransport_1.default(emitter, "from1", "to1://local/rpc-error");
                    c = new RequestManager_1.default([transport]);
                    c.startBatch();
                    requests = [
                        c.request({ method: "foo", params: ["bar"] }),
                        c.request({ method: "foo", params: ["bar"] }),
                    ];
                    c.stopBatch();
                    return [4 /*yield*/, expect(Promise.all(requests)).rejects.toThrowError("Error message")];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    }); });
    it("can batch a request", function () { return __awaiter(void 0, void 0, void 0, function () {
        var emitter, transport, c, requests, _a, a, b;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    emitter = new events_1.EventEmitter();
                    eventEmitter_1.addMockServerTransport(emitter, "from1", "to1://local/rpc-response");
                    transport = new EventEmitterTransport_1.default(emitter, "from1", "to1://local/rpc-response");
                    c = new RequestManager_1.default([transport]);
                    c.startBatch();
                    requests = [
                        c.request({ method: "foo", params: [] }),
                        c.request({ method: "foo", params: ["bar"] })
                    ];
                    c.stopBatch();
                    return [4 /*yield*/, Promise.all(requests)];
                case 1:
                    _a = _b.sent(), a = _a[0], b = _a[1];
                    expect(a.method).toEqual("foo");
                    expect(b.method).toEqual("foo");
                    expect(a.params).toEqual([]);
                    expect(b.params).toEqual(["bar"]);
                    return [2 /*return*/];
            }
        });
    }); });
    it("can batch a notifications", function () { return __awaiter(void 0, void 0, void 0, function () {
        var emitter, transport, c, requests, _a, a, b;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    emitter = new events_1.EventEmitter();
                    eventEmitter_1.addMockServerTransport(emitter, "from1", "to1://local/rpc-response");
                    transport = new EventEmitterTransport_1.default(emitter, "from1", "to1://local/rpc-response");
                    c = new RequestManager_1.default([transport]);
                    c.startBatch();
                    requests = [
                        c.request({ method: "foo", params: [] }, true),
                        c.request({ method: "foo", params: ["bar"] }, true),
                    ];
                    c.stopBatch();
                    return [4 /*yield*/, Promise.all(requests)];
                case 1:
                    _a = _b.sent(), a = _a[0], b = _a[1];
                    return [2 /*return*/];
            }
        });
    }); });
    describe("stopBatch", function () {
        it("does nothing if the batch is empty", function () {
            var emitter = new events_1.EventEmitter();
            var transport = new EventEmitterTransport_1.default(emitter, "from1", "to1");
            transport.sendData = jest.fn();
            var c = new RequestManager_1.default([transport]);
            c.startBatch();
            c.stopBatch();
            expect(transport.sendData).not.toHaveBeenCalled();
        });
    });
    describe("startBatch", function () {
        it("it does nothing if a batch is already started", function () { return __awaiter(void 0, void 0, void 0, function () {
            var emitter, transport, c;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        emitter = new events_1.EventEmitter();
                        transport = new EventEmitterTransport_1.default(emitter, "from1", "to1");
                        c = new RequestManager_1.default([transport]);
                        return [4 /*yield*/, c.connect()];
                    case 1:
                        _a.sent();
                        c.startBatch();
                        c.request({ method: "foo", params: [] });
                        expect(c.batch.length).toBe(1);
                        c.startBatch();
                        c.request({ method: "foo", params: [] });
                        expect(c.batch.length).toBe(2);
                        return [2 /*return*/];
                }
            });
        }); });
    });
});
