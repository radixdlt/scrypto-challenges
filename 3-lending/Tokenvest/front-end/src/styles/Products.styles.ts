export const styles = {
    productsWrapper: {
        display: "flex",
        flexDirection: "column",
        gap: "16px"
    },
    product: {
        width: "100%",
        height: "200px",
        borderRadius: "16px",
        background: "rgba(0,74,128,0.2)",
        padding: "16px",
        display: "flex",
        justifyContent: "space-between",
        cursor: "pointer",
        position: "relative",
        ":hover": {
            background: "rgba(0,74,128,0.15)",
        }
    },
    completeProduct: {
        background: "rgba(0,74,128,0.15)",
        opacity: "0.7"
    },
    infoWrapper: {
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-between",
    },
    titleDescriptionBox: {
        display: "flex",
        flexDirection: "column",
        gap: "8px"
    },
    title: {
        fontWeight: "bold",
        fontSize: "24px",
        color: "rgb(13, 56, 116)"
    },
    description: {
        fontSize: "16px",
        color: "rgb(13, 56, 116)",
        display: '-webkit-box',
        overflow: 'hidden',
        WebkitBoxOrient: 'vertical',
        WebkitLineClamp: 3,
        width: "70%"
    },
    linearProgressWrapper: {
        height: "16px",
        width: "100%",
        display: "flex",
        alignItems: "center",
        gap: "16px"
    },
    linearProgress: {
        width: "300px",
        height: "100%",
        borderRadius: "8px"
    },
    raisedStatus: {
        fontWeight: "semibold",
        color: "rgb(93,149,219)"
    },
    completeWrapper: {
        position: "absolute",
        top: "0",
        left: "0",
        width: "100%",
        height: "100%",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
    },
    completeBox: {
        width: "200px",
        height: "120px",
        background: "white",
        borderRadius: "36px",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        boxShadow: "0px 0px 20px #888888",
        ">p": {
            fontWeight: "bold",
            color: "rgb(13, 56, 116)"
        }
    },
    finishedText: {
        fontWeight: "bold",
        fontSize: "36px",
        color: "rgb(13, 56, 116)"
    },
    image: {
        height: "100%",
        width: "300px",
        backgroundSize: "100% 100%",
        borderRadius: "16px"
    },
    bigImage: {
        width: "50%",
        height: "400px",
        backgroundSize: "100% 100%",
        borderRadius: "16px",
        mb: "16px"
    }
}