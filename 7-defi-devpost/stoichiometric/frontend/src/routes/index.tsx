import { useRoutes } from 'react-router-dom';

// project import
import MainRoutes from './MainRoutes';

import { UserContext } from 'contexts/UserContext';

import Profile from 'pages/Profile';

import { useContext } from 'react';

// ==============================|| ROUTING RENDER ||============================== //

export default function ThemeRoutes() {
    const { user } = useContext(UserContext);

    return useRoutes(user.address == null ? MainRoutes : MainRoutes.concat({
        path: '/profile',
        element: <Profile />
    }))
}