sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Flights", {
		navigationFlightDetails: function() {
				var oThis = this;
				var flightDetailsFunc = function(oEvent) {
					var oModel = oEvent.getSource().getModel();
					var oContext = oEvent.getSource().getBindingContext();
					var obj = oContext.sPath.split('(')[1].split(')')[0];
					oThis.getOwnerComponent().getRouter().navTo("flightDetails", {
						flightID: obj
					});
				};
				var myFunc = function(item) {
					item.setProperty("type", "Navigation");
					item.attachPress(flightDetailsFunc);
				};
				var m = this.getView().byId("flightsTable");
				var f = m.getTable().getItems();
				f.forEach(myFunc);
			}
		/**
		 * Called when a controller is instantiated and its View controls (if available) are already created.
		 * Can be used to modify the View before it is displayed, to bind event handlers and do other one-time initialization.
		 * @memberOf com.sap.Aviation.view.flights
		 */
		//	onInit: function() {
		//
		//	},

		/**
		 * Similar to onAfterRendering, but this hook is invoked before the controller's View is re-rendered
		 * (NOT before the first rendering! onInit() is used for that one!).
		 * @memberOf com.sap.Aviation.view.flights
		 */
		//	onBeforeRendering: function() {
		//
		//	},

		/**
		 * Called when the View has been rendered (so its HTML is part of the document). Post-rendering manipulations of the HTML could be done here.
		 * This hook is the same one that SAPUI5 controls get after being rendered.
		 * @memberOf com.sap.Aviation.view.flights
		 */
		//	onAfterRendering: function() {
		//
		//	},

		/**
		 * Called when the Controller is destroyed. Use this one to free resources and finalize activities.
		 * @memberOf com.sap.Aviation.view.flights
		 */
		//	onExit: function() {
		//
		//	}

	});

});