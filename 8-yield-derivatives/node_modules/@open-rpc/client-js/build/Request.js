"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getNotifications = exports.getBatchRequests = exports.isNotification = void 0;
exports.isNotification = function (data) {
    return (data.request.id === undefined || data.request.id === null);
};
exports.getBatchRequests = function (data) {
    if (data instanceof Array) {
        return data.filter(function (datum) {
            var id = datum.request.request.id;
            return id !== null && id !== undefined;
        }).map(function (batchRequest) {
            return batchRequest.request;
        });
    }
    return [];
};
exports.getNotifications = function (data) {
    if (data instanceof Array) {
        return data.filter(function (datum) {
            return exports.isNotification(datum.request);
        }).map(function (batchRequest) {
            return batchRequest.request;
        });
    }
    if (exports.isNotification(data)) {
        return [data];
    }
    return [];
};
