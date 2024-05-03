import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import BuySuperSection from "../../sections/BuySuperSection.jsx";
import SecondaryNavbar from "../../components/SecondaryNavBar.jsx";

function ManageSuperPage() {
    return (
        <>
            <PrimaryNavbar />
            <SecondaryNavbar />
            <main>
                <BuySuperSection />
            </main>
        </>
    );
}

export default ManageSuperPage;