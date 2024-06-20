import { useMemo } from 'react';

/**
 * Custom hook to find an event by name in a generic receipt.
 * This hook is used to extract and filter events by name from the events received from useSendTransaction.
 *
 * @param {Array} receipt - The generic receipt array.
 * @param {string} eventName - The name of the event to filter by.
 * @returns {Array|null} An array of filtered events, or null if receipt is not available.
 */
const useGetEventInReceipt = (receipt, eventName) => {
    return useMemo(() => {
        if (!receipt) return null;

        // Filter events by name
        const filteredEvents = receipt.filter(event => event.name === eventName);

        // Map to extract field names and values, with special handling for arrays
        return filteredEvents.map(event =>
            event.data.fields.reduce((acc, field) => {
                if (field.kind === "Array" && field.field_name === "dapp_definition_caddy") {

                    // Handle a single-element array directly as the value
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
