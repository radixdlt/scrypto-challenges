import "../../App.css"
import DevModeInstruction from "../../components/DevModeInstruction.jsx"
import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";

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