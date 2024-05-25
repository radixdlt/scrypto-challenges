import "../../App.css"
import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import DocumentationSection from "../../sections/DocumentationSection.jsx";

function DocsPage() {
    return (
        <>
            <PrimaryNavbar />
            <main>
                <DocumentationSection />
            </main>
        </>
    );
}

export default DocsPage;