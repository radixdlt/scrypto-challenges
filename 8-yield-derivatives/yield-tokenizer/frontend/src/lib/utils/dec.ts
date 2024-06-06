import Decimal from "decimal.js";
import { PreciseDecimal } from "./state_fetcher";

export const dec = (input: any): Decimal => {
    return new Decimal(input ? input : '0');
}

export const pdec = (input: any): Decimal => {
    return new PreciseDecimal(input);
}