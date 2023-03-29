/** @jsxImportSource theme-ui */

const style = {
    button: {
        border: 'solid .5px',
        borderColor: 'primary',
        fontFamily: 'primary',
        color: 'text',
        fontSize: 1,
        fontWeight: '500',
        background: 'transparent',
        padding: '8px 18px',
        borderRadius: '100px',
        cursor: 'pointer',
        zIndex: '1000',
        transition: '.2s',
        '&:hover': {
            color: 'white',
            background: 'primary'
        }
    }
}

interface Props {
    children?: string;
    onClick?: React.MouseEventHandler<HTMLElement>;
}

const Button: React.FC<Props> = (props) => {
    return (
        <button sx={style.button} onClick={props.onClick}>
            {props.children}
        </button>
    )
}

export default Button;