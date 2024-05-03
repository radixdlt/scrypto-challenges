"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.mapTo = void 0;
var lift_1 = require("../util/lift");
var OperatorSubscriber_1 = require("./OperatorSubscriber");
function mapTo(value) {
    return lift_1.operate(function (source, subscriber) {
        source.subscribe(new OperatorSubscriber_1.OperatorSubscriber(subscriber, function () { return subscriber.next(value); }));
    });
}
exports.mapTo = mapTo;
//# sourceMappingURL=mapTo.js.map