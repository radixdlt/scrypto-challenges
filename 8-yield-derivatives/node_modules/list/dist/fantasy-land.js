"use strict";
function __export(m) {
    for (var p in m) if (!exports.hasOwnProperty(p)) exports[p] = m[p];
}
Object.defineProperty(exports, "__esModule", { value: true });
var index_1 = require("./index");
__export(require("./index"));
var flOf = "fantasy-land/of";
var flEmpty = "fantasy-land/empty";
index_1.List.prototype["fantasy-land/equals"] = function (l) {
    return index_1.equals(this, l);
};
index_1.List.prototype["fantasy-land/map"] = function (f) {
    return index_1.map(f, this);
};
index_1.List.prototype[flOf] = index_1.of;
index_1.List[flOf] = index_1.List.prototype[flOf];
index_1.List.prototype["fantasy-land/ap"] = function (listF) {
    return index_1.ap(listF, this);
};
index_1.List.prototype["fantasy-land/chain"] = function (f) {
    return index_1.chain(f, this);
};
index_1.List.prototype["fantasy-land/filter"] = function (predicate) {
    return index_1.filter(predicate, this);
};
index_1.List.prototype[flEmpty] = function () {
    return index_1.empty();
};
index_1.List[flEmpty] = index_1.List.prototype[flEmpty];
index_1.List.prototype["fantasy-land/concat"] = function (right) {
    return index_1.concat(this, right);
};
index_1.List.prototype["fantasy-land/reduce"] = function (f, initial) {
    return index_1.foldl(f, initial, this);
};
index_1.List.prototype["fantasy-land/traverse"] = function (of, f) {
    return index_1.traverse(of, f, this);
};
