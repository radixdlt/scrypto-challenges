import { useContext } from "react";

import {
    saleStarted,
    saleCompleted,
    saleStartTimeUnix,
    saleStartTimeUtc,
    saleEndTimeUnix,
    saleEndTimeUtc,
} from "../context/contexts.jsx";

export const useSaleStarted = () => {
    const started = useContext(saleStarted);
    return started;
};

export const useSaleCompleted = () => {
    const completed = useContext(saleCompleted);
    return completed;
};





export const useSaleStartTimeUnix = () => {
    const startTimeUnix = useContext(saleStartTimeUnix);
    return startTimeUnix;
};

export const useSaleStartTimeUtc = () => {
    const startTimeUtc = useContext(saleStartTimeUtc);
    return startTimeUtc;
};

export const useSaleEndTimeUnix = () => {
    const endTimeUnix = useContext(saleEndTimeUnix);
    return endTimeUnix;
};

export const useSaleEndTimeUtc = () => {
    const endTimeUtc = useContext(saleEndTimeUtc);
    return endTimeUtc;
};
