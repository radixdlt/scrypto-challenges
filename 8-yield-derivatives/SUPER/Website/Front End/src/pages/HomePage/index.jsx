import "../../App.css"
import DevModeInstruction from "../../sections/DevModeInstruction.jsx"
import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";

/**
 * HomePage component that serves as the main landing page of the DApp.
 *
 * @returns {JSX.Element} The rendered home page component.
 */
function HomePage() {
    return (
        <>
            <PrimaryNavbar />
            <main>
                <DevModeInstruction />
            </main>
        </>
    );
}

export default HomePage;