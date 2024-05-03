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
var PostMessageIframeTransport_1 = __importDefault(require("./PostMessageIframeTransport"));
var requestData_1 = require("../__mocks__/requestData");
var serve_handler_1 = __importDefault(require("serve-handler"));
var http_1 = __importDefault(require("http"));
var server;
var port;
beforeAll(function (done) {
    server = http_1.default.createServer(function (request, response) {
        return serve_handler_1.default(request, response, {
            public: "src/testiframe",
        });
    });
    server.listen(0, function () {
        if (server) {
            port = server.address().port; //tslint:disable-line
        }
        done();
    });
});
afterAll(function () {
    if (server) {
        server.close();
    }
});
describe("PostMessageIframeTransport", function () {
    describe("iframe", function () {
        it("can connect", function () {
            var pmt = new PostMessageIframeTransport_1.default("http://localhost:" + port + "/iframe.html");
            return pmt.connect();
        });
        it("can error connect with bad uri", function () {
            var pmt = new PostMessageIframeTransport_1.default("foo://localhost:5000/iframe.html");
            expect(pmt.connect()).rejects.toThrowError("Bad URI");
        });
        it("can close", function () {
            var pmt = new PostMessageIframeTransport_1.default("http://localhost:" + port + "/iframe.html");
            pmt.close();
        });
        it("can send and receive data", function () { return __awaiter(void 0, void 0, void 0, function () {
            var pmt, result;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        pmt = new PostMessageIframeTransport_1.default("http://localhost:" + port + "/iframe.html");
                        return [4 /*yield*/, pmt.connect()];
                    case 1:
                        _a.sent();
                        return [4 /*yield*/, pmt.sendData({
                                request: requestData_1.generateMockRequest(0, "foo", ["bar"]),
                                internalID: 0
                            })];
                    case 2:
                        result = _a.sent();
                        expect(result).toEqual("bar");
                        return [2 /*return*/];
                }
            });
        }); });
        it("can send and receive data against potential timeout", function () { return __awaiter(void 0, void 0, void 0, function () {
            var pmt, result;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        pmt = new PostMessageIframeTransport_1.default("http://localhost:" + port + "/iframe.html");
                        return [4 /*yield*/, pmt.connect()];
                    case 1:
                        _a.sent();
                        return [4 /*yield*/, pmt.sendData({
                                request: requestData_1.generateMockRequest(0, "foo", ["bar"]),
                                internalID: 0
                            }, 10000)];
                    case 2:
                        result = _a.sent();
                        expect(result).toEqual("bar");
                        return [4 /*yield*/, new Promise(function (resolve) { return setTimeout(resolve, 100); })];
                    case 3:
                        _a.sent();
                        return [2 /*return*/];
                }
            });
        }); });
        it("can send and receive errors", function () { return __awaiter(void 0, void 0, void 0, function () {
            var pmt;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        pmt = new PostMessageIframeTransport_1.default("http://localhost:" + port + "/iframe.html");
                        return [4 /*yield*/, pmt.connect()];
                    case 1:
                        _a.sent();
                        return [4 /*yield*/, expect(pmt.sendData({
                                request: requestData_1.generateMockRequest(1, "foo", ["bar"]),
                                internalID: 1,
                            })).rejects.toThrowError("Error message")];
                    case 2:
                        _a.sent();
                        return [4 /*yield*/, new Promise(function (resolve) { return setTimeout(resolve, 100); })];
                    case 3:
                        _a.sent();
                        return [2 /*return*/];
                }
            });
        }); });
        it("can handle underlying transport crash", function () { return __awaiter(void 0, void 0, void 0, function () {
            var pmt;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        pmt = new PostMessageIframeTransport_1.default("http://localhost:" + port + "/iframe.html");
                        return [4 /*yield*/, pmt.connect()];
                    case 1:
                        _a.sent();
                        return [4 /*yield*/, expect(pmt.sendData({
                                request: requestData_1.generateMockRequest(2, "foo", ["bar"]),
                                internalID: 2,
                            })).rejects.toThrowError("Random Segfault that crashes fetch")];
                    case 2:
                        _a.sent();
                        return [4 /*yield*/, new Promise(function (resolve) { return setTimeout(resolve, 100); })];
                    case 3:
                        _a.sent();
                        return [2 /*return*/];
                }
            });
        }); });
    });
});
