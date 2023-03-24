sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Portfolio", {

		navigationCovers: function() {
				var oThis = this;
				var m = this.getView().byId("policiesTable");
				var alertFunc = function(oEvent) {
					var oModel = oEvent.getSource().getModel();
					var oContext = oEvent.getSource().getBindingContext();
					var obj = oModel.getProperty("PolicyTitle", oContext);
					oThis.getOwnerComponent().getRouter().navTo("policy", {
						policyTitle: obj
					});
				};
				var myFunc = function(item) {
					item.setProperty("type", "Navigation");
					item.attachPress(alertFunc);
				};
				var f = m.getTable().getItems();
				f.forEach(myFunc);

			},
		
		onInit: function() {
			this.router = this.getOwnerComponent().getRouter();
			this.router.getRoute("portfolio").attachPatternMatched(this._onRouteMatched, this);

		},

		_onRouteMatched: function(oEvent) {

			if (oEvent.getParameter("arguments")) {
				var pId = oEvent.getParameter("arguments").policyTitle;

			}

		},
		
		onPortfolioAnalyticButtonPress: function()
		{
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("analyticsPortfolio"); 
		}

	});

});