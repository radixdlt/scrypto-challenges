sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.FlightDetails", {
		onInit: function() {
			this.router = this.getOwnerComponent().getRouter();
			this.router.getRoute("flightDetails").attachPatternMatched(this._onRouteMatched, this);
		},
		_onRouteMatched: function(oEvent) {
				if (oEvent.getParameter("arguments")) {
					var fId = oEvent.getParameter("arguments").IDENTIFIER;
					this.getView().byId("smartFormFlightData").bindElement("/flightRoute('"+fId+"')");
					this.getView().byId("flightId").setText("EK180");

				}
			},
		
		onFlighDetailTilePress:function(oEvent) {
				
				var navRoute = oEvent.getSource().getProperty("title");
				var oRoute = this.getOwnerComponent().getRouter();
				var obj ="Emirates";
				switch(navRoute)
				{
					case "Aircraft": oRoute.navTo("aircraft",{aircraftId: obj}); break;
					case "Analytics-simulation": break;
					case "Operator": oRoute.navTo("operator",{operatorID: obj}); break;
				}
				
			}

	});

});