sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.IncidentsMainApp", {

		/**
		 * Called when a controller is instantiated and its View controls (if available) are already created.
		 * Can be used to modify the View before it is displayed, to bind event handlers and do other one-time initialization.
		 * @memberOf com.sap.Aviation.view.IncidentsMainApp
		 */
		onInit: function() {
			var oModel = new sap.ui.model.json.JSONModel();
			oModel.loadData("IncidentsData");
			this.getView().byId("incidentsList").setModel(oModel);
		},
		onIncidentItemPress: function(oEvent){
			var oRoute = this.getOwnerComponent().getRouter();
			var oModel = this.getView().getModel("IncidentsData").getProperty("/IncidentsReported");
			var oContext = oEvent.getSource().getBindingContextPath().split('/')[2];
			oRoute.navTo("incidentDetails", {
					incidentId: oContext
				});

		}

		/**
		 * Similar to onAfterRendering, but this hook is invoked before the controller's View is re-rendered
		 * (NOT before the first rendering! onInit() is used for that one!).
		 * @memberOf com.sap.Aviation.view.IncidentsMainApp
		 */
		//	onBeforeRendering: function() {
		//
		//	},

		/**
		 * Called when the View has been rendered (so its HTML is part of the document). Post-rendering manipulations of the HTML could be done here.
		 * This hook is the same one that SAPUI5 controls get after being rendered.
		 * @memberOf com.sap.Aviation.view.IncidentsMainApp
		 */
		//	onAfterRendering: function() {
		//
		//	},

		/**
		 * Called when the Controller is destroyed. Use this one to free resources and finalize activities.
		 * @memberOf com.sap.Aviation.view.IncidentsMainApp
		 */
		//	onExit: function() {
		//
		//	}

	});

});