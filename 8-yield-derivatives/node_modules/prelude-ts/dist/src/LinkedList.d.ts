/**
 * A sequence of values, organized in-memory as a strict linked list.
 * Each element has an head (value) and a tail (the rest of the list).
 *
 * The code is organized through the class [[EmptyLinkedList]] (empty list
 * or tail), the class [[ConsLinkedList]] (list value and pointer to next),
 * and the type alias [[LinkedList]] (empty or cons).
 *
 * Finally, "static" functions on Option are arranged in the class
 * [[LinkedListStatic]] and are accessed through the global constant LinkedList.
 *
 * Random access is expensive, appending is expensive, prepend or getting
 * the tail of the list is very cheap.
 * If you often need random access you should rather use [[Vector]].
 * Avoid appending at the end of the list in a loop, prefer prepending and
 * then reversing the list.
 *
 * Examples:
 *
 *     LinkedList.of(1,2,3);
 *     LinkedList.of(1,2,3).map(x => x*2).last();
 */
import { Option, Some, None } from "./Option";
import { Vector } from "./Vector";
import { WithEquality, Ordering, ToOrderable } from "./Comparison";
import { inspect } from "./Value";
import { HashMap } from "./HashMap";
import { HashSet } from "./HashSet";
import { Seq, IterableArray } from "./Seq";
import { Stream } from "./Stream";
/**
 * Holds the "static methods" for [[LinkedList]]
 */
export declare class LinkedListStatic {
    /**
     * The empty stream
     */
    empty<T>(): LinkedList<T>;
    /**
     * Create a LinkedList with the elements you give.
     */
    of<T>(elt: T, ...elts: T[]): ConsLinkedList<T>;
    of<T>(...elts: T[]): LinkedList<T>;
    /**
     * Build a stream from any iterable, which means also
     * an array for instance.
     * @param T the item type
     */
    ofIterable<T>(elts: Iterable<T>): LinkedList<T>;
    /**
     * Curried type guard for LinkedList.
     * Sometimes needed also due to https://github.com/Microsoft/TypeScript/issues/20218
     *
     *     Vector.of(LinkedList.of(1), LinkedList.empty<number>())
     *         .filter(LinkedList.isEmpty)
     *     => Vector.of(LinkedList.empty<number>())
     */
    isEmpty<T>(l: LinkedList<T>): l is EmptyLinkedList<T>;
    /**
     * Curried type guard for LinkedList.
     * Sometimes needed also due to https://github.com/Microsoft/TypeScript/issues/20218
     *
     *     Vector.of(Stream.of(1), Stream.empty<number>())
     *         .filter(Stream.isNotEmpty)
     *         .map(s => s.head().get()+1)
     *     => Vector.of(2)
     */
    isNotEmpty<T>(l: LinkedList<T>): l is ConsLinkedList<T>;
    /**
     * Dual to the foldRight function. Build a collection from a seed.
     * Takes a starting element and a function.
     * It applies the function on the starting element; if the
     * function returns None, it stops building the list, if it
     * returns Some of a pair, it adds the first element to the result
     * and takes the second element as a seed to keep going.
     *
     *     LinkedList.unfoldRight(
     *          10, x=>Option.of(x)
     *              .filter(x => x!==0)
     *              .map<[number,number]>(x => [x,x-1]))
     *     => LinkedList.of(10, 9, 8, 7, 6, 5, 4, 3, 2, 1)
     */
    unfoldRight<T, U>(seed: T, fn: (x: T) => Option<[U, T]>): LinkedList<U>;
    /**
     * Combine any number of iterables you give in as
     * parameters to produce a new collection which combines all,
     * in tuples. For instance:
     *
     *     LinkedList.zip(LinkedList.of(1,2,3), ["a","b","c"], Vector.of(8,9,10))
     *     => LinkedList.of([1,"a",8], [2,"b",9], [3,"c",10])
     *
     * The result collection will have the length of the shorter
     * of the input iterables. Extra elements will be discarded.
     *
     * Also see the non-static version [[ConsLinkedList.zip]], which only combines two
     * collections.
     * @param A A is the type of the tuple that'll be generated
     *          (`[number,string,number]` for the code sample)
     */
    zip<A extends any[]>(...iterables: IterableArray<A>): LinkedList<A>;
}
/**
 * The LinkedList constant allows to call the LinkedList "static" methods
 */
export declare const LinkedList: LinkedListStatic;
/**
 * A LinkedList is either [[EmptyLinkedList]] or [[ConsLinkedList]]
 * "static methods" available through [[LinkedListStatic]]
 * @param T the item type
 */
export declare type LinkedList<T> = EmptyLinkedList<T> | ConsLinkedList<T>;
/**
 * EmptyLinkedList is the empty linked list; every non-empty
 * linked list also has a pointer to an empty linked list
 * after its last element.
 * "static methods" available through [[LinkedListStatic]]
 * @param T the item type
 */
export declare class EmptyLinkedList<T> implements Seq<T> {
    /**
     * @hidden
     */
    hasTrueEquality(): boolean;
    /**
     * Implementation of the Iterator interface.
     */
    [Symbol.iterator](): Iterator<T>;
    /**
     * @hidden
     */
    readonly className: "EmptyLinkedList";
    /**
     * View this Some a as LinkedList. Useful to help typescript type
     * inference sometimes.
     */
    asLinkedList(): LinkedList<T>;
    /**
     * Get the length of the collection.
     */
    length(): number;
    /**
     * If the collection contains a single element,
     * return Some of its value, otherwise return None.
     */
    single(): Option<T>;
    /**
     * true if the collection is empty, false otherwise.
     */
    isEmpty(): this is EmptyLinkedList<T>;
    /**
     * Get the first value of the collection, if any.
     * In this case the list is empty, so returns Option.none
     */
    head(): None<T>;
    /**
     * Get all the elements in the collection but the first one.
     * If the collection is empty, return None.
     */
    tail(): Option<LinkedList<T>>;
    /**
     * Get the last value of the collection, if any.
     * returns Option.Some if the collection is not empty,
     * Option.None if it's empty.
     */
    last(): Option<T>;
    /**
     * Retrieve the element at index idx.
     * Returns an option because the collection may
     * contain less elements than the index.
     *
     * Careful this is going to have poor performance
     * on LinkedList, which is not a good data structure
     * for random access!
     */
    get(idx: number): Option<T>;
    /**
     * Search for an item matching the predicate you pass,
     * return Option.Some of that element if found,
     * Option.None otherwise.
     */
    find(predicate: (v: T) => boolean): Option<T>;
    /**
     * Returns true if the item is in the collection,
     * false otherwise.
     */
    contains(v: T & WithEquality): boolean;
    /**
     * Return a new stream keeping only the first n elements
     * from this stream.
     */
    take(n: number): LinkedList<T>;
    /**
     * Returns a new collection, discarding the elements
     * after the first element which fails the predicate.
     */
    takeWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection, discarding the elements
     * after the first element which fails the predicate,
     * but starting from the end of the collection.
     *
     *     LinkedList.of(1,2,3,4).takeRightWhile(x => x > 2)
     *     => LinkedList.of(3,4)
     */
    takeRightWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection with the first
     * n elements discarded.
     * If the collection has less than n elements,
     * returns the empty collection.
     */
    drop(n: number): LinkedList<T>;
    /**
     * Returns a new collection, discarding the first elements
     * until one element fails the predicate. All elements
     * after that point are retained.
     */
    dropWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection with the last
     * n elements discarded.
     * If the collection has less than n elements,
     * returns the empty collection.
     */
    dropRight(n: number): LinkedList<T>;
    /**
     * Returns a new collection, discarding the last elements
     * until one element fails the predicate. All elements
     * before that point are retained.
     */
    dropRightWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Reduces the collection to a single value using the
     * associative binary function you give. Since the function
     * is associative, order of application doesn't matter.
     *
     * Example:
     *
     *     LinkedList.of(1,2,3).fold(0, (a,b) => a + b);
     *     => 6
     */
    fold(zero: T, fn: (v1: T, v2: T) => T): T;
    /**
     * Reduces the collection to a single value.
     * Left-associative.
     *
     * Example:
     *
     *     Vector.of("a", "b", "c").foldLeft("!", (xs,x) => x+xs);
     *     => "cba!"
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
     *
     * Example:
     *
     *     Vector.of("a", "b", "c").foldRight("!", (x,xs) => xs+x);
     *     => "!cba"
     *
     * @param zero The initial value
     * @param fn A function taking the current collection item and
     *           the previous value , and returning
     *           an updated value.
     */
    foldRight<U>(zero: U, fn: (cur: T, soFar: U) => U): U;
    /**
     * Combine this collection with the collection you give in
     * parameter to produce a new collection which combines both,
     * in pairs. For instance:
     *
     *     Vector.of(1,2,3).zip(["a","b","c"])
     *     => Vector.of([1,"a"], [2,"b"], [3,"c"])
     *
     * The result collection will have the length of the shorter
     * of both collections. Extra elements will be discarded.
     *
     * Also see [[LinkedListStatic.zip]] (static version which can more than two
     * iterables)
     */
    zip<U>(other: Iterable<U>): LinkedList<[T, U]>;
    /**
     * Combine this collection with the index of the elements
     * in it. Handy if you need the index when you map on
     * the collection for instance:
     *
     *     LinkedList.of("a","b").zipWithIndex().map(([v,idx]) => v+idx);
     *     => LinkedList.of("a0", "b1")
     */
    zipWithIndex(): LinkedList<[T, number]>;
    /**
     * Reverse the collection. For instance:
     *
     *     LinkedList.of(1,2,3).reverse();
     *     => LinkedList.of(3,2,1)
     */
    reverse(): LinkedList<T>;
    /**
     * Takes a predicate; returns a pair of collections.
     * The first one is the longest prefix of this collection
     * which satisfies the predicate, and the second collection
     * is the remainder of the collection.
     *
     *    LinkedList.of(1,2,3,4,5,6).span(x => x <3)
     *    => [LinkedList.of(1,2), LinkedList.of(3,4,5,6)]
     */
    span(predicate: (x: T) => boolean): [LinkedList<T>, LinkedList<T>];
    /**
     * Split the collection at a specific index.
     *
     *     LinkedList.of(1,2,3,4,5).splitAt(3)
     *     => [LinkedList.of(1,2,3), LinkedList.of(4,5)]
     */
    splitAt(index: number): [LinkedList<T>, LinkedList<T>];
    /**
     * Returns a pair of two collections; the first one
     * will only contain the items from this collection for
     * which the predicate you give returns true, the second
     * will only contain the items from this collection where
     * the predicate returns false.
     *
     *     LinkedList.of(1,2,3,4).partition(x => x%2===0)
     *     => [LinkedList.of(2,4),LinkedList.of(1,3)]
     */
    partition<U extends T>(predicate: (v: T) => v is U): [LinkedList<U>, LinkedList<Exclude<T, U>>];
    partition(predicate: (x: T) => boolean): [LinkedList<T>, LinkedList<T>];
    /**
     * Group elements in the collection using a classifier function.
     * Elements are then organized in a map. The key is the value of
     * the classifier, and in value we get the list of elements
     * matching that value.
     *
     * also see [[ConsLinkedList.arrangeBy]]
     */
    groupBy<C>(classifier: (v: T) => C & WithEquality): HashMap<C, LinkedList<T>>;
    /**
     * Matches each element with a unique key that you extract from it.
     * If the same key is present twice, the function will return None.
     *
     * also see [[ConsLinkedList.groupBy]]
     */
    arrangeBy<K>(getKey: (v: T) => K & WithEquality): Option<HashMap<K, T>>;
    /**
     * Randomly reorder the elements of the collection.
     */
    shuffle(): LinkedList<T>;
    /**
     * Append an element at the end of this LinkedList.
     * Warning: appending in a loop on a linked list is going
     * to be very slow!
     */
    append(v: T): LinkedList<T>;
    appendAll(elts: Iterable<T>): LinkedList<T>;
    /**
     * Remove multiple elements from a LinkedList
     *
     *     LinkedList.of(1,2,3,4,3,2,1).removeAll([2,4])
     *     => LinkedList.of(1,3,3,1)
     */
    removeAll(elts: Iterable<T & WithEquality>): LinkedList<T>;
    /**
     * Removes the first element matching the predicate
     * (use [[Seq.filter]] to remove all elements matching a predicate)
     */
    removeFirst(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Prepend an element at the beginning of the collection.
     */
    prepend(elt: T): LinkedList<T>;
    /**
     * Prepend multiple elements at the beginning of the collection.
     */
    prependAll(elt: Iterable<T>): LinkedList<T>;
    /**
     * Return a new collection where each element was transformed
     * by the mapper function you give.
     */
    map<U>(mapper: (v: T) => U): LinkedList<U>;
    /**
     * Apply the mapper function on every element of this collection.
     * The mapper function returns an Option; if the Option is a Some,
     * the value it contains is added to the result Collection, if it's
     * a None, the value is discarded.
     *
     *     LinkedList.of(1,2,6).mapOption(x => x%2===0 ?
     *         Option.of(x+1) : Option.none<number>())
     *     => LinkedList.of(3, 7)
     */
    mapOption<U>(mapper: (v: T) => Option<U>): LinkedList<U>;
    /**
     * Calls the function you give for each item in the collection,
     * your function returns a collection, all the collections are
     * concatenated.
     * This is the monadic bind.
     */
    flatMap<U>(mapper: (v: T) => LinkedList<U>): LinkedList<U>;
    /**
     * Returns true if the predicate returns true for all the
     * elements in the collection.
     */
    allMatch<U extends T>(predicate: (v: T) => v is U): this is LinkedList<U>;
    allMatch(predicate: (v: T) => boolean): boolean;
    /**
     * Returns true if there the predicate returns true for any
     * element in the collection.
     */
    anyMatch(predicate: (v: T) => boolean): boolean;
    /**
     * Call a predicate for each element in the collection,
     * build a new collection holding only the elements
     * for which the predicate returned true.
     */
    filter<U extends T>(predicate: (v: T) => v is U): LinkedList<U>;
    filter(predicate: (v: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection with elements
     * sorted according to the comparator you give.
     *
     *     const activityOrder = ["Writer", "Actor", "Director"];
     *     LinkedList.of({name:"George", activity: "Director"}, {name:"Robert", activity: "Actor"})
     *         .sortBy((p1,p2) => activityOrder.indexOf(p1.activity) - activityOrder.indexOf(p2.activity));
     *     => LinkedList.of({"name":"Robert","activity":"Actor"}, {"name":"George","activity":"Director"})
     *
     * also see [[ConsLinkedList.sortOn]]
     */
    sortBy(compare: (v1: T, v2: T) => Ordering): LinkedList<T>;
    /**
     * Give a function associating a number or a string with
     * elements from the collection, and the elements
     * are sorted according to that value.
     *
     *     LinkedList.of({a:3,b:"b"},{a:1,b:"test"},{a:2,b:"a"}).sortOn(elt=>elt.a)
     *     => LinkedList.of({a:1,b:"test"},{a:2,b:"a"},{a:3,b:"b"})
     *
     * You can also sort by multiple criteria, and request 'descending'
     * sorting:
     *
     *     LinkedList.of({a:1,b:"b"},{a:1,b:"test"},{a:2,b:"a"}).sortOn(elt=>elt.a,{desc:elt=>elt.b})
     *     => LinkedList.of({a:1,b:"test"},{a:1,b:"b"},{a:2,b:"a"})
     *
     * also see [[ConsLinkedList.sortBy]]
     */
    sortOn(...getKeys: Array<ToOrderable<T> | {
        desc: ToOrderable<T>;
    }>): LinkedList<T>;
    /**
     * Remove duplicate items; elements are mapped to keys, those
     * get compared.
     *
     *     LinkedList.of(1,1,2,3,2,3,1).distinctBy(x => x)
     *     => LinkedList.of(1,2,3)
     */
    distinctBy<U>(keyExtractor: (x: T) => U & WithEquality): LinkedList<T>;
    /**
     * Call a function for element in the collection.
     */
    forEach(fn: (v: T) => void): LinkedList<T>;
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
     * also see [[ConsLinkedList.minOn]]
     */
    minBy(compare: (v1: T, v2: T) => Ordering): Option<T>;
    /**
     * Call the function you give for each value in the collection
     * and return the element for which the result was the smallest.
     * Returns Option.none if the collection is empty.
     *
     *     LinkedList.of({name:"Joe", age:12}, {name:"Paula", age:6}).minOn(x=>x.age)
     *     => Option.of({name:"Paula", age:6})
     *
     * also see [[ConsLinkedList.minBy]]
     */
    minOn(getOrderable: ToOrderable<T>): Option<T>;
    /**
     * Compare values in the collection and return the largest element.
     * Returns Option.none if the collection is empty.
     *
     * also see [[ConsLinkedList.maxOn]]
     */
    maxBy(compare: (v1: T, v2: T) => Ordering): Option<T>;
    /**
     * Call the function you give for each value in the collection
     * and return the element for which the result was the largest.
     * Returns Option.none if the collection is empty.
     *
     *     LinkedList.of({name:"Joe", age:12}, {name:"Paula", age:6}).maxOn(x=>x.age)
     *     => Option.of({name:"Joe", age:12})
     *
     * also see [[ConsLinkedList.maxBy]]
     */
    maxOn(getOrderable: ToOrderable<T>): Option<T>;
    /**
     * Call the function you give for each element in the collection
     * and sum all the numbers, return that sum.
     * Will return 0 if the collection is empty.
     *
     *     LinkedList.of(1,2,3).sumOn(x=>x)
     *     => 6
     */
    sumOn(getNumber: (v: T) => number): number;
    /**
     * Slides a window of a specific size over the sequence.
     * Returns a lazy stream so memory use is not prohibitive.
     *
     *     LinkedList.of(1,2,3,4,5,6,7,8).sliding(3)
     *     => Stream.of(LinkedList.of(1,2,3), LinkedList.of(4,5,6), LinkedList.of(7,8))
     */
    sliding(count: number): Stream<ConsLinkedList<T>>;
    /**
     * Apply the function you give to all elements of the sequence
     * in turn, keeping the intermediate results and returning them
     * along with the final result in a list.
     * The last element of the result is the final cumulative result.
     *
     *     LinkedList.of(1,2,3).scanLeft(0, (soFar,cur)=>soFar+cur)
     *     => LinkedList.of(0,1,3,6)
     */
    scanLeft<U>(init: U, fn: (soFar: U, cur: T) => U): LinkedList<U>;
    /**
     * Apply the function you give to all elements of the sequence
     * in turn, keeping the intermediate results and returning them
     * along with the final result in a list.
     * The first element of the result is the final cumulative result.
     *
     *     LinkedList.of(1,2,3).scanRight(0, (cur,soFar)=>soFar+cur)
     *     => LinkedList.of(6,5,3,0)
     */
    scanRight<U>(init: U, fn: (cur: T, soFar: U) => U): LinkedList<U>;
    /**
     * Joins elements of the collection by a separator.
     * Example:
     *
     *     LinkedList.of(1,2,3).mkString(", ")
     *     => "1, 2, 3"
     */
    mkString(separator: string): string;
    /**
     * Convert to array.
     * Don't do it on an infinite stream!
     */
    toArray(): T[];
    /**
     * Convert to vector.
     * Don't do it on an infinite stream!
     */
    toVector(): Vector<T>;
    /**
     * Convert this collection to a map. You give a function which
     * for each element in the collection returns a pair. The
     * key of the pair will be used as a key in the map, the value,
     * as a value in the map. If several values get the same key,
     * entries will be lost.
     *
     *     LinkedList.of(1,2,3).toMap(x=>[x.toString(), x])
     *     => HashMap.of(["1",1], ["2",2], ["3",3])
     */
    toMap<K, V>(converter: (x: T) => [K & WithEquality, V]): HashMap<K, V>;
    /**
     * Convert this collection to a set. Since the elements of the
     * Seq may not support equality, you must pass a function returning
     * a value supporting equality.
     *
     *     LinkedList.of(1,2,3,3,4).toSet(x=>x)
     *     => HashSet.of(1,2,3,4)
     */
    toSet<K>(converter: (x: T) => K & WithEquality): HashSet<K>;
    /**
     * Transform this value to another value type.
     * Enables fluent-style programming by chaining calls.
     */
    transform<U>(converter: (x: LinkedList<T>) => U): U;
    /**
     * Two objects are equal if they represent the same value,
     * regardless of whether they are the same object physically
     * in memory.
     */
    equals(other: LinkedList<T & WithEquality>): boolean;
    /**
     * Get a number for that object. Two different values
     * may get the same number, but one value must always get
     * the same number. The formula can impact performance.
     */
    hashCode(): number;
    [inspect](): string;
    /**
     * Get a human-friendly string representation of that value.
     *
     * Also see [[ConsLinkedList.mkString]]
     */
    toString(): string;
}
/**
 * ConsLinkedList holds a value and a pointer to a next element,
 * which could be [[ConsLinkedList]] or [[EmptyLinkedList]].
 * A ConsLinkedList is basically a non-empty linked list. It will
 * contain at least one element.
 * "static methods" available through [[LinkedListStatic]]
 * @param T the item type
 */
export declare class ConsLinkedList<T> implements Seq<T> {
    protected value: T;
    protected _tail: LinkedList<T>;
    /**
     * @hidden
     */
    readonly className: "ConsLinkedList";
    /**
     * @hidden
     */
    constructor(value: T, _tail: LinkedList<T>);
    /**
     * @hidden
     */
    hasTrueEquality(): boolean;
    /**
     * View this Some a as LinkedList. Useful to help typescript type
     * inference sometimes.
     */
    asLinkedList(): LinkedList<T>;
    /**
     * Implementation of the Iterator interface.
     */
    [Symbol.iterator](): Iterator<T>;
    /**
     * Get the length of the collection.
     */
    length(): number;
    /**
     * If the collection contains a single element,
     * return Some of its value, otherwise return None.
     */
    single(): Option<T>;
    /**
     * true if the collection is empty, false otherwise.
     */
    isEmpty(): this is EmptyLinkedList<T>;
    /**
     * Get the first value of the collection, if any.
     * In this case the list is not empty, so returns Option.some
     */
    head(): Some<T>;
    /**
     * Get all the elements in the collection but the first one.
     * If the collection is empty, return None.
     */
    tail(): Some<LinkedList<T>>;
    /**
     * Get the last value of the collection, if any.
     * returns Option.Some if the collection is not empty,
     * Option.None if it's empty.
     */
    last(): Some<T>;
    /**
     * Retrieve the element at index idx.
     * Returns an option because the collection may
     * contain less elements than the index.
     *
     * Careful this is going to have poor performance
     * on LinkedList, which is not a good data structure
     * for random access!
     */
    get(idx: number): Option<T>;
    /**
     * Search for an item matching the predicate you pass,
     * return Option.Some of that element if found,
     * Option.None otherwise.
     */
    find(predicate: (v: T) => boolean): Option<T>;
    /**
     * Returns true if the item is in the collection,
     * false otherwise.
     */
    contains(v: T & WithEquality): boolean;
    /**
     * Return a new stream keeping only the first n elements
     * from this stream.
     */
    take(n: number): LinkedList<T>;
    /**
     * Returns a new collection, discarding the elements
     * after the first element which fails the predicate.
     */
    takeWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection, discarding the elements
     * after the first element which fails the predicate,
     * but starting from the end of the collection.
     *
     *     LinkedList.of(1,2,3,4).takeRightWhile(x => x > 2)
     *     => LinkedList.of(3,4)
     */
    takeRightWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection with the first
     * n elements discarded.
     * If the collection has less than n elements,
     * returns the empty collection.
     */
    drop(n: number): LinkedList<T>;
    /**
     * Returns a new collection, discarding the first elements
     * until one element fails the predicate. All elements
     * after that point are retained.
     */
    dropWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection with the last
     * n elements discarded.
     * If the collection has less than n elements,
     * returns the empty collection.
     */
    dropRight(n: number): LinkedList<T>;
    /**
     * Returns a new collection, discarding the last elements
     * until one element fails the predicate. All elements
     * before that point are retained.
     */
    dropRightWhile(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Reduces the collection to a single value using the
     * associative binary function you give. Since the function
     * is associative, order of application doesn't matter.
     *
     * Example:
     *
     *     LinkedList.of(1,2,3).fold(0, (a,b) => a + b);
     *     => 6
     */
    fold(zero: T, fn: (v1: T, v2: T) => T): T;
    /**
     * Reduces the collection to a single value.
     * Left-associative.
     *
     * Example:
     *
     *     Vector.of("a", "b", "c").foldLeft("!", (xs,x) => x+xs);
     *     => "cba!"
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
     *
     * Example:
     *
     *     Vector.of("a", "b", "c").foldRight("!", (x,xs) => xs+x);
     *     => "!cba"
     *
     * @param zero The initial value
     * @param fn A function taking the current collection item and
     *           the previous value , and returning
     *           an updated value.
     */
    foldRight<U>(zero: U, fn: (cur: T, soFar: U) => U): U;
    /**
     * Combine this collection with the collection you give in
     * parameter to produce a new collection which combines both,
     * in pairs. For instance:
     *
     *     Vector.of(1,2,3).zip(["a","b","c"])
     *     => Vector.of([1,"a"], [2,"b"], [3,"c"])
     *
     * The result collection will have the length of the shorter
     * of both collections. Extra elements will be discarded.
     *
     * Also see [[LinkedListStatic.zip]] (static version which can more than two
     * iterables)
     */
    zip<U>(other: Iterable<U>): LinkedList<[T, U]>;
    /**
     * Combine this collection with the index of the elements
     * in it. Handy if you need the index when you map on
     * the collection for instance:
     *
     *     LinkedList.of("a","b").zipWithIndex().map(([v,idx]) => v+idx);
     *     => LinkedList.of("a0", "b1")
     */
    zipWithIndex(): LinkedList<[T, number]>;
    /**
     * Reverse the collection. For instance:
     *
     *     LinkedList.of(1,2,3).reverse();
     *     => LinkedList.of(3,2,1)
     */
    reverse(): LinkedList<T>;
    /**
     * Takes a predicate; returns a pair of collections.
     * The first one is the longest prefix of this collection
     * which satisfies the predicate, and the second collection
     * is the remainder of the collection.
     *
     *    LinkedList.of(1,2,3,4,5,6).span(x => x <3)
     *    => [LinkedList.of(1,2), LinkedList.of(3,4,5,6)]
     */
    span(predicate: (x: T) => boolean): [LinkedList<T>, LinkedList<T>];
    /**
     * Split the collection at a specific index.
     *
     *     LinkedList.of(1,2,3,4,5).splitAt(3)
     *     => [LinkedList.of(1,2,3), LinkedList.of(4,5)]
     */
    splitAt(index: number): [LinkedList<T>, LinkedList<T>];
    /**
     * Returns a pair of two collections; the first one
     * will only contain the items from this collection for
     * which the predicate you give returns true, the second
     * will only contain the items from this collection where
     * the predicate returns false.
     *
     *     LinkedList.of(1,2,3,4).partition(x => x%2===0)
     *     => [LinkedList.of(2,4),LinkedList.of(1,3)]
     */
    partition<U extends T>(predicate: (v: T) => v is U): [LinkedList<U>, LinkedList<Exclude<T, U>>];
    partition(predicate: (x: T) => boolean): [LinkedList<T>, LinkedList<T>];
    /**
     * Group elements in the collection using a classifier function.
     * Elements are then organized in a map. The key is the value of
     * the classifier, and in value we get the list of elements
     * matching that value.
     *
     * also see [[ConsLinkedList.arrangeBy]]
     */
    groupBy<C>(classifier: (v: T) => C & WithEquality): HashMap<C, LinkedList<T>>;
    /**
     * Matches each element with a unique key that you extract from it.
     * If the same key is present twice, the function will return None.
     *
     * also see [[ConsLinkedList.groupBy]]
     */
    arrangeBy<K>(getKey: (v: T) => K & WithEquality): Option<HashMap<K, T>>;
    /**
     * Randomly reorder the elements of the collection.
     */
    shuffle(): LinkedList<T>;
    /**
     * Append an element at the end of this LinkedList.
     * Warning: appending in a loop on a linked list is going
     * to be very slow!
     */
    append(v: T): LinkedList<T>;
    appendAll(elts: Iterable<T>): LinkedList<T>;
    /**
     * Remove multiple elements from a LinkedList
     *
     *     LinkedList.of(1,2,3,4,3,2,1).removeAll([2,4])
     *     => LinkedList.of(1,3,3,1)
     */
    removeAll(elts: Iterable<T & WithEquality>): LinkedList<T>;
    /**
     * Removes the first element matching the predicate
     * (use [[Seq.filter]] to remove all elements matching a predicate)
     */
    removeFirst(predicate: (x: T) => boolean): LinkedList<T>;
    /**
     * Prepend an element at the beginning of the collection.
     */
    prepend(elt: T): LinkedList<T>;
    /**
     * Prepend multiple elements at the beginning of the collection.
     */
    prependAll(elts: Iterable<T>): LinkedList<T>;
    /**
     * Return a new collection where each element was transformed
     * by the mapper function you give.
     */
    map<U>(mapper: (v: T) => U): LinkedList<U>;
    /**
     * Apply the mapper function on every element of this collection.
     * The mapper function returns an Option; if the Option is a Some,
     * the value it contains is added to the result Collection, if it's
     * a None, the value is discarded.
     *
     *     LinkedList.of(1,2,6).mapOption(x => x%2===0 ?
     *         Option.of(x+1) : Option.none<number>())
     *     => LinkedList.of(3, 7)
     */
    mapOption<U>(mapper: (v: T) => Option<U>): LinkedList<U>;
    /**
     * Calls the function you give for each item in the collection,
     * your function returns a collection, all the collections are
     * concatenated.
     * This is the monadic bind.
     */
    flatMap<U>(mapper: (v: T) => LinkedList<U>): LinkedList<U>;
    /**
     * Returns true if the predicate returns true for all the
     * elements in the collection.
     */
    allMatch<U extends T>(predicate: (v: T) => v is U): this is LinkedList<U>;
    allMatch(predicate: (v: T) => boolean): boolean;
    /**
     * Returns true if there the predicate returns true for any
     * element in the collection.
     */
    anyMatch(predicate: (v: T) => boolean): boolean;
    /**
     * Call a predicate for each element in the collection,
     * build a new collection holding only the elements
     * for which the predicate returned true.
     */
    filter<U extends T>(predicate: (v: T) => v is U): LinkedList<U>;
    filter(predicate: (v: T) => boolean): LinkedList<T>;
    /**
     * Returns a new collection with elements
     * sorted according to the comparator you give.
     *
     *     const activityOrder = ["Writer", "Actor", "Director"];
     *     LinkedList.of({name:"George", activity: "Director"}, {name:"Robert", activity: "Actor"})
     *         .sortBy((p1,p2) => activityOrder.indexOf(p1.activity) - activityOrder.indexOf(p2.activity));
     *     => LinkedList.of({"name":"Robert","activity":"Actor"}, {"name":"George","activity":"Director"})
     *
     * also see [[ConsLinkedList.sortOn]]
     */
    sortBy(compare: (v1: T, v2: T) => Ordering): LinkedList<T>;
    /**
     * Give a function associating a number or a string with
     * elements from the collection, and the elements
     * are sorted according to that value.
     *
     *     LinkedList.of({a:3,b:"b"},{a:1,b:"test"},{a:2,b:"a"}).sortOn(elt=>elt.a)
     *     => LinkedList.of({a:1,b:"test"},{a:2,b:"a"},{a:3,b:"b"})
     *
     * You can also sort by multiple criteria, and request 'descending'
     * sorting:
     *
     *     LinkedList.of({a:1,b:"b"},{a:1,b:"test"},{a:2,b:"a"}).sortOn(elt=>elt.a,{desc:elt=>elt.b})
     *     => LinkedList.of({a:1,b:"test"},{a:1,b:"b"},{a:2,b:"a"})
     *
     * also see [[ConsLinkedList.sortBy]]
     */
    sortOn(...getKeys: Array<ToOrderable<T> | {
        desc: ToOrderable<T>;
    }>): LinkedList<T>;
    /**
     * Remove duplicate items; elements are mapped to keys, those
     * get compared.
     *
     *     LinkedList.of(1,1,2,3,2,3,1).distinctBy(x => x)
     *     => LinkedList.of(1,2,3)
     */
    distinctBy<U>(keyExtractor: (x: T) => U & WithEquality): LinkedList<T>;
    /**
     * Call a function for element in the collection.
     */
    forEach(fn: (v: T) => void): LinkedList<T>;
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
     * also see [[ConsLinkedList.minOn]]
     */
    minBy(compare: (v1: T, v2: T) => Ordering): Option<T>;
    /**
     * Call the function you give for each value in the collection
     * and return the element for which the result was the smallest.
     * Returns Option.none if the collection is empty.
     *
     *     LinkedList.of({name:"Joe", age:12}, {name:"Paula", age:6}).minOn(x=>x.age)
     *     => Option.of({name:"Paula", age:6})
     *
     * also see [[ConsLinkedList.minBy]]
     */
    minOn(getOrderable: ToOrderable<T>): Option<T>;
    /**
     * Compare values in the collection and return the largest element.
     * Returns Option.none if the collection is empty.
     *
     *     LinkedList.of({name:"Joe", age:12}, {name:"Paula", age:6}).maxOn(x=>x.age)
     *     => Option.of({name:"Joe", age:12})
     *
     * also see [[ConsLinkedList.maxOn]]
     */
    maxBy(compare: (v1: T, v2: T) => Ordering): Option<T>;
    /**
     * Call the function you give for each value in the collection
     * and return the element for which the result was the largest.
     * Returns Option.none if the collection is empty.
     *
     * also see [[ConsLinkedList.maxBy]]
     */
    maxOn(getOrderable: ToOrderable<T>): Option<T>;
    /**
     * Call the function you give for each element in the collection
     * and sum all the numbers, return that sum.
     * Will return 0 if the collection is empty.
     *
     *     LinkedList.of(1,2,3).sumOn(x=>x)
     *     => 6
     */
    sumOn(getNumber: (v: T) => number): number;
    /**
     * Slides a window of a specific size over the sequence.
     * Returns a lazy stream so memory use is not prohibitive.
     *
     *     LinkedList.of(1,2,3,4,5,6,7,8).sliding(3)
     *     => Stream.of(LinkedList.of(1,2,3), LinkedList.of(4,5,6), LinkedList.of(7,8))
     */
    sliding(count: number): Stream<ConsLinkedList<T>>;
    /**
     * Apply the function you give to all elements of the sequence
     * in turn, keeping the intermediate results and returning them
     * along with the final result in a list.
     *
     *     LinkedList.of(1,2,3).scanLeft(0, (soFar,cur)=>soFar+cur)
     *     => LinkedList.of(0,1,3,6)
     */
    scanLeft<U>(init: U, fn: (soFar: U, cur: T) => U): LinkedList<U>;
    /**
     * Apply the function you give to all elements of the sequence
     * in turn, keeping the intermediate results and returning them
     * along with the final result in a list.
     * The first element of the result is the final cumulative result.
     *
     *     LinkedList.of(1,2,3).scanRight(0, (cur,soFar)=>soFar+cur)
     *     => LinkedList.of(6,5,3,0)
     */
    scanRight<U>(init: U, fn: (cur: T, soFar: U) => U): LinkedList<U>;
    /**
     * Joins elements of the collection by a separator.
     * Example:
     *
     *     LinkedList.of(1,2,3).mkString(", ")
     *     => "1, 2, 3"
     */
    mkString(separator: string): string;
    /**
     * Convert to array.
     * Don't do it on an infinite stream!
     */
    toArray(): T[];
    /**
     * Convert to vector.
     * Don't do it on an infinite stream!
     */
    toVector(): Vector<T>;
    /**
     * Convert this collection to a map. You give a function which
     * for each element in the collection returns a pair. The
     * key of the pair will be used as a key in the map, the value,
     * as a value in the map. If several values get the same key,
     * entries will be lost.
     *
     *     LinkedList.of(1,2,3).toMap(x=>[x.toString(), x])
     *     => HashMap.of(["1",1], ["2",2], ["3",3])
     */
    toMap<K, V>(converter: (x: T) => [K & WithEquality, V]): HashMap<K, V>;
    /**
     * Convert this collection to a set. Since the elements of the
     * Seq may not support equality, you must pass a function returning
     * a value supporting equality.
     *
     *     LinkedList.of(1,2,3,3,4).toSet(x=>x)
     *     => HashSet.of(1,2,3,4)
     */
    toSet<K>(converter: (x: T) => K & WithEquality): HashSet<K>;
    /**
     * Transform this value to another value type.
     * Enables fluent-style programming by chaining calls.
     */
    transform<U>(converter: (x: LinkedList<T>) => U): U;
    /**
     * Two objects are equal if they represent the same value,
     * regardless of whether they are the same object physically
     * in memory.
     */
    equals(other: LinkedList<T & WithEquality>): boolean;
    /**
     * Get a number for that object. Two different values
     * may get the same number, but one value must always get
     * the same number. The formula can impact performance.
     */
    hashCode(): number;
    [inspect](): string;
    /**
     * Get a human-friendly string representation of that value.
     *
     * Also see [[ConsLinkedList.mkString]]
     */
    toString(): string;
}
