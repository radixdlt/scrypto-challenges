import { IMap } from "./IMap";
import { WithEquality } from "./Comparison";
import { Option } from "./Option";
import { HashSet } from "./HashSet";
import { Vector } from "./Vector";
import { LinkedList } from "./LinkedList";
import { inspect } from "./Value";
/**
 * A dictionary, mapping keys to values.
 * @param K the key type
 * @param V the value type
 */
export declare class HashMap<K, V> implements IMap<K, V> {
    private hamt;
    /**
     * @hidden
     */
    protected constructor(hamt: any);
    /**
     * The empty map.
     * @param K the key type
     * @param V the value type
     */
    static empty<K, V>(): HashMap<K, V>;
    /**
     * Build a HashMap from key-value pairs.
     *
     *     HashMap.of([1,"a"],[2,"b"])
     *
     */
    static of<K, V>(...entries: Array<[K & WithEquality, V]>): HashMap<K, V>;
    /**
     * Build a HashMap from an iterable containing key-value pairs.
     *
     *    HashMap.ofIterable(Vector.of<[number,string]>([1,"a"],[2,"b"]));
     */
    static ofIterable<K, V>(entries: Iterable<[K & WithEquality, V]>): HashMap<K, V>;
    /**
     * Build a HashMap from a javascript object literal representing
     * a dictionary. Note that the key type must always be string,
     * as that's the way it works in javascript.
     * Also note that entries with undefined values will be stripped
     * from the map.
     *
     *     HashMap.ofObjectDictionary<number>({a:1,b:2})
     *     => HashMap.of(["a",1],["b",2])
     */
    static ofObjectDictionary<V>(object: {
        [index: string]: V | undefined;
    }): HashMap<string, V>;
    /**
     * Curried predicate to find out whether the HashMap is empty.
     *
     *     Vector.of(HashMap.of([1,2]), HashMap.empty<number,number>())
     *         .filter(HashMap.isEmpty)
     *     => Vector.of(HashMap.empty<number,number>())
     */
    static isEmpty<K, V>(v: HashMap<K, V>): boolean;
    /**
     * Curried predicate to find out whether the HashMap is empty.
     *
     *     Vector.of(HashMap.of([1,2]), HashMap.empty<number,number>())
     *         .filter(HashMap.isNotEmpty)
     *     => Vector.of(HashMap.of([1,2]))
     */
    static isNotEmpty<K, V>(v: HashMap<K, V>): boolean;
    /**
     * Get the value for the key you give, if the key is present.
     */
    get(k: K & WithEquality): Option<V>;
    /**
     * Implementation of the Iterator interface.
     */
    [Symbol.iterator](): Iterator<[K, V]>;
    /**
     * @hidden
     */
    hasTrueEquality(): boolean;
    /**
     * Add a new entry in the map. If there was entry with the same
     * key, it will be overwritten.
     * @param k the key
     * @param v the value
     */
    put(k: K & WithEquality, v: V): HashMap<K, V>;
    /**
     * Return a new map with the key you give removed.
     */
    remove(k: K & WithEquality): HashMap<K, V>;
    /**
     * Add a new entry in the map; in case there was already an
     * entry with the same key, the merge function will be invoked
     * with the old and the new value to produce the value to take
     * into account.
     *
     * It is guaranteed that the merge function first parameter
     * will be the entry from this map, and the second parameter
     * from the map you give.
     * @param k the key
     * @param v the value
     * @param merge a function to merge old and new values in case of conflict.
     */
    putWithMerge(k: K & WithEquality, v: V, merge: (v1: V, v2: V) => V): HashMap<K, V>;
    /**
     * number of items in the map
     */
    length(): number;
    /**
     * If the collection contains a single element,
     * return Some of its value, otherwise return None.
     */
    single(): Option<[K, V]>;
    /**
     * true if the map is empty, false otherwise.
     */
    isEmpty(): boolean;
    /**
     * Get a Set containing all the keys in the map
     */
    keySet(): HashSet<K>;
    /**
     * Get an iterable containing all the values in the map
     * (can't return a set as we don't constrain map values
     * to have equality in the generics type)
     */
    valueIterable(): Iterable<V>;
    /**
     * Create a new map combining the entries of this map, and
     * the other map you give. In case an entry from this map
     * and the other map have the same key, the merge function
     * will be invoked to get a combined value.
     *
     * It is guaranteed that the merge function first parameter
     * will be the entry from this map, and the second parameter
     * from the map you give.
     * @param other another map to merge with this one
     * @param merge a merge function to combine two values
     *        in case two entries share the same key.
     */
    mergeWith(elts: Iterable<[K & WithEquality, V]>, merge: (v1: V, v2: V) => V): HashMap<K, V>;
    /**
     * Return a new map where each entry was transformed
     * by the mapper function you give. You return key,value
     * as pairs.
     */
    map<K2, V2>(fn: (k: K & WithEquality, v: V) => [K2 & WithEquality, V2]): HashMap<K2, V2>;
    /**
     * Return a new map where keys are the same as in this one,
     * but values are transformed
     * by the mapper function you give. You return key,value
     * as pairs.
     */
    mapValues<V2>(fn: (v: V) => V2): HashMap<K, V2>;
    /**
     * Call a function for element in the collection.
     */
    forEach(fun: (x: [K, V]) => void): HashMap<K, V>;
    /**
     * Calls the function you give for each item in the map,
     * your function returns a map, all the maps are
     * merged.
     */
    flatMap<K2, V2>(fn: (k: K, v: V) => Iterable<[K2 & WithEquality, V2]>): HashMap<K2, V2>;
    /**
     * Returns true if the predicate returns true for all the
     * elements in the collection.
     */
    allMatch(predicate: (k: K, v: V) => boolean): boolean;
    /**
     * Returns true if there the predicate returns true for any
     * element in the collection.
     */
    anyMatch(predicate: (k: K, v: V) => boolean): boolean;
    /**
     * Returns true if the item is in the collection,
     * false otherwise.
     */
    contains(val: [K & WithEquality, V & WithEquality]): boolean;
    /**
     * Returns true if there is item with that key in the collection,
     * false otherwise.
     *
     *     HashMap.of<number,string>([1,"a"],[2,"b"]).containsKey(1);
     *     => true
     *
     *     HashMap.of<number,string>([1,"a"],[2,"b"]).containsKey(3);
     *     => false
     */
    containsKey(key: K & WithEquality): boolean;
    /**
     * Call a predicate for each element in the collection,
     * build a new collection holding only the elements
     * for which the predicate returned true.
     */
    filter(predicate: (k: K, v: V) => boolean): HashMap<K, V>;
    /**
     * Search for an item matching the predicate you pass,
     * return Option.Some of that element if found,
     * Option.None otherwise.
     * We name the method findAny instead of find to emphasize
     * that there is not ordering in a hashset.
     *
     *     HashMap.of<number,string>([1,'a'],[2,'b'],[3,'c'])
     *         .findAny((k,v) => k>=2 && v === "c")
     *     => Option.of([3,'c'])
     *
     *     HashMap.of<number,string>([1,'a'],[2,'b'],[3,'c'])
     *         .findAny((k,v) => k>=3 && v === "b")
     *     => Option.none<[number,string]>()
     */
    findAny(predicate: (k: K, v: V) => boolean): Option<[K, V]>;
    /**
     * Call a predicate for each key in the collection,
     * build a new collection holding only the elements
     * for which the predicate returned true.
     *
     *     HashMap.of<number,string>([1,"a"],[2,"b"]).filterKeys(k=>k%2===0)
     *     => HashMap.of<number,string>([2,"b"])
     */
    filterKeys<U extends K>(fn: (v: K) => v is U): HashMap<U, V>;
    filterKeys(predicate: (k: K) => boolean): HashMap<K, V>;
    /**
     * Call a predicate for each value in the collection,
     * build a new collection holding only the elements
     * for which the predicate returned true.
     *
     *     HashMap.of<number,string>([1,"a"],[2,"ab"]).filterValues(v=>v.length>1)
     *     => HashMap.of<number,string>([2,"ab"])
     */
    filterValues<U extends V>(fn: (v: V) => v is U): HashMap<K, U>;
    filterValues(predicate: (k: V) => boolean): HashMap<K, V>;
    /**
     * Reduces the collection to a single value using the
     * associative binary function you give. Since the function
     * is associative, order of application doesn't matter.
     *
     * Example:
     *
     *     HashMap.of<number,string>([1,"a"],[2,"b"],[3,"c"])
     *      .fold([0,""], ([a,b],[c,d])=>[a+c, b>d?b:d])
     *     => [6,"c"]
     */
    fold(zero: [K, V], fn: (v1: [K, V], v2: [K, V]) => [K, V]): [K, V];
    /**
     * Reduces the collection to a single value.
     * Left-associative.
     * No guarantees for the order of items in a hashset!
     *
     * Example:
     *
     *     HashMap.of([1,"a"], [2,"bb"], [3,"ccc"])
     *     .foldLeft(0, (soFar,[item,val])=>soFar+val.length);
     *     => 6
     *
     * @param zero The initial value
     * @param fn A function taking the previous value and
     *           the current collection item, and returning
     *           an updated value.
     */
    foldLeft<U>(zero: U, fn: (soFar: U, cur: [K, V]) => U): U;
    /**
     * Reduces the collection to a single value.
     * Right-associative.
     * No guarantees for the order of items in a hashset!
     *
     * Example:
     *
     *     HashMap.of([1,"a"], [2,"bb"], [3,"ccc"])
     *     .foldRight(0, ([item,value],soFar)=>soFar+value.length);
     *     => 6
     *
     * @param zero The initial value
     * @param fn A function taking the current collection item and
     *           the previous value , and returning
     *           an updated value.
     */
    foldRight<U>(zero: U, fn: (cur: [K, V], soFar: U) => U): U;
    /**
     * Reduces the collection to a single value by repeatedly
     * calling the combine function.
     * No starting value. The order in which the elements are
     * passed to the combining function is undetermined.
     */
    reduce(combine: (v1: [K, V], v2: [K, V]) => [K, V]): Option<[K, V]>;
    /**
     * Convert to array.
     */
    toArray(): Array<[K, V]>;
    /**
     * Convert this map to a vector of key,value pairs.
     * Note that Map is already an iterable of key,value pairs!
     */
    toVector(): Vector<[K, V]>;
    /**
     * Convert this map to a list of key,value pairs.
     * Note that Map is already an iterable of key,value pairs!
     */
    toLinkedList(): LinkedList<[K, V]>;
    /**
     * Convert to a javascript object dictionary
     * You must provide a function to convert the
     * key to a string.
     *
     *     HashMap.of<string,number>(["a",1],["b",2])
     *         .toObjectDictionary(x=>x);
     *     => {a:1,b:2}
     */
    toObjectDictionary(keyConvert: (k: K) => string): {
        [index: string]: V;
    };
    /**
     * Convert to an ES6 Map.
     * You must provide a function to convert the
     * key to a string, number or boolean, because
     * with other types equality is not correctly
     * managed by JS.
     * https://stackoverflow.com/questions/29759480/how-to-customize-object-equality-for-javascript-set
     * https://esdiscuss.org/topic/maps-with-object-keys
     *
     *     HashMap.of<string,number>(["a",1],["b",2])
     *         .toJsMap(x=>x);
     *     => new Map([["a",1], ["b",2]])
     */
    toJsMap(keyConvert: (k: K) => string): Map<string, V>;
    toJsMap(keyConvert: (k: K) => number): Map<number, V>;
    toJsMap(keyConvert: (k: K) => boolean): Map<boolean, V>;
    /**
     * Transform this value to another value type.
     * Enables fluent-style programming by chaining calls.
     */
    transform<U>(converter: (x: HashMap<K, V>) => U): U;
    /**
     * Two objects are equal if they represent the same value,
     * regardless of whether they are the same object physically
     * in memory.
     */
    equals(other: IMap<K & WithEquality, V & WithEquality>): boolean;
    /**
     * Get a number for that object. Two different values
     * may get the same number, but one value must always get
     * the same number. The formula can impact performance.
     */
    hashCode(): number;
    toString(): string;
    [inspect](): string;
}
