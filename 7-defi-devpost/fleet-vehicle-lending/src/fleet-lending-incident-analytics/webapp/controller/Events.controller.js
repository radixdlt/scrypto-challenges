sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/model/json/JSONModel"
], function(Controller, JSONModel) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Events", {
		
		onInit: function(){
			var oModel = new JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/policeOfficer.json"));
			this.getView().setModel(oModel);
		},
		
		navigationEventDetails: function() {
			var oThis = this;
			var eventDetailsFunc = function(oEvent) {
				var oModel = oEvent.getSource().getModel();
				var oContext = oEvent.getSource().getBindingContext();
				var obj = oModel.getProperty("EventId", oContext); 
				oThis.getOwnerComponent().getRouter().navTo("eventDetails",{eventId:obj});
			};
			var myFunc = function(item) {
				item.setProperty("type", "Navigation");
				item.attachPress(eventDetailsFunc);
			};
			var m = this.getView().byId("eventsTable");
			var f = m.getTable().getItems();
			f.forEach(myFunc);

		},
		
		onEventAnalyticButtonPress: function()
		{
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("analyticsEvent"); 
		},
		
		policeOfficerPressed: function(oEvent){
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("policeDetails", {policeId: 1});
		},
		
				onNavBack: function(oEvent) {
			this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true);
		},
	
	});

});