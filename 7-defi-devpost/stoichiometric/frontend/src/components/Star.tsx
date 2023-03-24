/** @jsxImportSource theme-ui */
import { useContext } from "react";

import useMousePosition from "utils/useMousePosition";

import { ResponsiveContext } from "contexts/ResponsiveContext";

interface Props {
    top?: string;
    bottom?: string;
    left?: string;
    right?: string;
    color?: string;
    height?: string;
}

const Star: React.FC<Props> = (props) => {
    const mousePosition = useMousePosition();
    const { device, windowSize } = useContext(ResponsiveContext);

    let x: number;
    let y: number;
    let color: string;
    let height: string;

    props.color ? color = props.color : color = "text";
    props.height ? height = props.height : height = "20";

    props.left ? x = parseFloat(props.left) : props.right ? x = parseFloat(props.right) : x = 0;
    props.top ? y = parseFloat(props.top) : props.bottom ? y = parseFloat(props.bottom) : y = 0;

    function translate(x: GLfloat, y: GLfloat) {
        if (x > y) return Math.sqrt(Math.abs(x - y));
        else return -Math.sqrt(Math.abs(x - y));
    }

    const style = {
        movingSquare: {
            height: `${device == "mobile" ? parseFloat(height) / 2 : height}px`,
            aspectRatio: '1',
            background: color,
            animation: 'opacityChange 2.2s infinite alternate',
            position: 'absolute' as 'absolute',
            transition: '.1s',
            top: `${props.top}%`,
            bottom: `${props.bottom}%`,
            left: `${props.left}%`,
            right: `${props.right}%`,
            transform: `Translate(${translate(mousePosition.x, windowSize.width * x / 100)}px,${translate(mousePosition.y, windowSize.height * y / 100)}px)`
        }
    }

    return (
        <div sx={style.movingSquare} />
    )
};

export default Star;