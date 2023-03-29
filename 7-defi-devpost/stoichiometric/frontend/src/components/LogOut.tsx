/** @jsxImportSource theme-ui */

import { useContext } from "react";

import { UserContext } from "contexts/UserContext";

function LogOut() {

    const { connectUser } = useContext(UserContext);

    const style = {
        logout: {
            color: 'white',
            fontSize: 1,
            fontFamily: 'primary',
            border: 'none',
            background: 'primary',
            borderRadius: '5px',
            width: '100px',
            padding: '5px 10px',
            cursor: 'pointer',

            '&:hover': {
                opacity: '.5'
            }
        }
    };

    return (
        <button sx={style.logout} onClick={connectUser}>Log out</button>
    )
}

export default LogOut;