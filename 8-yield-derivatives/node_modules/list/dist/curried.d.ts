import * as L from "./index";
import { List } from "./index";
export { Node, List, list, isList, length, of, empty, first, head, last, flatten, pop, init, tail, from, toArray, reverse, backwards, sort, group, dropRepeats, isEmpty } from "./index";
export interface Curried2<A, B, R> {
    (a: A): (b: B) => R;
    (a: A, b: B): R;
}
export interface Curried3<A, B, C, R> {
    (a: A, b: B, c: C): R;
    (a: A, b: B): (c: C) => R;
    (a: A): Curried2<B, C, R>;
}
export declare const prepend: typeof L.prepend & (<A>(value: A) => (l: List<A>) => List<A>);
export declare const append: typeof prepend;
export declare const pair: typeof L.pair & (<A>(first: A) => (second: A) => List<A>);
export declare const repeat: typeof L.repeat & (<A>(value: A) => (times: number) => List<A>);
export declare const times: typeof L.times & (<A>(func: (index: number) => A) => (times: number) => List<A>);
export declare const nth: typeof L.nth & ((index: number) => <A>(l: List<A>) => A | undefined);
export declare const map: typeof L.map & (<A, B>(f: (a: A) => B) => (l: List<A>) => List<B>);
export declare const forEach: typeof L.forEach & (<A>(callback: (a: A) => void) => (l: List<A>) => void);
export declare const pluck: typeof L.pluck & (<K extends string>(key: K) => <C, B extends K & (keyof C)>(l: List<C>) => List<C[B]>);
export declare const intersperse: typeof prepend;
export declare const range: typeof L.range & ((start: number) => (end: number) => List<number>);
export declare const filter: typeof L.filter & (<A, B extends A>(predicate: (a: A) => a is B) => (l: List<A>) => List<B>) & (<A>(predicate: (a: A) => boolean) => (l: List<A>) => List<A>);
export declare const reject: typeof filter;
export declare const partition: typeof L.partition & (<A>(predicate: (a: A) => boolean) => (l: List<A>) => [List<A>, List<A>]);
export declare const join: typeof L.join & ((seperator: string) => (l: List<string>) => List<string>);
export declare const ap: typeof L.ap & (<A, B>(listF: List<(a: A) => B>) => (l: List<A>) => List<B>);
export declare const flatMap: typeof L.flatMap & (<A, B>(f: (a: A) => List<B>) => (l: List<A>) => List<B>);
export declare const chain: typeof L.flatMap & (<A, B>(f: (a: A) => L.List<B>) => (l: L.List<A>) => L.List<B>);
export declare const every: typeof L.every & (<A>(predicate: (a: A) => boolean) => (l: List<A>) => boolean);
export declare const all: typeof every;
export declare const some: typeof every;
export declare const any: typeof every;
export declare const none: typeof every;
export declare const find: typeof L.find & (<A>(predicate: (a: A) => boolean) => (l: List<A>) => A | undefined);
export declare const findLast: typeof find;
export declare const indexOf: typeof L.indexOf & (<A>(element: A) => (l: List<A>) => number);
export declare const lastIndexOf: typeof indexOf;
export declare const findIndex: typeof L.findIndex & (<A>(predicate: (a: A) => boolean) => (l: List<A>) => number);
export declare const includes: typeof L.includes & (<A>(element: A) => (l: List<A>) => number);
export declare const contains: typeof L.includes & (<A>(element: A) => (l: L.List<A>) => number);
export declare const equals: typeof L.equals & (<A>(first: List<A>) => (second: List<A>) => boolean);
export declare const concat: typeof L.concat & (<A>(left: List<A>) => (right: List<A>) => List<A>);
export declare const take: typeof L.take & ((n: number) => <A>(l: List<A>) => List<A>);
export declare const takeLast: typeof take;
export declare const drop: typeof take;
export declare const dropRepeatsWith: typeof L.dropRepeatsWith & (<A>(f: (a: A, b: A) => boolean) => (l: List<A>) => List<A>);
export declare const dropLast: typeof take;
export declare const takeWhile: typeof filter;
export declare const takeLastWhile: typeof filter;
export declare const dropWhile: typeof filter;
export declare const splitAt: typeof L.splitAt & ((index: number) => <A>(l: List<A>) => [List<A>, List<A>]);
export declare const splitWhen: typeof L.splitWhen & (<A>(predicate: (a: A) => boolean) => (l: List<A>) => [List<A>, List<A>]);
export declare const splitEvery: typeof L.splitEvery & (<A>(size: number) => (l: List<A>) => List<List<A>>);
export declare const sortBy: typeof L.sortBy & (<A, B extends L.Comparable>(f: (a: A) => B) => (l: List<A>) => List<A>);
export declare const sortWith: typeof L.sortWith & (<A>(comparator: (a: A, b: A) => L.Ordering) => (l: List<A>) => List<A>);
export declare const groupWith: typeof L.groupWith & (<A>(f: (a: A, b: A) => boolean) => (l: List<A>) => List<List<A>>);
export declare const zip: typeof L.zip & (<A>(as: List<A>) => <B>(bs: List<B>) => List<[A, B]>);
export declare const sequence: typeof L.sequence & ((ofObj: L.Of) => <A>(l: List<L.Applicative<A>>) => any);
export declare const foldl: typeof L.foldl & {
    <A, B>(f: (acc: B, value: A) => B): Curried2<B, List<A>, B>;
    <A, B>(f: (acc: B, value: A) => B, initial: B): (l: List<A>) => B;
};
export declare const reduce: typeof foldl;
export declare const scan: typeof L.scan & {
    <A, B>(f: (acc: B, value: A) => B): Curried2<B, List<A>, List<B>>;
    <A, B>(f: (acc: B, value: A) => B, initial: B): (l: List<A>) => List<B>;
};
export declare const foldr: typeof L.foldl & {
    <A, B>(f: (value: A, acc: B) => B): Curried2<B, List<A>, B>;
    <A, B>(f: (value: A, acc: B) => B, initial: B): (l: List<A>) => B;
};
export declare const traverse: typeof L.traverse & {
    (of: L.Of): (<A, B>(f: (a: A) => L.Applicative<B>) => (l: List<B>) => any) & (<A, B>(f: (a: A) => L.Applicative<B>, l: List<B>) => any);
    <A, B>(of: L.Of, f: (a: A) => L.Applicative<B>): (l: List<B>) => any;
};
export declare const equalsWith: typeof L.equalsWith & {
    <A>(f: (a: A, b: A) => boolean): Curried2<List<A>, List<A>, boolean>;
    <A>(f: (a: A, b: A) => boolean, l1: List<A>): (l2: List<A>) => boolean;
};
export declare const reduceRight: typeof foldr;
export declare const update: typeof L.update & {
    <A>(index: number, a: A): (l: List<A>) => List<A>;
    <A>(index: number): ((a: A, l: List<A>) => List<A>) & ((a: A) => (l: List<A>) => List<A>);
};
export declare const adjust: typeof L.adjust & {
    <A>(index: number, f: (value: A) => A): (l: List<A>) => List<A>;
    (index: number): <A>(f: (value: A) => A, l: List<A>) => List<A> & (<A>(f: (value: A) => A) => (l: List<A>) => List<A>);
};
export declare const slice: typeof L.slice & {
    (from: number): (<A>(to: number) => (l: List<A>) => List<A>) & (<A>(to: number, l: List<A>) => List<A>);
    (from: number, to: number): <A>(l: List<A>) => List<A>;
};
export declare const remove: typeof slice;
export declare const insert: typeof update;
export declare const insertAll: typeof L.insertAll & {
    <A>(index: number, elements: List<A>): (l: List<A>) => List<A>;
    <A>(index: number): ((elements: List<A>, l: List<A>) => List<A>) & ((elements: List<A>) => (l: List<A>) => List<A>);
};
export declare const zipWith: typeof L.zipWith & {
    <A, B, C>(f: (a: A, b: B) => C, as: List<A>): (bs: List<B>) => List<C>;
    <A, B, C>(f: (a: A, b: B) => C): Curried2<List<A>, List<B>, List<C>>;
};
export declare const foldlWhile: typeof L.foldlWhile & {
    <A, B>(predicate: (acc: B, value: A) => boolean, f: (acc: B, value: A) => B, initial: B): (l: List<A>) => B;
    <A, B>(predicate: (acc: B, value: A) => boolean, f: (acc: B, value: A) => B): Curried2<B, List<A>, B>;
    <A, B>(predicate: (acc: B, value: A) => boolean): Curried3<(acc: B, value: A) => B, B, List<A>, B>;
};
export declare const reduceWhile: typeof foldlWhile;
