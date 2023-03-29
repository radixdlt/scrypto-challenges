import { useAccounts } from '@/hooks/useAccounts';
import { useManifest } from '@/hooks/useManifest';
import { styles } from '@/styles/CreateProduct.styles';
import { Box, Button, CircularProgress, Input, Typography } from '@mui/material';
import { ChangeEvent, useState } from 'react';


export default function CreateProduct() {

    const [title, setTitle] = useState("");
    const [description, setDescription] = useState("");
    const [raiseAmount, setRaiseAmount] = useState("");
    const [files, setFiles] = useState([]);
    const accounts = useAccounts();

    const { createProduct, isLoading } = useManifest();

    const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>, func: Function) => {
        func(e.target.value);
    };

    const handleFileChange = (e: any) => {
        setFiles(e.target.files);
    }

    const handleClick = async () => {
        if(accounts?.[0]?.address){
            await createProduct(title, description, raiseAmount,files[0]);
        } else {
            alert("Please connect !!!")
        }
    };

    return (
        <>
            <Box sx={styles.wrapper}>
                <Box>
                    <Typography>Product Title</Typography>
                    <Input onChange={(e) => { handleChange(e, setTitle) }} />
                </Box>
                <Box>
                    <Typography>Product Description</Typography>
                    <Input
                        sx={styles.descInput}
                        multiline
                        onChange={(e) => { handleChange(e, setDescription) }} />
                </Box>
                <Box>
                    <Typography>Raise Amount</Typography>
                    <Input type='number' onChange={(e) => { handleChange(e, setRaiseAmount) }} />
                    <Input type='file' onChange={handleFileChange} />
                </Box>
                <Button
                    sx={styles.button}
                    variant='contained'
                    onClick={handleClick}
                    disabled={isLoading}
                >CREATE A PRODUCT</Button>
                {isLoading && <CircularProgress />}
            </Box>
        </>
    )
};
