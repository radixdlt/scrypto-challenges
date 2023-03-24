#How to run the website ?
 - Download the frontend folder
 - Open it in any terminal
 - run `npm install`
 - run `npm start`
 - Open your browser and go to localhost:3000

The frontend is build in TSX and uses the React framework. It is divided in three main parts.

1. The contexts

The contexts allows the website to keep in mind some variables through the pages. We have 6 differents contexts :
 - BurgerContext keeps in mind the status of the burger menu for the mobile version (open or closed)
 - ResponsiveContext gives us the width and height of the screen and the device we are using (mobile, laptop, ...)
 - SnackbarContext is used to keep track of the different alerts we need to show ("Logged in", "Logged out", "Transaction failed", ...)
 - ThemeContext is used to keep in mind the theme of the website (light, dark)
 - TokensContext gives us the list of tokens, pools, lenders, dao proposals, ... In short, everything related to the blockchain but not directly related to the user
 - UserContext is used to have every informations needed about the current user (account address, positions, tokens owned, ...)


2. The routes, pages and components

We use ReactRouter to route the url to a page. The important file is "src/routes/index.tsx". This file maps every url to a page (React component exported from "pages/x/index.tsx").
For every page, the style is an object returned by a function exported from "pages/x/style.tsx". We use in-js style in order to deal more easily with themes and variables.
Finaly, some component are used one multiple pages, you can find them in "components/". The tricky ones are Dashboard and ConnectWallet :
 - Dashboard is the left menu of every page. We pass the page as a child of the dashboard and this component does the rest.
 - ConnectWallet is our custom vertion of the Connect Wallet button from Radix. We used the Radix Dapp Toolkit.


3. Utils

Everything related to API Calls, Blockchain connection, ... is in there. 
 - The Radix Dapp Toolkit related functions are in "utils/connectToWallet".
 - The main features are splited in folders:
  - DAO for the DAO-related Api calls and manifests like having the list of proposals, send a new proposal, ...
  - DEX for the DEX-related Api calls and manifests (list of pools, swap, ...)
  - General for the General Api calls (list of tokens, ...) and a few constants and functions
  - StableCoin for the StableCoin-related Api calls and manifests (get loan informations, liquidate, ...)