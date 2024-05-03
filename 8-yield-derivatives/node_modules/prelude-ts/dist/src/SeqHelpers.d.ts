import { Option } from "./Option";
import { WithEquality, Ordering, ToOrderable } from "./Comparison";
import { HashMap } from "./HashMap";
import { Seq } from "./Seq";
import { Collection } from "./Collection";
import { Stream } from "./Stream";
/**
 * @hidden
 */
export declare function shuffle(array: any[]): any[];
/**
 * @hidden
 */
export declare function arrangeBy<T, K>(collection: Collection<T>, getKey: (v: T) => K & WithEquality): Option<HashMap<K, T>>;
/**
 * @hidden
 */
export declare function seqHasTrueEquality<T>(seq: Seq<T>): boolean;
/**
 * @hidden
 */
export declare function zipWithIndex<T>(seq: Seq<T>): Seq<[T, number]>;
/**
 * @hidden
 */
export declare function sortOn<T>(seq: Seq<T>, getKeys: Array<ToOrderable<T> | {
    desc: ToOrderable<T>;
}>): Seq<T>;
/**
 * @hidden
 */
export declare function distinctBy<T, U>(seq: Collection<T>, keyExtractor: (x: T) => U & WithEquality): Collection<T>;
/**
 * Utility function to help converting a value to string
 * util.inspect seems to depend on node.
 * @hidden
 */
export declare function toStringHelper(obj: any | null, options?: {
    quoteStrings: boolean;
}): string;
/**
 * @hidden
 */
export declare function minBy<T>(coll: Collection<T>, compare: (v1: T, v2: T) => Ordering): Option<T>;
/**
 * @hidden
 */
export declare function minOn<T>(coll: Collection<T>, getSortable: ToOrderable<T>): Option<T>;
/**
 * @hidden
 */
export declare function maxBy<T>(coll: Collection<T>, compare: (v1: T, v2: T) => Ordering): Option<T>;
/**
 * @hidden
 */
export declare function maxOn<T>(coll: Collection<T>, getSortable: ToOrderable<T>): Option<T>;
/**
 * @hidden
 */
export declare function sumOn<T>(coll: Collection<T>, getNumber: (v: T) => number): number;
/**
 * @hidden
 */
export declare function reduce<T>(coll: Collection<T>, combine: (v1: T, v2: T) => T): Option<T>;
/**
 * @hidden
 */
export declare function sliding<T>(seq: Seq<T>, count: number): Stream<Seq<T>>;
/**
 * @hidden
 */
export declare function removeAll<T>(seq: Seq<T>, elts: Iterable<T & WithEquality>): Seq<T>;
