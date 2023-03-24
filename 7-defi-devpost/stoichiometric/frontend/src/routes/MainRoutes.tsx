//import MainLayout from 'layout/MainLayout';

// render - pages
import Home from 'pages/Home';
import Swap from 'pages/Swap';
import Liquidity from 'pages/Liquidity';
import Lend from 'pages/Lend';
import Liquidate from 'pages/Liquidiate';
import Dao from 'pages/Dao';
import Card from 'pages/Card';
// ==============================|| MAIN ROUTING ||============================== //

const MainRoutes = [
    {
        path: '/',
        element: <Home />
    }, {
        path: '/swap',
        element: <Swap />
    }, {
        path: '/liquidity',
        element: <Liquidity />
    }, {
        path: '/lend',
        element: <Lend />
    }, {
        path: '/liquidate',
        element: <Liquidate />
    }, {
        path: '/dao',
        element: <Dao />
    },
    {
        path: '/card',
        element: <Card />
    }
];

export default MainRoutes;
