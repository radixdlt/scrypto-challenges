# Tokenvest

## Idea
Tokenvest is an investment deFi platform powered by blockchain, using the **Radix** decentralized network which is the best suit for deFi applications.

With this platform, users will have the opportunity to invest in products and to create a product/idea to get an investment. 
Products are getting stored in the blockchain, and transactions are being made via *RDX* token.

## Application
Application as an MVP consists of 3 pages
1. Home - user details being shown, including name, connected accounts number, and status. 
2. Products - list of created products.
3. Create a Product - create product functionality

## Technology
The architecture consists of 3 main concepts(folders).
1. Smart Contracts - All the Radix smart contracts
2. Frontend - all the UI code, including interactions with smart contracts
3. [strapi](https://strapi.io) - the headless CMS implementation to sync products in our database.

## How to run

### Requirements
NodeJs version 14 or higher
as a test project .env variable is directly pushed to the strapi folder so it will run automatically. 

### Commands
1. yarn
2. yarn setup
3. yarn start:strapi
4. yarn start:frontend

### Wallet
1. setup the [radix mobile application](https://docs-babylon.radixdlt.com/main/getting-started-developers/wallet-and-connector.html)
2. install the [connector extension](https://docs-babylon.radixdlt.com/main/getting-started-developers/wallet-and-connector.html#_install_the_connector)
3. Transfer tokens to your account from radix betanet.
4. connect your account in our platform using the **connect** button in the header

![image](https://user-images.githubusercontent.com/23248910/227128067-6824769e-92c9-4aea-990c-82d5ab1d9097.png)

Specific business logic is described in the video


## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.

