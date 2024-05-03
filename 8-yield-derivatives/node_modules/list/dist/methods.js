"use strict";
function __export(m) {
    for (var p in m) if (!exports.hasOwnProperty(p)) exports[p] = m[p];
}
Object.defineProperty(exports, "__esModule", { value: true });
var fantasy_land_1 = require("./fantasy-land");
var L = require("./fantasy-land");
__export(require("./index"));
fantasy_land_1.List.prototype.append = function (value) {
    return L.append(value, this);
};
fantasy_land_1.List.prototype.intersperse = function (separator) {
    return L.intersperse(separator, this);
};
fantasy_land_1.List.prototype.nth = function (index) {
    return L.nth(index, this);
};
fantasy_land_1.List.prototype.empty = function () {
    return L.empty();
};
fantasy_land_1.List.prototype.of = function (b) {
    return L.of(b);
};
fantasy_land_1.List.prototype.prepend = function (value) {
    return L.prepend(value, this);
};
fantasy_land_1.List.prototype.append = function (value) {
    return L.append(value, this);
};
fantasy_land_1.List.prototype.first = function () {
    return L.first(this);
};
fantasy_land_1.List.prototype.head = fantasy_land_1.List.prototype.first;
fantasy_land_1.List.prototype.last = function () {
    return L.last(this);
};
fantasy_land_1.List.prototype.map = function (f) {
    return L.map(f, this);
};
fantasy_land_1.List.prototype.pluck = function (key) {
    return L.pluck(key, this);
};
fantasy_land_1.List.prototype.foldl = function foldl(f, initial) {
    return L.foldl(f, initial, this);
};
fantasy_land_1.List.prototype.reduce = fantasy_land_1.List.prototype.foldl;
fantasy_land_1.List.prototype.scan = function scan(f, initial) {
    return L.scan(f, initial, this);
};
fantasy_land_1.List.prototype.foldr = function (f, initial) {
    return L.foldr(f, initial, this);
};
fantasy_land_1.List.prototype.reduceRight = fantasy_land_1.List.prototype.foldr;
fantasy_land_1.List.prototype.foldlWhile = function foldlWhile(predicate, f, initial) {
    return L.foldlWhile(predicate, f, initial, this);
};
fantasy_land_1.List.prototype.reduceWhile = fantasy_land_1.List.prototype.foldlWhile;
fantasy_land_1.List.prototype.traverse = function (of, f) {
    return L.traverse(of, f, this);
};
fantasy_land_1.List.prototype.sequence = function (of) {
    return L.sequence(of, this);
};
fantasy_land_1.List.prototype.forEach = function (callback) {
    return L.forEach(callback, this);
};
fantasy_land_1.List.prototype.filter = function (predicate) {
    return L.filter(predicate, this);
};
fantasy_land_1.List.prototype.reject = function (predicate) {
    return L.reject(predicate, this);
};
fantasy_land_1.List.prototype.partition = function (predicate) {
    return L.partition(predicate, this);
};
fantasy_land_1.List.prototype.join = function (separator) {
    return L.join(separator, this);
};
fantasy_land_1.List.prototype.ap = function (listF) {
    return L.ap(listF, this);
};
fantasy_land_1.List.prototype.flatten = function () {
    return L.flatten(this);
};
fantasy_land_1.List.prototype.flatMap = function (f) {
    return L.flatMap(f, this);
};
fantasy_land_1.List.prototype.chain = fantasy_land_1.List.prototype.flatMap;
fantasy_land_1.List.prototype.every = function (predicate) {
    return L.every(predicate, this);
};
fantasy_land_1.List.prototype.some = function (predicate) {
    return L.some(predicate, this);
};
fantasy_land_1.List.prototype.none = function (predicate) {
    return L.none(predicate, this);
};
fantasy_land_1.List.prototype.indexOf = function (element) {
    return L.indexOf(element, this);
};
fantasy_land_1.List.prototype.lastIndexOf = function (element) {
    return L.lastIndexOf(element, this);
};
fantasy_land_1.List.prototype.find = function find(predicate) {
    return L.find(predicate, this);
};
fantasy_land_1.List.prototype.findLast = function findLast(predicate) {
    return L.findLast(predicate, this);
};
fantasy_land_1.List.prototype.findIndex = function (predicate) {
    return L.findIndex(predicate, this);
};
fantasy_land_1.List.prototype.includes = function (element) {
    return L.includes(element, this);
};
fantasy_land_1.List.prototype.equals = function (secondList) {
    return L.equals(this, secondList);
};
fantasy_land_1.List.prototype.equalsWith = function (f, secondList) {
    return L.equalsWith(f, this, secondList);
};
fantasy_land_1.List.prototype.concat = function (right) {
    return L.concat(this, right);
};
fantasy_land_1.List.prototype.update = function (index, a) {
    return L.update(index, a, this);
};
fantasy_land_1.List.prototype.adjust = function (index, f) {
    return L.adjust(index, f, this);
};
fantasy_land_1.List.prototype.slice = function (from, to) {
    return L.slice(from, to, this);
};
fantasy_land_1.List.prototype.take = function (n) {
    return L.take(n, this);
};
fantasy_land_1.List.prototype.takeWhile = function (predicate) {
    return L.takeWhile(predicate, this);
};
fantasy_land_1.List.prototype.takeLast = function (n) {
    return L.takeLast(n, this);
};
fantasy_land_1.List.prototype.takeLastWhile = function (predicate) {
    return L.takeLastWhile(predicate, this);
};
fantasy_land_1.List.prototype.splitAt = function (index) {
    return L.splitAt(index, this);
};
fantasy_land_1.List.prototype.splitWhen = function (predicate) {
    return L.splitWhen(predicate, this);
};
fantasy_land_1.List.prototype.splitEvery = function (size) {
    return L.splitEvery(size, this);
};
fantasy_land_1.List.prototype.remove = function (from, amount) {
    return L.remove(from, amount, this);
};
fantasy_land_1.List.prototype.drop = function (n) {
    return L.drop(n, this);
};
fantasy_land_1.List.prototype.dropWhile = function (predicate) {
    return L.dropWhile(predicate, this);
};
fantasy_land_1.List.prototype.dropRepeats = function () {
    return L.dropRepeats(this);
};
fantasy_land_1.List.prototype.dropRepeatsWith = function (predicate) {
    return L.dropRepeatsWith(predicate, this);
};
fantasy_land_1.List.prototype.dropLast = function (n) {
    return L.dropLast(n, this);
};
fantasy_land_1.List.prototype.pop = function () {
    return L.pop(this);
};
fantasy_land_1.List.prototype.tail = function () {
    return L.tail(this);
};
fantasy_land_1.List.prototype.toArray = function () {
    return L.toArray(this);
};
fantasy_land_1.List.prototype.insert = function (index, element) {
    return L.insert(index, element, this);
};
fantasy_land_1.List.prototype.insertAll = function (index, elements) {
    return L.insertAll(index, elements, this);
};
fantasy_land_1.List.prototype.reverse = function () {
    return L.reverse(this);
};
fantasy_land_1.List.prototype.backwards = function () {
    return L.backwards(this);
};
fantasy_land_1.List.prototype.zipWith = function (f, bs) {
    return L.zipWith(f, this, bs);
};
fantasy_land_1.List.prototype.zip = function (bs) {
    return L.zip(this, bs);
};
fantasy_land_1.List.prototype.sort = function () {
    return L.sort(this);
};
fantasy_land_1.List.prototype.sortWith = function (comparator) {
    return L.sortWith(comparator, this);
};
fantasy_land_1.List.prototype.sortBy = function (f) {
    return L.sortBy(f, this);
};
fantasy_land_1.List.prototype.group = function () {
    return L.group(this);
};
fantasy_land_1.List.prototype.groupWith = function (f) {
    return L.groupWith(f, this);
};
fantasy_land_1.List.prototype.isEmpty = function () {
    return L.isEmpty(this);
};
