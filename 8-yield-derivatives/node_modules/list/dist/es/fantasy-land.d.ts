export * from "./index";
declare module "./index" {
    interface List<A> {
        "fantasy-land/equals"(l: List<A>): boolean;
        "fantasy-land/map"<B>(f: (a: A) => B): List<B>;
        "fantasy-land/of"<B>(b: B): List<B>;
        "fantasy-land/ap"<B>(f: List<(a: A) => B>): List<B>;
        "fantasy-land/chain"<B>(f: (a: A) => List<B>): List<B>;
        "fantasy-land/filter"(predicate: (a: A) => boolean): List<A>;
        "fantasy-land/empty"(): List<any>;
        "fantasy-land/concat"(right: List<A>): List<A>;
        "fantasy-land/reduce"<B>(f: (acc: B, value: A) => B, initial: B): B;
        "fantasy-land/traverse"<A, B>(of: Of, f: (a: A) => Applicative<B>): any;
    }
}
