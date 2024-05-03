"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var operators_1 = require("rxjs/operators");
function shareReplay(configOrBufferSize, windowTime, scheduler) {
    if (configOrBufferSize && typeof configOrBufferSize === 'object') {
        return operators_1.shareReplay(configOrBufferSize)(this);
    }
    return operators_1.shareReplay(configOrBufferSize, windowTime, scheduler)(this);
}
exports.shareReplay = shareReplay;
//# sourceMappingURL=shareReplay.js.map