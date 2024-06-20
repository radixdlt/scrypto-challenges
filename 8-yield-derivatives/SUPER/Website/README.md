> [!NOTE]
> This document explains how to run the DApp, by either using the frontend, backend, and Scrypto on Stokenet or using just the scrypto on `resim`
> - [What am I running?](../README.md)
> - [Scrypto Docs](../Smart%20Contract/README.md)
> - [Front End Docs](./Front%20End/README.md)
> - [Back End Docs](./Back%20End/Server/README.md)


# Run the SUPER DApp

## Introduction

Welcome to the SUPER DApp documentation. This guide provides detailed instructions on how to set up and run the SUPER DApp, including backend and frontend configurations, necessary environment variables, and steps to get everything running locally.

## Table of Contents

1. [Folder Structure](#folder-structure)
2. [Environment Variables](#environment-variables)
3. [Setup and Run the Backend Server](#setup-and-run-the-backend-server)
4. [Setup and Run the Frontend](#setup-and-run-the-frontend)
5. [Common Issues and Troubleshooting](#common-issues-and-troubleshooting)

## Folder Structure

The folder structure of the SUPER DApp project is as follows:

```
.
├── Back End
│   └── Server
│       ├── models
│       │   ├── saleModel.js
│       │   └── nftModel.js
│       ├── routes
│       │   ├── saleRoutes.js
│       │   └── nftRoutes.js
│       ├── .env (NOT INCLUDED)
│       ├── app.js
│       └── goose.js
├── Front End
│   └── public
│   └──src
│   │   └── api  # API calls and integrations
│   │   └── assets  # Images, icons, and other static assets
│   │   └── components  # Reusable components used throughout the application
│   │   └── context  # Context providers for state management
│   │   └── hooks  # Custom React hooks
│   │   └── manifests  # Manifest files for transactions
│   │   └── pages  # Pages and main views
│   │   └──sections  # Sections of pages, larger than components
│   │   └── AccountContext.jsx  # Context for managing account state
│   │   └── App.css  # Global CSS for the application
│   │   └── App.jsx  # Main application component
│   │   └── index.jsx  # Entry point for the application
│   ├── .env  # Environment variables for frontend configuration (NOT INCLUDED)
│   ├── .eslintrc.cjs  # ESLint configuration
│   ├── .gitignore  # Git ignore file
│   ├── index.html  # Main HTML file for the application
│   ├── package.json  # NPM package configuration
│   ├── package-lock.json  # NPM package lock file
│   ├── README.md  # Project documentation
│   └── vite.config.js  # Vite configuration

```

## Environment Variables

### Backend Environment Variables [(`Back End/Server/.env`)]

- **`ENV_ATLAS_URI`**: The MongoDB connection URI.

Example:
```env
ENV_ATLAS_URI=mongodb+srv://user:password@yoo.brtac38.mongodb.net/?retryWrites=true&w=majority&appName=Yoo
```

### Frontend Environment Variables [(`Front End/.env`)]

- **`VITE_BACKEND_BASE_URL`**: The base URL of the backend server.
- **`VITE_PKG_ADDY`**: Package address.
- **`VITE_PUBLISH_TX_ID`**: Publish transaction ID.
- **`VITE_DAPP_ID`**: DApp Definition Address.

Example:
```env
# Base URL of the backend
VITE_BACKEND_BASE_URL=http://localhost:8080

# Package address
VITE_PKG_ADDY=package_tdx_2_1pknesjtssk4vql0aqeap7tgzrdv4vsq745zk0nn6u8xu79e0zwvmyz

# Publish transaction ID
VITE_PUBLISH_TX_ID=txid_tdx_2_1wenp0l6vdkv5fmwaxv552e0p4a8aff9am7sgftp7cm44t0sgujlqpr5l3w

# DApp Definition Address.
VITE_DAPP_ID=account_tdx_2_129f8pjvtzz7hsmaex30z0mtw43yz5l46ccpasy50pra0sd2stv56ws
```

## Setup and Run the Backend Server

1. **Navigate to the Backend Server Directory:**
   ```sh
   cd Back\ End/Server
   ```

2. **Install Dependencies:**
   ```sh
   npm install
   ```

3. **Set Up Environment Variables:**
    - Create a `.env` file in the `Back End/Server` directory.
    - Add your MongoDB URI to the `.env` file as shown in the [Environment Variables](#environment-variables) section.

4. **Start the Backend Server:**
   ```sh
   npm start
   ```

   The backend server should now be running on `http://localhost:8080`.

## Setup and Run the Frontend

1. **Navigate to the Frontend Directory:**
   ```sh
   cd Front End
   ```

2. **Install Dependencies:**
   ```sh
   npm install
   ```

3. **Set Up Environment Variables:**
    - Create a `.env` file in the `Front End` directory.
    - Add the necessary environment variables as shown in the [Environment Variables](#environment-variables) section.
    - Ensure `VITE_BACKEND_BASE_URL` is set to `http://localhost:8080`.

4. **Start the Frontend Development Server:**
   ```sh
   npm run dev
   ```

   The frontend should now be running on `http://localhost:3000`.

## Common Issues and Troubleshooting

1. **Database Connection Issues:**
    - Ensure your MongoDB URI is correct and accessible.
    - Check your network connection.

2. **CORS Issues:**
    - Ensure CORS is properly configured in `Back End/Server/app.js`.

3. **Environment Variables:**
    - Double-check the `.env` files in both backend and frontend directories for any missing or incorrect values.

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
