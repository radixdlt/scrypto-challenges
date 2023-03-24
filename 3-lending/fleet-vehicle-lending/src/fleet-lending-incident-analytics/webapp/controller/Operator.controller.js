sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Operator", {

			onOperatorTilePress:function(oEvent) {
				
				var navRoute = oEvent.getSource().getHeader();
				var oRoute = this.getOwnerComponent().getRouter();
				var obj ="Emirates";
				switch(navRoute)
				{
					case "Financial data": oRoute.navTo("operatorFinancialData",{operatorID: obj}); break;
					case "Claims": oRoute.navTo("claims",{operatorID: obj}); break;
					case "Aircrafts": break; //carousal page
					case "Flights": oRoute.navTo("flights",{operatorID: obj}); break;
					case "Delays": oRoute.navTo("operatorDelays",{operatorID: obj}); break;
					case "Analytics": oRoute.navTo(""); break; 
					default : oRoute.navTo("events",{operatorID: obj});
				}
				
			},
			
			onInit: function() {
				this.getView().byId("smartFormFlightData").bindElement("/policies('1')");
			}

	});

});