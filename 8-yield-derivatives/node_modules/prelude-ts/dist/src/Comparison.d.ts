import { Option } from "./Option";
/**
 * Sorting function for type T: function
 * to convert this type to a type which is natively
 * sortable in javascript, that is string, number or boolean.
 * `((v:T)=>number) | ((v:T)=>string) | ((v:T)=>boolean`
 */
export declare type ToOrderable<T> = ((v: T) => number) | ((v: T) => string) | ((v: T) => boolean);
/**
 * List of types which provide equality semantics:
 * some builtin JS types, for which === provides
 * proper semantics, and then types providing HasEquals.
 * The reason I use all over the place T&WithEquality
 * instead of saying <T extends WithEquality> earlier
 * in the declaration is: https://stackoverflow.com/a/45903143/516188
 */
export declare type WithEquality = string | number | boolean | null | HasEquals;
/**
 * A type with semantic equality relationships
 */
export declare type HasEquals = {
    equals(other: any): boolean;
    hashCode(): number;
};
/**
 * Type guard for HasEquals: find out for a type with
 * semantic equality, whether you should call .equals
 * or ===
 */
export declare function hasEquals(v: WithEquality): v is HasEquals;
/**
 * Helper function for your objects so you can compute
 * a hashcode. You can pass to this function all the fields
 * of your object that should be taken into account for the
 * hash, and the function will return a reasonable hash code.
 *
 * @param fields the fields of your object to take
 *        into account for the hashcode
 */
export declare function fieldsHashCode(...fields: any[]): number;
/**
 * Helper function to compute a reasonable hashcode for strings.
 */
export declare function stringHashCode(str: string): number;
/**
 * Equality function which tries semantic equality (using .equals())
 * if possible, degrades to === if not available, and is also null-safe.
 */
export declare function areEqual(obj: any | null, obj2: any | null): boolean;
/**
 * Hashing function which tries to call hashCode()
 * and uses the object itself for numbers, then degrades
 * for stringHashCode of the string representation if
 * not available.
 */
export declare function getHashCode(obj: any | null): number;
/**
 * @hidden
 */
export declare function hasTrueEquality(val: any): Option<boolean>;
/**
 * Enumeration used to express ordering relationships.
 * it's a const enum, is replaced by integers in the source.
 */
export declare const enum Ordering {
    /**
     * Lower Than
     */
    LT = -1,
    /**
     * EQuals
     */
    EQ = 0,
    /**
     * Greater Than
     */
    GT = 1
}
/**
 * Typescript doesn't infer typeguards for lambdas; it only sees
 * predicates. This type allows you to cast a predicate to a type
 * guard in a handy manner.
 *
 * It comes in handy for discriminated unions with a 'kind' discriminator,
 * for instance:
 *
 * .`filter(<TypeGuard<InBoard|OutBoard,InBoard>>(p => p.kind === "in_board"))`
 *
 * Also see [[typeGuard]], [[instanceOf]] and [[typeOf]].
 */
export declare type TypeGuard<T, U extends T> = (x: T) => x is U;
/**
 * Typescript doesn't infer typeguards for lambdas; it only sees
 * predicates. This type allows you to cast a predicate to a type
 * guard in a handy manner.
 *
 * It comes in handy for discriminated unions with a 'kind' discriminator,
 * for instance:
 *
 * `.filter(typeGuard(p => p.kind === "in_board", {} as InBoard))`
 *
 * Normally you'd have to give both type parameters, but you can use
 * the type witness parameter as shown in that example to skip
 * the first type parameter.
 *
 * Also see [[typeGuard]], [[instanceOf]] and [[typeOf]].
 */
export declare function typeGuard<T, U extends T>(predicate: (x: T) => boolean, typeWitness?: U): TypeGuard<T, U>;
/**
 * Curried function returning a type guard telling us if a value
 * is of a specific instance.
 * Can be used when filtering to filter for the type and at the
 * same time change the type of the generics on the container.
 *
 *     Vector.of<any>("bad", new Date('04 Dec 1995 00:12:00 GMT')).filter(instanceOf(Date))
 *     => Vector.of<Date>(new Date('04 Dec 1995 00:12:00 GMT'))
 *
 *     Option.of<any>("test").filter(instanceOf(Date))
 *     => Option.none<Date>()
 *
 *     Option.of<any>(new Date('04 Dec 1995 00:12:00 GMT')).filter(instanceOf(Date))
 *     => Option.of<Date>(new Date('04 Dec 1995 00:12:00 GMT'))
 *
 * Also see [[typeGuard]] and [[typeOf]].
 */
export declare function instanceOf<T>(ctor: new (...args: any[]) => T): TypeGuard<any, T>;
/**
 * Curried function returning a type guard telling us if a value
 * is of a specific type.
 * Can be used when filtering to filter for the type and at the
 * same time change the type of the generics on the container.
 *
 *     Vector.of<any>(1,"a",2,3,"b").filter(typeOf("number"))
 *     => Vector.of<number>(1,2,3)
 *
 *     Option.of<any>(1).filter(typeOf("string"))
 *     => Option.none<string>()
 *
 *     Option.of<any>("str").filter(typeOf("string"))
 *     => Option.of<string>("str")
 *
 * Also see [[instanceOf]] and [[typeGuard]].
 */
export declare function typeOf(typ: "string"): TypeGuard<any, string>;
export declare function typeOf(typ: "number"): TypeGuard<any, number>;
export declare function typeOf(typ: "boolean"): TypeGuard<any, boolean>;
export declare function typeOf(typ: "symbol"): TypeGuard<any, symbol>;
