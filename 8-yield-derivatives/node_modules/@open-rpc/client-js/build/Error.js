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
exports.convertJSONToRPCError = exports.JSONRPCError = exports.ERR_UNKNOWN = exports.ERR_MISSIING_ID = exports.ERR_TIMEOUT = void 0;
exports.ERR_TIMEOUT = 7777;
exports.ERR_MISSIING_ID = 7878;
exports.ERR_UNKNOWN = 7979;
var JSONRPCError = /** @class */ (function (_super) {
    __extends(JSONRPCError, _super);
    function JSONRPCError(message, code, data) {
        var _newTarget = this.constructor;
        var _this = _super.call(this, message) || this;
        _this.message = message;
        _this.code = code;
        _this.data = data;
        Object.setPrototypeOf(_this, _newTarget.prototype); // restore prototype chain: see https://github.com/open-rpc/client-js/issues/209
        return _this;
    }
    return JSONRPCError;
}(Error));
exports.JSONRPCError = JSONRPCError;
exports.convertJSONToRPCError = function (payload) {
    if (payload.error) {
        var _a = payload.error, message = _a.message, code = _a.code, data = _a.data;
        return new JSONRPCError(message, code, data);
    }
    return new JSONRPCError("Unknown error", exports.ERR_UNKNOWN, payload);
};
