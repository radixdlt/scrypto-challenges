jQuery.sap.require("com.sap.Aviation.utils.formatter");

sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History",
	"sap/ui/model/json/JSONModel"
], function(Controller, History, JSONModel) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.incidentStatus", {
		onInit: function() {
			var oModel = this.getOwnerComponent().getModel("IncidentsData");
			this.getView().byId("incidentsList").setModel(oModel);
			var oRouter = sap.ui.core.UIComponent.getRouterFor(this);
			oRouter.getRoute("incidentStatus").attachMatched(this._onRouteMatched, this);
		},
		
		_onRouteMatched: function(oEvent){
			this.getOwnerComponent().getModel("IncidentsData").refresh();
		},
		onNavBack: function(oEvent) {
			// var oHistory, sPreviousHash;
			// oHistory = History.getInstance();
			// sPreviousHash = oHistory.getPreviousHash();
			// if (sPreviousHash !== undefined) {
			// 	window.history.go(-1);
			// } else {
			// 	this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true);
			// }
			
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("dashboardDetail");
		},

		onIncidentItemPress: function(oEvent) {
			var oRoute = this.getOwnerComponent().getRouter();
			var selectedIncident = oEvent.getSource().getBindingContext().getProperty("eventNum");
			oRoute.navTo("incidentCustomer", {
				eventId: selectedIncident,
				incident: "old"
			}, true);
		}

	});

});