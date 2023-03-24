sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("emiratespost.controller.Landing", {

		onInit: function() {
			//this.getOwnerComponent().getRouter().getRoute("landing").attachPatternMatched(this._onObjectMatched, this);
		},

		onTilePress: function(oEvent) {
			//this.getOwnerComponent().getModel("busyIndicatorModel").setProperty("/busy", true);
			var navRoute = oEvent.getSource().getProperty("title");
			var oRoute = this.getOwnerComponent().getRouter();
			switch(navRoute){
				case "Blockchain": oRoute.navTo("blockchain"); 
				break;
				case "Track Trace": oRoute.navTo("tracktrace");
				break;
				case "Digital Locker": oRoute.navTo("digitalLockerPost"); 
				break;
				case "Alerts": oRoute.navTo("alerts");
				break;
				case "Locker info": oRoute.navTo("temperature"); 
				break;
				case "Epost Offers Box": oRoute.navTo("epost");
				break;
			}
		},
		
		onObjectMatched: function(){
			
		}
	});
});