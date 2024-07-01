import "../../App.css"
import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import DocumentationSection from "../../sections/DocumentationSection.jsx";

/**
 * DocsPage component that serves as the documentation page of the DApp.
 *
 * @returns {JSX.Element} The rendered documentation page component.
 */
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