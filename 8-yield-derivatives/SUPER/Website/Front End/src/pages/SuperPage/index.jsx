import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import SecondaryNavbar from "../../components/SecondaryNavBar.jsx";
import { Outlet } from "react-router-dom";
/**
 * SuperPage component that serves as a container for sub-pages for managing and buying SUPER.
 *
 * @returns {JSX.Element} The rendered "Super" page component.
 */
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
