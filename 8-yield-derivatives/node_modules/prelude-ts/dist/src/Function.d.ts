/**
 * Rich functions with helpers such as [[Function1.andThen]],
 * [[Function2.apply1]] and so on.
 *
 * We support functions of arities up to 5. For each arity, we have
 * the interface ([[Function1]], [[Function2]], ...), builders are on functions
 * on [[Function1Static]], [[Function2Static]]... accessible on constants
 * named Function1, Function2,...
 *
 * Examples:
 *
 *     const combined = Function1.of((x:number)=>x+2).andThen(x=>x*3);
 *     combined(6);
 *     => 24
 *
 *     const plus5 = Function2.of((x:number,y:number)=>x+y).apply1(5);
 *     plus5(1);
 *     => 6
 */
/**
 * Function0 encapsulates a parameterless function
 * which returns a value. It adds some useful functions
 * to combine or transform functions.
 *
 * @param T the parameter type
 * @param U the result type
 */
export interface Function0<R> {
    /**
     * Invoke the function
     */
    (): R;
    /**
     * Returns a new composed function which first calls the current
     * function and then the one you pass as parameter.
     */
    andThen<V>(fn: (x: R) => V): Function0<V>;
}
/**
 * Function1 encapsulates a function taking a single parameter
 * and returning a value. It adds some useful functions
 * to combine or transform functions.
 *
 * @param T the parameter type
 * @param U the result type
 */
export interface Function1<T, U> {
    /**
     * Invoke the function
     */
    (x: T): U;
    /**
     * Returns a new composed function which first applies the current
     * function and then the one you pass as parameter.
     */
    andThen<V>(fn: (x: U) => V): Function1<T, V>;
    /**
     *
     */
    compose<S>(fn: (x: S) => T): Function1<S, U>;
}
/**
 * Function2 encapsulates a function taking two parameters
 * and returning a value. It adds some useful functions
 * to combine or transform functions.
 *
 * @param T1 the first parameter type
 * @param T2 the second parameter type
 * @param R the result type
 */
export interface Function2<T1, T2, R> {
    /**
     * Invoke the function
     */
    (x: T1, y: T2): R;
    /**
     * Returns a new composed function which first applies the current
     * function and then the one you pass as parameter.
     */
    andThen<V>(fn: (x: R) => V): Function2<T1, T2, V>;
    /**
     * Returns a curried version of this function, for example:
     *
     *     const plus5 = Function2.of(
     *         (x:number,y:number)=>x+y)
     *            .curried()(5);
     *     assert.equal(6, plus5(1));
     */
    curried(): Function1<T1, Function1<T2, R>>;
    /**
     * Returns a version of this function which takes a tuple
     * instead of individual parameters. Useful in combination
     * with [[Vector.zip]] for instance.
     */
    tupled(): Function1<[T1, T2], R>;
    /**
     * Returns a version of this function taking its parameters
     * in the reverse order.
     */
    flipped(): Function2<T2, T1, R>;
    /**
     * Applies this function partially to one argument.
     *
     *     const plus5 = Function2.of(
     *         (x:number,y:number)=>x+y)
     *            .apply1(5);
     *     assert.equal(6, plus5(1));
     */
    apply1(param1: T1): Function1<T2, R>;
}
/**
 * Function3 encapsulates a function taking three parameters
 * and returning a value. It adds some useful functions
 * to combine or transform functions.
 *
 * @param T1 the first parameter type
 * @param T2 the second parameter type
 * @param T3 the third parameter type
 * @param R the result type
 */
export interface Function3<T1, T2, T3, R> {
    /**
     * Invoke the function
     */
    (x: T1, y: T2, z: T3): R;
    /**
     * Returns a new composed function which first applies the current
     * function and then the one you pass as parameter.
     */
    andThen<V>(fn: (x: R) => V): Function3<T1, T2, T3, V>;
    /**
     * Returns a curried version of this function, for example:
     * See [[Function2.curried]]
     */
    curried(): Function1<T1, Function1<T2, Function1<T3, R>>>;
    /**
     * Returns a version of this function which takes a tuple
     * instead of individual parameters.
     */
    tupled(): Function1<[T1, T2, T3], R>;
    /**
     * Returns a version of this function taking its parameters
     * in the reverse order.
     */
    flipped(): Function3<T3, T2, T1, R>;
    /**
     * Applies this function partially to one argument.
     *
     *     const plus5 = Function3.of(
     *         (x:number,y:number,z:number)=>x+y+z)
     *            .apply1(5);
     *     assert.equal(8, plus5(1,2));
     */
    apply1(param1: T1): Function2<T2, T3, R>;
    /**
     * Applies this function partially to two arguments.
     *
     *     const plus54 = Function3.of(
     *         (x:number,y:number,z:number)=>x+y+z)
     *            .apply2(5,4);
     *     assert.equal(12, plus54(3));
     */
    apply2(param1: T1, param2: T2): Function1<T3, R>;
}
/**
 * Function4 encapsulates a function taking four parameters
 * and returning a value. It adds some useful functions
 * to combine or transform functions.
 *
 * @param T1 the first parameter type
 * @param T2 the second parameter type
 * @param T3 the third parameter type
 * @param T4 the fourth parameter type
 * @param R the result type
 */
export interface Function4<T1, T2, T3, T4, R> {
    /**
     * Invoke the function
     */
    (x: T1, y: T2, z: T3, a: T4): R;
    /**
     * Returns a new composed function which first applies the current
     * function and then the one you pass as parameter.
     */
    andThen<V>(fn: (x: R) => V): Function4<T1, T2, T3, T4, V>;
    /**
     * Returns a curried version of this function, for example:
     * See [[Function2.curried]]
     */
    curried(): Function1<T1, Function1<T2, Function1<T3, Function1<T4, R>>>>;
    /**
     * Returns a version of this function which takes a tuple
     * instead of individual parameters.
     */
    tupled(): Function1<[T1, T2, T3, T4], R>;
    /**
     * Returns a version of this function taking its parameters
     * in the reverse order.
     */
    flipped(): Function4<T4, T3, T2, T1, R>;
    /**
     * Applies this function partially to one argument.
     *
     *     const plus5 = Function4.of(
     *         (x:number,y:number,z:number,a:number)=>x+y+z+a)
     *            .apply1(5);
     *     assert.equal(11, plus5(1,2,3));
     */
    apply1(param1: T1): Function3<T2, T3, T4, R>;
    /**
     * Applies this function partially to two arguments.
     *
     *     const plus51 = Function4.of(
     *         (x:number,y:number,z:number,a:number)=>x+y+z+a)
     *            .apply2(5,1);
     *     assert.equal(11, plus51(2,3));
     */
    apply2(param1: T1, param2: T2): Function2<T3, T4, R>;
    /**
     * Applies this function partially to three arguments.
     *
     *     const plus512 = Function4.of(
     *         (x:number,y:number,z:number,a:number)=>x+y+z+a)
     *            .apply3(5,1,2);
     *     assert.equal(11, plus512(3));
     */
    apply3(param1: T1, param2: T2, param3: T3): Function1<T4, R>;
}
/**
 * Function5 encapsulates a function taking give parameters
 * and returning a value. It adds some useful functions
 * to combine or transform functions.
 *
 * @param T1 the first parameter type
 * @param T2 the second parameter type
 * @param T3 the third parameter type
 * @param T4 the fourth parameter type
 * @param T5 the fifth parameter type
 * @param R the result type
 */
export interface Function5<T1, T2, T3, T4, T5, R> {
    /**
     * Invoke the function
     */
    (x: T1, y: T2, z: T3, a: T4, b: T5): R;
    /**
     * Returns a new composed function which first applies the current
     * function and then the one you pass as parameter.
     */
    andThen<V>(fn: (x: R) => V): Function5<T1, T2, T3, T4, T5, V>;
    /**
     * Returns a curried version of this function, for example:
     * See [[Function2.curried]]
     */
    curried(): Function1<T1, Function1<T2, Function1<T3, Function1<T4, Function1<T5, R>>>>>;
    /**
     * Returns a version of this function which takes a tuple
     * instead of individual parameters.
     */
    tupled(): Function1<[T1, T2, T3, T4, T5], R>;
    /**
     * Returns a version of this function taking its parameters
     * in the reverse order.
     */
    flipped(): Function5<T5, T4, T3, T2, T1, R>;
    /**
     * Applies this function partially to one argument.
     *
     *     const plus5 = Function5.of(
     *         (x:number,y:number,z:number,a:number,b:number)=>x+y+z+a+b)
     *            .apply1(5);
     *     assert.equal(15, plus5(1,2,3,4));
     */
    apply1(param1: T1): Function4<T2, T3, T4, T5, R>;
    /**
     * Applies this function partially to two arguments.
     *
     *     const plus51 = Function5.of(
     *         (x:number,y:number,z:number,a:number,b:number)=>x+y+z+a+b)
     *            .apply2(5,1);
     *     assert.equal(15, plus51(2,3,4));
     */
    apply2(param1: T1, param2: T2): Function3<T3, T4, T5, R>;
    /**
     * Applies this function partially to three arguments.
     *
     *     const plus512 = Function5.of(
     *         (x:number,y:number,z:number,a:number,b:number)=>x+y+z+a+b)
     *            .apply3(5,1,2);
     *     assert.equal(15, plus512(3,4));
     */
    apply3(param1: T1, param2: T2, param3: T3): Function2<T4, T5, R>;
    /**
     * Applies this function partially to four arguments.
     *
     *     const plus5123 = Function5.of(
     *         (x:number,y:number,z:number,a:number,b:number)=>x+y+z+a+b)
     *            .apply4(5,1,2,3);
     *     assert.equal(15, plus5123(4));
     */
    apply4(param1: T1, param2: T2, param3: T3, param4: T4): Function1<T5, R>;
}
/**
 * This is the type of the Function0 constant, which
 * offers some helper functions to deal
 * with [[Function0]] including
 * the ability to build [[Function0]]
 * from functions using [[Function0Static.of]].
 * It also offers some builtin functions like [[Function0Static.constant]].
 */
export declare class Function0Static {
    /**
     * The constant function of one parameter:
     * will always return the value you give, no
     * matter the parameter it's given.
     */
    constant<R>(val: R): Function0<R>;
    /**
     * Take a one-parameter function and lift it to become a [[Function1Static]],
     * enabling you to call [[Function1.andThen]] and other such methods on it.
     */
    of<R>(fn: () => R): Function0<R>;
}
/**
 * The Function1 constant allows to call the [[Function0]] "static" methods.
 */
export declare const Function0: Function0Static;
/**
 * This is the type of the Function1 constant, which
 * offers some helper functions to deal
 * with [[Function1]] including
 * the ability to build [[Function1]]
 * from functions using [[Function1Static.of]].
 * It also offers some builtin functions like [[Function1Static.constant]].
 */
export declare class Function1Static {
    /**
     * The identity function.
     */
    id<T>(): Function1<T, T>;
    /**
     * The constant function of one parameter:
     * will always return the value you give, no
     * matter the parameter it's given.
     */
    constant<U, T>(val: T): Function1<U, T>;
    /**
     * Take a one-parameter function and lift it to become a [[Function1Static]],
     * enabling you to call [[Function1.andThen]] and other such methods on it.
     */
    of<T, U>(fn: (x: T) => U): Function1<T, U>;
}
/**
 * The Function1 constant allows to call the [[Function1]] "static" methods.
 */
export declare const Function1: Function1Static;
/**
 * This is the type of the Function2 constant, which
 * offers some helper functions to deal
 * with [[Function2]] including
 * the ability to build [[Function2]]
 * from functions using [[Function2Static.of]].
 * It also offers some builtin functions like [[Function2Static.constant]].
 */
export declare class Function2Static {
    /**
     * The constant function of two parameters:
     * will always return the value you give, no
     * matter the parameters it's given.
     */
    constant<T1, T2, R>(val: R): Function2<T1, T2, R>;
    /**
     * Take a two-parameter function and lift it to become a [[Function2]],
     * enabling you to call [[Function2.andThen]] and other such methods on it.
     */
    of<T1, T2, R>(fn: (x: T1, y: T2) => R): Function2<T1, T2, R>;
}
/**
 * The Function2 constant allows to call the [[Function2]] "static" methods.
 */
export declare const Function2: Function2Static;
/**
 * This is the type of the Function3 constant, which
 * offers some helper functions to deal
 * with [[Function3]] including
 * the ability to build [[Function3]]
 * from functions using [[Function3Static.of]].
 * It also offers some builtin functions like [[Function3Static.constant]].
 */
export declare class Function3Static {
    /**
     * The constant function of three parameters:
     * will always return the value you give, no
     * matter the parameters it's given.
     */
    constant<T1, T2, T3, R>(val: R): Function3<T1, T2, T3, R>;
    /**
     * Take a three-parameter function and lift it to become a [[Function3]],
     * enabling you to call [[Function3.andThen]] and other such methods on it.
     */
    of<T1, T2, T3, R>(fn: (x: T1, y: T2, z: T3) => R): Function3<T1, T2, T3, R>;
}
/**
 * The Function3 constant allows to call the [[Function3]] "static" methods.
 */
export declare const Function3: Function3Static;
/**
 * This is the type of the Function4 constant, which
 * offers some helper functions to deal
 * with [[Function4]] including
 * the ability to build [[Function4]]
 * from functions using [[Function4Static.of]].
 * It also offers some builtin functions like [[Function4Static.constant]].
 */
export declare class Function4Static {
    /**
     * The constant function of four parameters:
     * will always return the value you give, no
     * matter the parameters it's given.
     */
    constant<T1, T2, T3, T4, R>(val: R): Function4<T1, T2, T3, T4, R>;
    /**
     * Take a four-parameter function and lift it to become a [[Function4]],
     * enabling you to call [[Function4.andThen]] and other such methods on it.
     */
    of<T1, T2, T3, T4, R>(fn: (x: T1, y: T2, z: T3, a: T4) => R): Function4<T1, T2, T3, T4, R>;
}
/**
 * The Function4 constant allows to call the [[Function4]] "static" methods.
 */
export declare const Function4: Function4Static;
/**
 * This is the type of the Function5 constant, which
 * offers some helper functions to deal
 * with [[Function5]] including
 * the ability to build [[Function5]]
 * from functions using [[Function5Static.of]].
 * It also offers some builtin functions like [[Function5Static.constant]].
 */
export declare class Function5Static {
    /**
     * The constant function of five parameters:
     * will always return the value you give, no
     * matter the parameters it's given.
     */
    constant<T1, T2, T3, T4, T5, R>(val: R): Function5<T1, T2, T3, T4, T5, R>;
    /**
     * Take a five-parameter function and lift it to become a [[Function5]],
     * enabling you to call [[Function5.andThen]] and other such methods on it.
     */
    of<T1, T2, T3, T4, T5, R>(fn: (x: T1, y: T2, z: T3, a: T4, b: T5) => R): Function5<T1, T2, T3, T4, T5, R>;
}
/**
 * The Function5 constant allows to call the [[Function5]] "static" methods.
 */
export declare const Function5: Function5Static;
