import { useMemo } from 'react';
/*
// Custom hook to find an event by name
const useGetEventInReceipt = (receipt, eventName) => {
    // Use useMemo for performance optimization
    return useMemo(() => {
        if (!receipt) return null;
        const filteredEvents = receipt.filter(event => event.name === eventName);

        // Map to extract only field names and values
        return filteredEvents.map(event =>
            event.data.fields.reduce((acc, field) => ({
                ...acc,
                [field.field_name]: field.value
            }), {})
        );
    }, [receipt, eventName]); // Dependencies to recompute if they change
};

export default useGetEventInReceipt;*/



const useGetEventInReceipt = (receipt, eventName) => {
    return useMemo(() => {
        if (!receipt) return null;
        const filteredEvents = receipt.filter(event => event.name === eventName);

        return filteredEvents.map(event =>
            event.data.fields.reduce((acc, field) => {
                if (field.kind === "Array" && field.field_name === "dapp_definition_caddy") {
                    // Handle single-element array directly as the value
                    if (field.elements.length === 1) {
                        acc[field.field_name] = field.elements[0].value;
                    } else {
                        // If more than one element, return the whole array of values
                        acc[field.field_name] = field.elements.map(element => element.value);
                    }
                } else {
                    // Handle normal fields
                    acc[field.field_name] = field.value;
                }
                return acc;
            }, {})
        );
    }, [receipt, eventName]); // Dependencies to recompute if they change
};

export default useGetEventInReceipt;
