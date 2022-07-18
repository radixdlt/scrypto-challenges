export const convertFromWei = (a, rounded=5) => {
  let num = window.web3.utils.fromWei(a);
  var re = new RegExp('^-?\\d+(?:\.\\d{0,' + (rounded || -1) + '})?');
  return parseFloat(num.toString().match(re)[0]);
}

export const convertToWei = (a) => {
  return window.web3.utils.toWei(a);
}

const convertToCSV = (objArray) => {
  let array = typeof objArray != 'object' ? JSON.parse(objArray) : objArray;
  let str = '';

  // Arr => comma separated string
  for (let i = 0; i < array.length; i++) {
    let line = '';
    for (let index in array[i]) {
      if (line !== '') line += ','
      line += array[i][index];
    }
    str += line + '\r\n';
  }
  return str;
}

const exportCSVFile = (headers, items, fileTitle) => {
  if (headers) {
    // Inserts at beginnning
    items.unshift(headers);
  }
  // Convert Object to JSON
  let jsonObject = JSON.stringify(items);

  let csv = convertToCSV(jsonObject);

  let exportedFilenmae = fileTitle + '.csv' || 'export.csv';

  let blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
  if (navigator.msSaveBlob) { // IE 10+
    // Saves file/blob to disk
    navigator.msSaveBlob(blob, exportedFilenmae);
  } 
  else {
    let link = document.createElement("a");
    if (link.download !== undefined) { // feature detection
      // Browsers that support HTML5 download attribute
      let url = URL.createObjectURL(blob);
      link.setAttribute("href", url);
      link.setAttribute("download", exportedFilenmae);
      link.style.visibility = 'hidden';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    }
  }
}

const fileTitle = 'Transactions';

const headers = {
  tx_hash: 'Txn Hash',
  tx_method: 'Method',
  tx_age: 'Date Time (UTC)',
  tx_value: 'Value',
  tx_fee: '[Txn Fee]'
};

export const downloadCSV = (arr) => {
  let itemsFormatted = [];

  // format the data
  arr.forEach((item) => {
    itemsFormatted.push({
      tx_hash: item.hash.replace(/,/g, ''),
      tx_method: item.method.replace(/,/g, ''),
      tx_age: item.age.replace(/,/g, ''),
      tx_value: item.value,
      tx_fee: item.fee,
    });
  });

  exportCSVFile(headers, itemsFormatted, fileTitle);
};