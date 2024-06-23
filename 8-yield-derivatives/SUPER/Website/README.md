> [!NOTE]
> This document explains how to run the DApp, by either using the frontend, backend, and Scrypto on Stokenet or using just the scrypto on `resim`
> - [What am I running?](../README.md)
> - [Scrypto Docs](../Smart%20Contract/README.md)
> - [Front End Docs](./Front%20End/README.md)
> - [Back End Docs](./Back%20End/Server/README.md)


# Run the SUPER DApp

This guide provides detailed instructions on how to set up and run the SUPER DApp, including backend and frontend 
configurations, necessary environment variables, and steps to get everything running locally.

> [!NOTE]
> For my own sanity, I refer to both functions and methods as functions.

## Table of Contents

1. [Folder Structure](#folder-structure)
2. [Environment Variables](#environment-variables)
3. [Setup and Run the Backend Server](#setup-and-run-the-backend-server)
4. [Setup and Run the Frontend](#setup-and-run-the-frontend)
5. [Common Issues and Troubleshooting](#common-issues-and-troubleshooting)

## Functions used to interact with the SUPER DApp
Regardless of which method you choose to interact with the SUPER DApp, you can only interact with it using a few functions.
the following functions are those that are called from outside the DApp to interact with the DApp.

## Public Functions
These are the functions that the public can access, although some functions become locked/unlocked once certain conditions are met.

- [`deposit()`](../Smart%20Contract/README.md#deposit)
- [`split_yield_nft()`](../Smart%20Contract/README.md#split_yield_nft)
- [`claim_yield()`](../Smart%20Contract/README.md#claim_yield)

## Functions requiring elevated permissions (badges)
When the DApp is instantied using `new()`, two badges are created - the SUPER Owner Badge and SUPER DB Updater Badge.
The Owner Badge has higher "permission levels" than the Database Updater Badge and so can access any functions that the DB Updater can.
- [`new()`](../Smart%20Contract/README.md#new)
  - Technically, this one doesn't require a badge, but it DOES create the badges the rest of the functions require.
- [`start_sale()`](../Smart%20Contract/README.md#start_sale)
- [`end_sale()`](../Smart%20Contract/README.md#end_sale) 
  - NOT RECOMMENDED TO USE, THE COMPONENT WILL CALL THIS ON ITSELF WHEN SOMEONE TRIES TO PURCHASE SUPER AFTER 7 DAYS
- [`update_dbs_to_now()`](../Smart%20Contract/README.md#update_dbs_to_now)
- [`vested_withdraw()`](../Smart%20Contract/README.md#vested_withdraw)

## General DApp Function Flow

### Owner Function Flow
1. **Owner** - Instantiates the DApp using `new()`
2. **Owner** - Starts the token sale using `start_sale()`
3. **Owner** or **DB Updater** - Until the token sale ends, `update_dbs_to_now()` must be run on an hourly basis.
4. **Owner** - On a weekly basis, withdraw vested funds.

### Public Function Flow
1. Once the owner starts the sale, the public can use the `deposit()` method to buy SUPER + SUPERt + SUPER NFT
2. Immediately after they have minted their SUPER NFT, they can call the `split_nft()` method.
3. Once the token sale ends, (After 7 days) participants with a SUPER NFT may `claim_yield()`

## Run the DApp locally using `resim` (Without frontend/backend)

> #### 1. Reset Resim
> ```powershell
> resim reset
> ```

> #### 2. Set the XRD Resource Address
> ```powershell
> $xrd = "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3"
> ```

> #### 3. Create a New Account
> The following script will create the account using `resim new-account` and also set the account address to the variable `$account_address`
> ```powershell
> $account_address = resim new-account | Select-String "Account component address:" | ForEach-Object { $_.Line -replace "Account component address:\s*", "" }
> ```

> #### 4. Publish the Package
> This will publish the package using `resim publish .` and also set the package address to the variable `$package_address`
> ```powershell
> $package_address = resim publish . | Select-String "Success! New Package:" | ForEach-Object { $_.Line -replace "Success! New Package:\s*", "" }
> ```

> #### 5. Instantiate the Component and extract `SaleDetailEvent` custom scrypto event
> This instantiates the DApp using the package and using your own account addresses as the DApp Definition Address. It saves the raw output as the variable`$new_raw`
> ```powershell
> $sale_detail_event = resim call-function $package_address Super new $account_address | Select-String -Pattern "SaleDetailEvent" -Context 0,20
> ```

> #### 6. Parse `SaleDetailEvent` to get component's various addresses
> ```powershell
> # Initialize an empty hashtable to store the extracted fields
> $sale_details = @{}
> 
> # Extract fields from the SaleDetailEvent
> $sale_detail_event.Context.PostContext | ForEach-Object {
>     if ($_ -match "\s*(\w+):\s+(.*)") {
>         $field_name = $matches[1]
>         $field_value = $matches[2]
>         # Check if the field name contains "addy"
>         if ($field_name -match "addy") {
>             if ($field_value -match '"([^"]+)"') {
>                 $field_value = $matches[1]
>             }
>         } elseif ($field_value -match '([^,]+)i64') {
>             $field_value = $matches[1]
>         }
>         if ($field_name -match "owner_badge") {
>             $field_value = "${field_value}:#0#"
>         }
>         $sale_details[$field_name] = $field_value
>     }
> }
> 
> # Display the extracted fields
> foreach ($key in $sale_details.Keys) {
>     Write-Output "${key}: $($sale_details[$key])"
> }
> Write-Output "
> "
> ```
> This will create the following fields within `$sale_details` (the addresses I got may be different from yours):
>> - **Sale Time and Status Variables**
>>   - `sale_start_time_utc`: "Sale hasn't begun"
>>   - `sale_end_time_unix`: 0
>>   - `sale_start_time_unix`: 0
>>   - `sale_end_time_utc`: "Sale hasn't begun"
>>   - `sale_completed`: false
>>   - `sale_started`: false
>> - **Token Resource Addresses**
>>   - `super_t_raddy`: resource_sim1t5d406396cmx60t5g7cvsly8d8wfmagzyz0lyvzk06y0lvwct2rv4q
>>   - `super_raddy`: resource_sim1t4jms3k6cxlzvknnj6hwx06dmuzfaz8q05lw0x599s9ekdgcjhevf9
>>   - `super_y_raddy`: resource_sim1t5d406396cmx60t5g7cvsly8d8wfmagzyz0lyvzk06y0lvwct2rv4q
>>   - `yield_nft_raddy`: resource_sim1ngqnc0uvp3e4q3yr6atqp0ls07lj34kwna4yt4g7ryj4qdm8npq740
>> - **Badge Recourse Addresses**
>>   - `owner_badge_raddy`: resource_sim1nt0fzfanrvjucld6yr798c52zxcpmcvz9aqmm75t2r9kzzwy4wyqpe:#0#
>>   - `db_updater_raddy`: resource_sim1nfnx6tfk4y73g839cvwf4wzjlq9pauaexxqj0txa82du54wajecuac
>>   - `component_badge_raddy`: resource_sim1nfxxxxxxxxxxglcllrxxxxxxxxx002350006550xxxxxxxxxk5870l
>> - **Component Addresses**
>>   - `pool_caddy`: pool_sim1c3acq8pamuyafpnavkt6na3jkju83udrz6kq4jyvfgu5pwxk8jcen6
>>   - `component_caddy`: component_sim1cpwu4wc6rg0am8l9prnh2lzqkk6hue6stzqhdx48rzvek2mmm5vp0p
>> - `dapp_definition_caddy`: [ This one isn't parsed properly but doesn't matter, since we know its = `$account_address`
>
> Any of these variables may be called as follows:
> ```powershell
> $sale_details["component_caddy"]
> ```

> #### 7. Start the token sale
> Now, start the sale, and extract sale_detail_event again using: 
> ```powershell
> $sale_detail_event_1 = resim call-method $sale_details["component_caddy"] start_sale ${xrd}:1 -p $sale_details["owner_badge_raddy"] | Select-String -Pattern "SaleDetailEvent" -Context 0,20
> ```

> #### 8. Parse sale_detail_event_1
> When the sale starts, some fields of saledetailevent must be updated, so we parse it again:
> ```powershell
> $sale_detail_event_1.Context.PostContext | ForEach-Object {
>     if ($_ -match "\s*(\w+):\s+(.*)") {
>         $field_name = $matches[1]
>         $field_value = $matches[2]
>         # Check if the field name contains "addy"
>         if ($field_name -match "addy") {
>             if ($field_value -match '"([^"]+)"') {
>                 $field_value = $matches[1]
>             }
>         } elseif ($field_value -match '([^,]+)i64') {
>             $field_value = $matches[1]
>         }
>         if ($field_name -match "owner_badge") {
>             $field_value = "${field_value}:#0#"
>         } elseif ($field_name -match "db_updater") {
>             $field_value = "${field_value}:#0#"
>         }
>         $sale_details[$field_name] = $field_value
>     }
> }
> 
> # Display the extracted fields
> foreach ($key in $sale_details.Keys) {
>     Write-Output "${key}: $($sale_details[$key])"
> }
> Write-Output "
> "
> ```


> #### 9. Buy SUPER
> Now that the sale has started, we can perform a `deposit()` and take the `create_yield_nft_event` to get the nft's data.
> ```powershell
> $nft_data = @{}
> 
> $create_yield_nft_event = resim call-method $sale_details["component_caddy"] deposit ${xrd}:100 | Select-String -Pattern "CreateYieldNFTEvent" -Context 0,20
> ```

> #### 10. Parse NFT Data
> ```powershell
> $create_yield_nft_event.Context.PostContext | ForEach-Object {
>     if ($_ -match "\s*(\w+):\s+(.*)") {
>         $field_name = $matches[1]
>         $field_value = $matches[2]
>         # Check if the field name contains "addy"
>         if ($field_value -match '([^,]+)u64') {
>             $field_value = $matches[1]
>         } elseif ($field_value -match '"([^"]+)"') {
>             $field_value = $matches[1]
>         }
>         if ($field_name -match 'nft_id' -or $field_name -match 'n_trust_minted' -or $field_name -match 'n_super_minted' -or $field_name -match 'hour_of_mint') {
>             $nft_data[$field_name] = $field_value
>         }
>     }
> }
> 
> # Display the extracted fields
> foreach ($key in $nft_data.Keys) {
>     Write-Output "${key}: $($nft_data[$key])"
> }
> Write-Output "
> "
> 
> $hashtag="#"
> $yield_nft_raddy = $sale_details["yield_nft_raddy"]
> $yield_nft_id = $nft_data["nft_id"]
> $full_nft_id = "${yield_nft_raddy}:${hashtag}${yield_nft_id}${hashtag}"
> ```
 
> #### Advance time by 8 days, updating databases every 24 hours
> ```powershell
> $current_time_raw = resim show-ledger | Select-String "Current Time:"
> 
> # Extract the current time value
> if ($current_time_raw -match "Current Time:\s+(.+)$") {
>     $current_time = $matches[1]
>     Write-Output "Current Time: $current_time"
> } else {
>     Write-Output "Current Time not found in the ledger output."
> }
> 
> $current_time_dt = [DateTimeOffset]::ParseExact($current_time, "yyyy-MM-ddTHH:mm:ssZ", $null)
> 
> for ($i = 0; $i -lt 8; $i++) {
>     $current_time_dt = $current_time_dt.AddHours(24)
>     $new_time_str = $current_time_dt.ToString("yyyy-MM-ddTHH:mm:ssZ")
>     
>     resim set-current-time $new_time_str
>     resim call-method $sale_details["component_caddy"] update_dbs_to_now -p $sale_details["owner_badge_raddy"]
>     Start-Sleep -Seconds 0.025
> }
> ```
 
> #### Split your NFT in half 
> ```powershell
> resim call-method $sale_details["component_caddy"] split_yield_nft $full_nft_id 2
> $full_nft_id_1 = "${yield_nft_raddy}:${hashtag}1${hashtag}"
> $full_nft_id_2 = "${yield_nft_raddy}:${hashtag}2${hashtag}"
> ```
> This will burn NFT 0 create 2 new NFTs (1 and 2), with 50 SUPER each.

> #### Claim yield on NFT 1
> ```powershell
> $super_t_raddy = $sale_details["super_t_raddy"]
> resim call-method $sale_details["component_caddy"] claim_yield $full_nft_id_1 ${super_t_raddy}:60
> ```

# Deploy the website to interact with the DApp on Stokenet

## Prerequisites
- A MongoDB account (Make sure you set up permissions to access from your IP)
- `npm`

## Environment Variables
There are variables stored in both the front end and the back end.
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
