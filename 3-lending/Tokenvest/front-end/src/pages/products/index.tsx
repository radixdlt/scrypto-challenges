import { PRODUCTS } from '@/constants/routes';
import { styles } from '@/styles/Products.styles';
import { Box, LinearProgress, Typography } from '@mui/material';
import Link from 'next/link';
import { Fragment } from 'react';
import { CMS_API, CMS_PRODUCTS, CMS_URL, POPULATE_ALL } from '@/constants/cms';
import { ICMSProduct, IProduct } from '@/interfaces/cmsInterface';

export async function getStaticProps() {
    const res = await fetch(`${CMS_API}${CMS_PRODUCTS}${POPULATE_ALL}`);
    const data = await res.json();

    const products: IProduct[] = data?.data?.map((item: ICMSProduct) => {
        return {
            id: item.id,
            title: item.attributes.title,
            description: item.attributes.description,
            raiseAmount: item.attributes.raiseAmount,
            raisedAmount: item.attributes.raisedAmount,
            componentId: item.attributes.componentId,
            ownerAddress: item.attributes.ownerAddress,
            complete: item.attributes.complete,
            image: item.attributes.image?.data?.attributes?.url || null
        }
    }) || [];

    return {
        props: {
            products
        }
    }
}


export default function Products({ products }: { products: IProduct[] }) {
    if (products.length > 0) {
        return (
            <>
                <Box sx={styles.productsWrapper}>
                    {products.map((item, index) => {
                        return (
                            <Fragment key={index + 1}>
                                <Link href={`${PRODUCTS}/${item.id}`}>
                                    <Box sx={!item.complete ? styles.product : { ...styles.product, ...styles.completeProduct }}>
                                        <Box sx={styles.infoWrapper}>
                                            <Box sx={styles.titleDescriptionBox}>
                                                <Typography sx={styles.title}>{item.title}</Typography>
                                                <Typography sx={styles.description}>{item.description}</Typography>
                                            </Box>
                                            <Box sx={styles.linearProgressWrapper}>
                                                <LinearProgress
                                                    sx={styles.linearProgress}
                                                    variant="determinate"
                                                    value={+item.raisedAmount >= +item.raiseAmount ? 100 : +item.raisedAmount * 100 / +item.raiseAmount} />
                                                <Typography sx={styles.raisedStatus}>{item.raisedAmount} / {item.raiseAmount}</Typography>
                                            </Box>
                                        </Box>
                                        {item.image ?
                                            <>
                                                <Box sx={{
                                                    ...styles.image,
                                                    backgroundImage:`url(${CMS_URL}${item.image})`,
                                                }}></Box>
                                            </> : <></>
                                        }
                                        {item.complete ?
                                            <Box sx={styles.completeWrapper}>
                                                <Box sx={styles.completeBox}>
                                                    <Typography>FINISHED</Typography>
                                                </Box>
                                            </Box> : <></>}
                                    </Box>
                                </Link>
                            </Fragment>
                        )
                    })}
                </Box>
            </>
        )
    } else {
        return (
            <>
                <Typography>No Products</Typography>
            </>
        )
    }

};
