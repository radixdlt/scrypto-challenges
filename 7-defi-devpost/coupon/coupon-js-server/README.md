## POC server app for generating gift code coupons
This app is generating encrypted gift code based on user id, amount and address to send to.
Codes are encrypted with btoa and this can be later improved for a security reasons.
Generated reference code can be shared and later executed as a payment transaction with Radix network and transaction will require further approvement.

### Start a server

Vanilla Node.js app

```npm install```

```node server```


### API endpoint
```POST /giftcode```

```json body: { "giftcode" : { "amount" : "40", id: "id", "to_address": "to_address" } }```

#### Expectations

if you trigger a request from your local:\
\
```http://127.0.0.1:3002/giftcode```\
with a body:\
```{ "giftcode" : { "amount" : "40", "id": "id", "to_address": "to_address" } }```\
you should expect a response with a payment link inside:\
\
[
    {
        "message": "Successfully generated a code from URL",
        "giftcode": "dG9fYWRkcmVzczQwaWQ="
    }
]

### API endpoint
 POC concept which will require further development for security and integration reasons.

```POST /redeemcode```

```json body: { "redeemcode" : { "giftcode" : "dG9fYWRkcmVzczQwaWQ=" } }```

#### Expectations

if you trigger a request from your local:\
\
```http://127.0.0.1:3002/redeem/code```\
with a body:\
```{ "redeemcode" : { "giftcode" : "dG9fYWRkcmVzczQwaWQ=" } }```\
you should expect a response with a payment link inside:\
\
[
    {
        "message": "Successfully decoded payment details",
        "paymentDetails" : { "amount" : "40", "id": "id", "to_address": "to_address" }
    }
]