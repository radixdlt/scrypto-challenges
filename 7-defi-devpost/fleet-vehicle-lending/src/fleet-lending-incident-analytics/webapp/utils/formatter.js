jQuery.sap.declare("com.sap.aviation.analyticspricemterics.utils.formatter");
com.sap.aviation.analyticspricemterics.utils.formatter ={


		numberPriceFormat: function(value) {
			var oLocale = new sap.ui.core.Locale("en-US");

 
 
var oNumberFormatOptions = {
    style: "short",
    decimals: 1,
    shortDecimals: 2
};
 
var oFloatFormat = sap.ui.core.format.NumberFormat.getFloatInstance(oNumberFormatOptions, oLocale);
value =oFloatFormat.format(value); // returns 1.23K (shortified number takes the shortDecimals parameter)
		
			return value +" $";
		

		},
				formatDate: function(state) {
		if(state=="Complete"){
			return "Success";
		}
			else if(state=="In Progress"){
				return "Error";
			}
			else{
				return "None";
			}
		}
		

};