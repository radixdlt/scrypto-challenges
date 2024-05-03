import mongoose from "mongoose";

const username = 'nft_creator';
const pw = '0NShiIwcdkZO5arz';


const connectToDatabase = async () => {
    const uri = String.raw`mongodb+srv://${username}:${pw}@yoo.brtac38.mongodb.net/nft_ids?retryWrites=true&w=majority`

    try {
        if (mongoose.connection.readyState === 1) {
            console.log('Already Connected to MongoDB using Mongoose');
        } else {
            await mongoose.connect(uri);
            console.log('Connected to MongoDB using Mongoose');
        }
    } catch (error) {
        console.error('Failed to connect to MongoDB', error);
    }
}

export default connectToDatabase;
