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
Object.defineProperty(exports, "__esModule", { value: true });
var Transport_1 = require("./Transport");
var Request_1 = require("../Request");
var Error_1 = require("../Error");
var EventEmitterTransport = /** @class */ (function (_super) {
    __extends(EventEmitterTransport, _super);
    function EventEmitterTransport(destEmitter, reqUri, resUri) {
        var _this = _super.call(this) || this;
        _this.connection = destEmitter;
        _this.reqUri = reqUri;
        _this.resUri = resUri;
        return _this;
    }
    EventEmitterTransport.prototype.connect = function () {
        var _this = this;
        this.connection.on(this.resUri, function (data) {
            _this.transportRequestManager.resolveResponse(data);
        });
        return Promise.resolve();
    };
    EventEmitterTransport.prototype.sendData = function (data, timeout) {
        if (timeout === void 0) { timeout = null; }
        var prom = this.transportRequestManager.addRequest(data, timeout);
        var notifications = Request_1.getNotifications(data);
        var parsedData = this.parseData(data);
        try {
            this.connection.emit(this.reqUri, parsedData);
            this.transportRequestManager.settlePendingRequest(notifications);
            return prom;
        }
        catch (e) {
            var responseErr = new Error_1.JSONRPCError(e.message, Error_1.ERR_UNKNOWN, e);
            this.transportRequestManager.settlePendingRequest(notifications, responseErr);
            return Promise.reject(responseErr);
        }
    };
    EventEmitterTransport.prototype.close = function () {
        this.connection.removeAllListeners();
    };
    return EventEmitterTransport;
}(Transport_1.Transport));
exports.default = EventEmitterTransport;
