import { Value, inspect } from "./Value";
import { Option } from "./Option";
import { Vector } from "./Vector";
import { ConsLinkedList } from "./LinkedList";
import { WithEquality } from "./Comparison";
/**
 * Contains a pair of two values, which may or may not have the same type.
 * Compared to the builtin typescript [T,U] type, we get equality semantics
 * and helper functions (like mapping and so on).
 * @param T the first item type
 * @param U the second item type
 */
export declare class Tuple2<T, U> implements Value {
    private _fst;
    private _snd;
    private constructor();
    /**
     * Build a pair of value from both values.
     */
    static of<T, U>(fst: T, snd: U): Tuple2<T, U>;
    /**
     * Build a tuple2 from javascript array. Compared to [[Tuple2.ofPair]],
     * it checks the length of the array and will return [[None]] in case
     * the length isn't two. However the types of the elements aren't checked.
     */
    static ofArray<T, U>(pair: Array<T | U>): Option<Tuple2<T, U>>;
    /**
     * Build a tuple2 from javascript pair.
     * Also see [[Tuple2.ofArray]]
     */
    static ofPair<T, U>(pair: [T, U]): Tuple2<T, U>;
    /**
     * @hidden
     */
    hasTrueEquality(): boolean;
    /**
     * Extract the first value from the pair
     */
    fst(): T;
    /**
     * Extract the second value from the pair
     */
    snd(): U;
    /**
     * Maps the first component of this tuple to a new value.
     */
    map1<V>(fn: (v: T) => V): Tuple2<V, U>;
    /**
     * Maps the second component of this tuple to a new value.
     */
    map2<V>(fn: (v: U) => V): Tuple2<T, V>;
    /**
     * Make a new tuple by mapping both values inside this one.
     */
    map<T1, U1>(fn: (a: T, b: U) => Tuple2<T1, U1>): Tuple2<T1, U1>;
    /**
     * Transform this value to another value type.
     * Enables fluent-style programming by chaining calls.
     */
    transform<V>(converter: (x: Tuple2<T, U>) => V): V;
    /**
     * Two objects are equal if they represent the same value,
     * regardless of whether they are the same object physically
     * in memory.
     */
    equals(other: Tuple2<T & WithEquality, U & WithEquality>): boolean;
    /**
     * Get a number for that object. Two different values
     * may get the same number, but one value must always get
     * the same number. The formula can impact performance.
     */
    hashCode(): number;
    /**
     * Convert the tuple to a javascript pair.
     * Compared to [[Tuple2.toArray]], it behaves the
     * same at runtime, the only difference is the
     * typescript type definition.
     */
    toPair(): [T, U];
    /**
     * Convert the tuple to a javascript array.
     * Compared to [[Tuple2.toPair]], it behaves the
     * same at runtime, the only difference is the
     * typescript type definition.
     */
    toArray(): Array<T | U>;
    /**
     * Convert the tuple to a vector.
     */
    toVector(): Vector<T | U>;
    /**
     * Convert the tuple to a linked list.
     */
    toLinkedList(): ConsLinkedList<T | U>;
    /**
     * Get a human-friendly string representation of that value.
     */
    toString(): string;
    /**
     * Used by the node REPL to display values.
     * Most of the time should be the same as toString()
     */
    [inspect](): string;
}
