export const useParseMaturity = (dataTimeObj) => {
    // Function to parse the object into a JavaScript Date object
    console.log(dataTimeObj);

      const fields = dataTimeObj;
      console.log(fields);
      const year = fields.find(field => field.field_name === 'year').value;
      const month = fields.find(field => field.field_name === 'month').value;
      const day = fields.find(field => field.field_name === 'day_of_month').value;
      const hour = fields.find(field => field.field_name === 'hour').value;
      const minute = fields.find(field => field.field_name === 'minute').value;
      const second = fields.find(field => field.field_name === 'second').value;
      console.log(day, hour, minute, second);
  
      // JavaScript Date months are 0-indexed, so subtract 1
      return new Date(year, month - 1, day, hour, minute, second);

  }