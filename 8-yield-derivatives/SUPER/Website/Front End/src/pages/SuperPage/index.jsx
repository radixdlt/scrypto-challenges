import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import SecondaryNavbar from "../../components/SecondaryNavBar.jsx";
import { Outlet } from "react-router-dom";

function SuperPage() {
    return (
        <>
            <PrimaryNavbar />
            <SecondaryNavbar />
            <main>
                {/* The Outlet will render child routes */}
                <Outlet />
            </main>
        </>
    );
}

export default SuperPage;
