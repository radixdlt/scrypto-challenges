import { useMemo } from 'react';

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

export default useGetEventInReceipt;
