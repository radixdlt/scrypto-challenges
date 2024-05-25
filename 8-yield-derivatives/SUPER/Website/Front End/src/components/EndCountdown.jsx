import { useState, useEffect } from 'react';
import {useEndTimeUnix, useSaleCompleted} from "../hooks/useComponentDetails.js";

function EndCountdown() {

    const endUnix = useEndTimeUnix();

    const saleCompleted = useSaleCompleted();

    const [timeLeft, setTimeLeft] = useState('');

    useEffect(() => {
        // Update the countdown every second
        const timer = setInterval(() => {
            const now = new Date();
            const endDate = new Date(endUnix * 1000); // Convert Unix time to Date object
            const difference = endDate - now;

            if (difference > 0) {
                // Time calculations for days, hours, minutes and seconds
                const days = Math.floor(difference / (1000 * 60 * 60 * 24));
                const hours = Math.floor((difference % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
                const minutes = Math.floor((difference % (1000 * 60 * 60)) / (1000 * 60));
                const seconds = Math.floor((difference % (1000 * 60)) / 1000);

                // Display the result in the format: 2d 10h 34m 56s
                setTimeLeft(`${days}d ${hours}h ${minutes}m ${seconds}s`);
            } else {
                setTimeLeft('Time is up!');
                clearInterval(timer); // Clear timer when countdown is over
            }
        }, 1000);

        // Clean up the timer when the component is unmounted
        return () => clearInterval(timer);
    }, [endUnix]); // Only rerun the effect if endDateUnix changes

    if (!endUnix || timeLeft === 'Time is up!' || timeLeft==='' || saleCompleted) {
        return null;
    }

    return (
        <p className="countdown-text">{timeLeft} Remaining</p>

    );
}

export default EndCountdown;