import { useState } from 'react';
import {UpdateTriggerContext} from '../contexts.jsx';


// Provider component
// eslint-disable-next-line react/prop-types
export const UpdateTriggerProvider = ({ children }) => {
    const [trigger, setTrigger] = useState(0); // initial value 0

    const update = () => {
        console.log("Update trigger pulled, bullet left the chamber");
        setTrigger(prev => prev + 1); // increment to trigger update
    };

    return (
        <UpdateTriggerContext.Provider value={{ trigger, update }}>
            {children}
        </UpdateTriggerContext.Provider>
    );
};