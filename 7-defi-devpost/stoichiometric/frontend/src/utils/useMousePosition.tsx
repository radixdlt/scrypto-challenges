import React from "react";

const useMousePosition = () => {
    const [
        mousePosition,
        setMousePosition
    ] = React.useState({ x: -1, y: -1 });

    React.useEffect(() => {
        function updateMousePosition(ev: Event) {
            const e = ev as MouseEvent;
            setMousePosition({ x: e.clientX, y: e.clientY });
        }

        window.addEventListener('mousemove', updateMousePosition);

        return () => {
            window.removeEventListener('mousemove', updateMousePosition);
        };
    }, []);

    return mousePosition;
};

export default useMousePosition;