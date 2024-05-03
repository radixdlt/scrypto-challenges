import { ISet, SortOnSpec, SortBySpec } from "./ISet";
import { Vector } from "./Vector";
import { HashMap } from "./HashMap";
import { LinkedList } from "./LinkedList";
import { Option } from "./Option";
import { WithEquality, Ordering, ToOrderable } from "./Comparison";
import { inspect } from "./Value";
/**
 * An unordered collection of values, where no two values
 * may be equal. A value can only be present once.
 * @param T the item type
 */
export declare class HashSet<T> implements ISet<T> {
    private hamt;
    /**
     * @hidden
     */
    protected constructor(hamt: any);
    /**
     * The empty hashset.
     * @param T the item type
     */
    static empty<T>(): HashSet<T>;
    /**
     * Build a hashset from any iterable, which means also
     * an array for instance.
     * @param T the item type
     */
    static ofIterable<T>(elts: Iterable<T & WithEquality>): HashSet<T>;
    /**
     * Build a hashset from a series of items (any number, as parameters)
     * @param T the item type
     */
    static of<T>(...arr: Array<T & WithEquality>): HashSet<T>;
    /**
     * Curried predicate to find out whether the HashSet is empty.
     *
     *     Vector.of(HashSet.of(1), HashSet.empty<number>())
     *         .filter(HashSet.isEmpty)
     *     => Vector.of(HashSet.empty<number>())
     */
    static isEmpty<T>(v: HashSet<T>): boolean;
    /**
     * Curried predicate to find out whether the HashSet is empty.
     *
     *     Vector.of(HashSet.of(1), HashSet.empty<number>())
     *         .filter(HashSet.isNotEmpty)
     *     => Vector.of(HashSet.of(1))
     */
    static isNotEmpty<T>(v: HashSet<T>): boolean;
    /**
     * Implementation of the Iterator interface.
     */
    [Symbol.iterator](): Iterator<T>;
    /**
     * Add an element to this set.
     */
    add(elt: T & WithEquality): HashSet<T>;
    private addAllArray;
    /**
     * Add multiple elements to this set.
     */
    addAll(elts: Iterable<T & WithEquality>): HashSet<T>;
    /**
     * Returns true if the element you give is present in
     * the set, false otherwise.
     */
    contains(elt: T & WithEquality): boolean;
    /**
     * Return a new collection where each element was transformed
     * by the mapper function you give.
     * The resulting set may be smaller than the source.
     */
    map<U>(mapper: (v: T) => U & WithEquality): HashSet<U>;
    /**
     * Apply the mapper function on every element of this collection.
     * The mapper function returns an Option; if the Option is a Some,
     * the value it contains is added to the result Collection, if it's
     * a None, the value is discarded.
     *
     *     HashSet.of(1,2,6).mapOption(x => x%2===0 ?
     *         Option.of(x+1) : Option.none<number>())
     *     => HashSet.of(3, 7)
     */
    mapOption<U>(mapper: (v: T) => Option<U & WithEquality>): HashSet<U>;
    /**
     * Call a function for element in the collection.
     */
    forEach(fun: (x: T) => void): HashSet<T>;
    /**
     * Calls the function you give for each item in the set,
     * your function returns a set, all the sets are
     * merged.
     */
    flatMap<U>(mapper: (v: T) => HashSet<U & WithEquality>): HashSet<U>;
    /**
     * Call a predicate for each element in the collection,
     * build a new collection holding only the elements
     * for which the predicate returned true.
     */
    filter<U extends T>(fn: (v: T) => v is U): HashSet<U>;
    filter(predicate: (v: T) => boolean): HashSet<T>;
    /**
     * Search for an item matching the predicate you pass,
     * return Option.Some of that element if found,
     * Option.None otherwise.
     * We name the method findAny instead of find to emphasize
     * that there is not ordering in a hashset.
     *
     *     HashSet.of(1,2,3).findAny(x => x>=3)
     *     => Option.of(3)
     *
     *     HashSet.of(1,2,3).findAny(x => x>=4)
     *     => Option.none<number>()
     */
    findAny(predicate: (v: T) => boolean): Option<T>;
    /**
     * Reduces the collection to a single value using the
     * associative binary function you give. Since the function
     * is associative, order of application doesn't matter.
     *
     * Example:
     *
     *     HashSet.of(1,2,3).fold(0, (a,b) => a + b);
     *     => 6
     */
    fold(zero: T, fn: (v1: T, v2: T) => T): T;
    /**
     * Reduces the collection to a single value.
     * Left-associative.
     * No guarantees for the order of items in a hashset!
     *
     * Example:
     *
     *     HashSet.of("a", "bb", "ccc").foldLeft(0, (soFar,item) => soFar+item.length);
     *     => 6
     *
     * @param zero The initial value
     * @param fn A function taking the previous value and
     *           the current collection item, and returning
     *           an updated value.
     */
    foldLeft<U>(zero: U, fn: (soFar: U, cur: T) => U): U;
    /**
     * Reduces the collection to a single value.
     * Right-associative.
     * No guarantees for the order of items in a hashset!
     *
     * Example:
     *
     *     HashSet.of("a", "bb", "ccc").foldRight(0, (item,soFar) => soFar+item.length);
     *     => 6
     *
     * @param zero The initial value
     * @param fn A function taking the current collection item and
     *           the previous value , and returning
     *           an updated value.
     */
    foldRight<U>(zero: U, fn: (cur: T, soFar: U) => U): U;
    /**
     * Converts this set to an array. Since a Set is not ordered
     * and since this method returns a JS array, it can be awkward
     * to get an array sorted in the way you'd like. So you can pass
     * an optional sorting function too.
     *
     *     HashSet.of(1,2,3).toArray().sort()
     *     => [1,2,3]
     *
     *     HashSet.of(1,2,3).toArray({sortOn:x=>x})
     *     => [1,2,3]
     *
     *     HashSet.of(1,2,3).toArray({sortBy:(x,y)=>x-y})
     *     => [1,2,3]
     *
     * You can also pass an array in sortOn, listing lambdas to
     * several fields to sort by those fields, and also {desc:lambda}
     * to sort by some fields descending.
     */
    toArray(sort?: SortOnSpec<T> | SortBySpec<T>): Array<T & WithEquality>;
    /**
     * Converts this set to an vector
     */
    toVector(): Vector<T & WithEquality>;
    /**
     * Converts this set to an list
     */
    toLinkedList(): LinkedList<T & WithEquality>;
    /**
     * Returns the number of elements in the set.
     */
    length(): number;
    /**
     * If the collection contains a single element,
     * return Some of its value, otherwise return None.
     */
    single(): Option<T>;
    /**
     * true if the set is empty, false otherwise.
     */
    isEmpty(): boolean;
    /**
     * Returns a new Set containing the difference
     * between this set and the other Set passed as parameter.
     * also see [[HashSet.intersect]]
     */
    diff(elts: ISet<T & WithEquality>): HashSet<T>;
    /**
     * Returns a new Set containing the intersection
     * of this set and the other Set passed as parameter
     * (the elements which are common to both sets)
     * also see [[HashSet.diff]]
     */
    intersect(other: ISet<T & WithEquality>): HashSet<T>;
    isSubsetOf(other: ISet<T & WithEquality>): boolean;
    /**
     * Returns a new set with the element you give removed
     * if it was present in the set.
     */
    remove(elt: T & WithEquality): HashSet<T>;
    /**
     * Returns a new set with all the elements of the current
     * Set, minus the elements of the iterable you give as a parameter.
     * If you call this function with a HashSet as parameter,
     * rather call 'diff', as it'll be faster.
     */
    removeAll(elts: Iterable<T & WithEquality>): HashSet<T>;
    /**
     * Returns true if the predicate returns true for all the
     * elements in the collection.
     */
    allMatch<U extends T>(predicate: (v: T) => v is U): this is HashSet<U>;
    allMatch(predicate: (v: T) => boolean): boolean;
    /**
     * Returns true if there the predicate returns true for any
     * element in the collection.
     */
    anyMatch(predicate: (v: T) => boolean): boolean;
    /**
     * Group elements in the collection using a classifier function.
     * Elements are then organized in a map. The key is the value of
     * the classifier, and in value we get the list of elements
     * matching that value.
     *
     * also see [[HashSet.arrangeBy]]
     */
    groupBy<C>(classifier: (v: T) => C & WithEquality): HashMap<C, HashSet<T>>;
    /**
     * Matches each element with a unique key that you extract from it.
     * If the same key is present twice, the function will return None.
     *
     * also see [[HashSet.groupBy]]
     */
    arrangeBy<K>(getKey: (v: T) => K & WithEquality): Option<HashMap<K, T>>;
    /**
     * Returns a pair of two sets; the first one
     * will only contain the items from this sets for
     * which the predicate you give returns true, the second
     * will only contain the items from this collection where
     * the predicate returns false.
     *
     *     HashSet.of(1,2,3,4).partition(x => x%2===0)
     *     => [HashSet.of(2,4), HashSet.of(1,3)]
     */
    partition<U extends T>(predicate: (v: T) => v is U): [HashSet<U>, HashSet<Exclude<T, U>>];
    partition(predicate: (x: T) => boolean): [HashSet<T>, HashSet<T>];
    /**
     * Reduces the collection to a single value by repeatedly
     * calling the combine function.
     * No starting value. The order in which the elements are
     * passed to the combining function is undetermined.
     */
    reduce(combine: (v1: T, v2: T) => T): Option<T>;
    /**
     * Compare values in the collection and return the smallest element.
     * Returns Option.none if the collection is empty.
     *
     * also see [[HashSet.minOn]]
     */
    minBy(compare: (v1: T, v2: T) => Ordering): Option<T>;
    /**
     * Call the function you give for each value in the collection
     * and return the element for which the result was the smallest.
     * Returns Option.none if the collection is empty.
     *
     * also see [[HashSet.minBy]]
     */
    minOn(getOrderable: ToOrderable<T>): Option<T>;
    /**
     * Compare values in the collection and return the largest element.
     * Returns Option.none if the collection is empty.
     *
     * also see [[HashSet.maxOn]]
     */
    maxBy(compare: (v1: T, v2: T) => Ordering): Option<T>;
    /**
     * Call the function you give for each value in the collection
     * and return the element for which the result was the largest.
     * Returns Option.none if the collection is empty.
     *
     * also see [[HashSet.maxBy]]
     */
    maxOn(getOrderable: ToOrderable<T>): Option<T>;
    /**
     * Call the function you give for each element in the collection
     * and sum all the numbers, return that sum.
     * Will return 0 if the collection is empty.
     *
     *     HashSet.of(1,2,3).sumOn(x=>x)
     *     => 6
     */
    sumOn(getNumber: (v: T) => number): number;
    /**
     * Transform this value to another value type.
     * Enables fluent-style programming by chaining calls.
     */
    transform<U>(converter: (x: HashSet<T>) => U): U;
    /**
     * Convert to an ES6 Set.
     * You must provide a function to convert the
     * key to a string, number or boolean, because
     * with other types equality is not correctly
     * managed by JS.
     * https://stackoverflow.com/questions/29759480/how-to-customize-object-equality-for-javascript-set
     * https://esdiscuss.org/topic/maps-with-object-keys
     *
     *     HashSet.of("a", "b").toJsSet(x=>x);
     *     => new Set(["a", "b"])
     */
    toJsSet(keyConvert: (k: T) => string): Set<string>;
    toJsSet(keyConvert: (k: T) => number): Set<number>;
    toJsSet(keyConvert: (k: T) => boolean): Set<boolean>;
    /**
     * Two objects are equal if they represent the same value,
     * regardless of whether they are the same object physically
     * in memory.
     */
    equals(other: HashSet<T>): boolean;
    /**
     * Get a number for that object. Two different values
     * may get the same number, but one value must always get
     * the same number. The formula can impact performance.
     */
    hashCode(): number;
    /**
     * Get a human-friendly string representation of that value.
     *
     * Also see [[HashSet.mkString]]
     */
    toString(): string;
    [inspect](): string;
    /**
     * Joins elements of the collection by a separator.
     * Example:
     *
     *     HashSet.of(1,2,3).mkString(", ")
     *     => "1, 2, 3"
     *
     * (of course, order is not guaranteed)
     */
    mkString(separator: string): string;
}
