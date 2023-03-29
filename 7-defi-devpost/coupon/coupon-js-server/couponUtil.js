module.exports = {

    generate: function(to_address, id, amount) {
        return new Promise(function (resolve) {
            let reference = btoa(to_address+"$"+amount+"$"+id);
            resolve(reference);
        }).catch(err => {
            console.log('Error: ', err.message);
            reject();
        });;
    },

    decode: function(giftcode) {
        return new Promise(function (resolve) {
            let reference = atob(giftcode);
            console.log('Reference', reference)
            let array = reference.split(/(\d+)/)
            let response = [
                {   "to_address": array[0],
                    "amount": array[1],
                    "id": array[2]
                },              
            ]
            resolve(response);
        }).catch(err => {
            console.log('Error: ', err.message);
            reject();
        });;
    },
}

