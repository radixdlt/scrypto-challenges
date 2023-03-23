import { ROUTES } from "@/constants/routes";
import { Box } from "@mui/material"
import Link from "next/link";
import { useRouter } from "next/router";
import { Fragment } from "react";
import { styles } from "./Header.styles";

declare global {
    namespace JSX {
        interface IntrinsicElements {
            "radix-connect-button": React.DetailedHTMLProps<
                React.HTMLAttributes<HTMLElement>,
                HTMLElement
            >;
        }
    }
}

const Header = () => {

    const router = useRouter();

    const activeRoute = router.asPath.split("/")[1];


    return (
        <Box sx={styles.header}>
            <Box sx={styles.linksWrapper}>
                {ROUTES.map((item, index) => {
                    return (
                        <Fragment key={index}>
                            <Link
                                href={item.slug}
                                style={activeRoute === item.slug.replace("/", "") ? { ...styles.link, ...styles.linkActive } : styles.link}
                            >{item.title}</Link>
                        </Fragment>
                    )
                })}
            </Box>
            <Box>
                <radix-connect-button></radix-connect-button>
            </Box>
        </Box>
    );
}

export default Header;