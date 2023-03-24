const giftCardObject = require('./couponUtil.js');

//Method to return giftcode
exports.generate= function(req, res) {
    let body = '';

    req.on('data',  function (chunk) {
      body += chunk.toString();
    });
 
    req.on('end', function () {
        console.log(body);
        let amount = JSON.parse(body).giftcode.amount
        let id = JSON.parse(body).giftcode.id
        let to_address = JSON.parse(body).giftcode.to_address

        console.log('Amount', amount);
        console.log('Id', id);
        console.log('To address', to_address);

        res.statusCode = 201;
        res.setHeader('content-Type', 'Application/json');
        giftCardObject.generate(to_address, id, amount).then((giftcode) => {
            console.log(giftcode);
            let response = 
                {   "status": 200,
                    "message": "Successfully generated a code from URL",
                    "giftcode": giftcode
                }
            res.end(JSON.stringify(response));
        });        

    });
    
}

// Method to decode coupon
exports.decode= function(req, res) {
    let body = '';

    req.on('data',  function (chunk) {
      body += chunk.toString();
    });
 
    req.on('end', function () {
        console.log(body);
        let giftcode = JSON.parse(body).redeemcode.giftcode

        console.log('Gift Code', giftcode);

        res.statusCode = 201;
        res.setHeader('content-Type', 'Application/json');
        giftCardObject.decode(giftcode).then((decodeResponse) => {
            console.log(giftcode);
            let response = 
                {   "status": 200,
                    "message": "Successfully decoded payment details",
                    "paymentDetails": decodeResponse
                }
            res.end(JSON.stringify(response))
        });        

    });
    
}

// Method to catch wrong requests 
exports.invalid = function(req, res) {
    var response = [
      {
      "message": "Gift code can not be generated"
      },
      availableEndpoints
    ]
    res.statusCode = 404;
    res.setHeader('content-Type', 'Application/json');
    res.end(JSON.stringify(response))
}

  
