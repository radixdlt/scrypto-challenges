"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
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
var isomorphic_ws_1 = __importDefault(require("isomorphic-ws"));
var Transport_1 = require("./Transport");
var Request_1 = require("../Request");
var Error_1 = require("../Error");
var WebSocketTransport = /** @class */ (function (_super) {
    __extends(WebSocketTransport, _super);
    function WebSocketTransport(uri) {
        var _this = _super.call(this) || this;
        _this.uri = uri;
        _this.connection = new isomorphic_ws_1.default(uri);
        return _this;
    }
    WebSocketTransport.prototype.connect = function () {
        var _this = this;
        return new Promise(function (resolve, reject) {
            var cb = function () {
                _this.connection.removeEventListener("open", cb);
                resolve();
            };
            _this.connection.addEventListener("open", cb);
            _this.connection.addEventListener("message", function (message) {
                var data = message.data;
                _this.transportRequestManager.resolveResponse(data);
            });
        });
    };
    WebSocketTransport.prototype.sendData = function (data, timeout) {
        if (timeout === void 0) { timeout = 5000; }
        return __awaiter(this, void 0, void 0, function () {
            var prom, notifications;
            var _this = this;
            return __generator(this, function (_a) {
                prom = this.transportRequestManager.addRequest(data, timeout);
                notifications = Request_1.getNotifications(data);
                this.connection.send(JSON.stringify(this.parseData(data)), function (err) {
                    if (err) {
                        var jsonError = new Error_1.JSONRPCError(err.message, Error_1.ERR_UNKNOWN, err);
                        _this.transportRequestManager.settlePendingRequest(notifications, jsonError);
                        _this.transportRequestManager.settlePendingRequest(Request_1.getBatchRequests(data), jsonError);
                        prom = Promise.reject(jsonError);
                    }
                    _this.transportRequestManager.settlePendingRequest(notifications);
                });
                return [2 /*return*/, prom];
            });
        });
    };
    WebSocketTransport.prototype.close = function () {
        this.connection.close();
    };
    return WebSocketTransport;
}(Transport_1.Transport));
exports.default = WebSocketTransport;
