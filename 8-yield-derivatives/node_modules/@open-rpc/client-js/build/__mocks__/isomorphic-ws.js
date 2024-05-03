"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
var req = __importStar(require("./requestData"));
var WebSocket = /** @class */ (function () {
    function WebSocket(url, props) {
        this.callbacks = {};
        this.url = url;
    }
    WebSocket.prototype.addEventListener = function (eventName, callback) {
        this.callbacks[eventName] = callback;
        if (eventName === "open") {
            setTimeout(function () {
                callback();
            }, 10);
        }
    };
    WebSocket.prototype.removeEventListener = function (eventName, callback) {
        delete this.callbacks[eventName];
    };
    WebSocket.prototype.send = function (data, callback) {
        var _this = this;
        if (this.url.match(/crash-null/)) {
            callback();
            return;
        }
        if (this.url.match(/crash/)) {
            callback(new Error("Random Segfault that crashes fetch"));
            return;
        }
        Object.entries(this.callbacks).forEach(function (_a) {
            var eventName = _a[0], cb = _a[1];
            if (eventName === "message") {
                cb({ data: req.generateMockResponseData(_this.url, data) });
                callback();
            }
        });
    };
    WebSocket.prototype.close = function () {
        this.callbacks = {};
    };
    return WebSocket;
}());
exports.default = WebSocket;
