import { useContext, useEffect } from 'react';
import {UpdateTriggerContext} from "../context/updateTriggerContext.jsx";

/**
 * Custom hook to update sale details.
 *
 * When the component using this hook is mounted, it automatically triggers an update of sale details.
 */
export const useUpdateSaleDetails = () => {
    const { update } = useContext(UpdateTriggerContext);

    useEffect(() => {
        update();  // Trigger an update on component mount
    }, []);
};