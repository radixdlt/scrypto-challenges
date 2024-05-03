import { inspect } from "./Value";
/**
 * Represent a lazily evaluated value. You give a function which
 * will return a value; that function is only called when the value
 * is requested from Lazy, but it will be computed at most once.
 * If the value is requested again, the previously computed result
 * will be returned: Lazy is memoizing.
 */
export declare class Lazy<T> {
    private thunk;
    private value;
    private constructor();
    /**
     * Build a Lazy from a computation returning a value.
     * The computation will be called at most once.
     */
    static of<T>(thunk: () => T): Lazy<T>;
    /**
     * Evaluate the value, cache its value, and return it, or return the
     * previously computed value.
     */
    get(): T;
    /**
     * Returns true if the computation underlying this Lazy was already
     * performed, false otherwise.
     */
    isEvaluated(): boolean;
    /**
     * Return a new lazy where the element was transformed
     * by the mapper function you give.
     */
    map<U>(mapper: (v: T) => U): Lazy<U>;
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
