"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Transport = void 0;
var TransportRequestManager_1 = require("./TransportRequestManager");
var Transport = /** @class */ (function () {
    function Transport() {
        this.transportRequestManager = new TransportRequestManager_1.TransportRequestManager();
        // add a noop for the error event to not require handling the error event
        // tslint:disable-next-line:no-empty
        this.transportRequestManager.transportEventChannel.on("error", function () { });
    }
    Transport.prototype.subscribe = function (event, handler) {
        this.transportRequestManager.transportEventChannel.addListener(event, handler);
    };
    Transport.prototype.unsubscribe = function (event, handler) {
        if (!event) {
            return this.transportRequestManager.transportEventChannel.removeAllListeners();
        }
        if (event && handler) {
            this.transportRequestManager.transportEventChannel.removeListener(event, handler);
        }
    };
    Transport.prototype.parseData = function (data) {
        if (data instanceof Array) {
            return data.map(function (batch) { return batch.request.request; });
        }
        return data.request;
    };
    return Transport;
}());
exports.Transport = Transport;
