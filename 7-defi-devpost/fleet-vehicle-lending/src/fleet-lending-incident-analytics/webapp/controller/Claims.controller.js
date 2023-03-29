sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Claims", {

		onClaimAnalyticButtonPress: function() {
			var oRoute = this.getOwnerComponent().getRouter();
			// oRoute.navTo(""); add route
		},

		navigationClaimDetail  : function() {
			var oThis = this;
			var m = this.getView().byId("claimsTable");
			var alertFunc = function(oEvent) {
				var oModel = oEvent.getSource().getModel();
				var oContext = oEvent.getSource().getBindingContext();
				var obj = oModel.getProperty("ClaimMasterReference", oContext);
				oThis.getOwnerComponent().getRouter().navTo("claimDetail", {
					claimMasterRef: obj
				});
			};
			var myFunc = function(item) {
				item.setProperty("type", "Navigation");
				item.attachPress(alertFunc);
			};
			var f = m.getTable().getItems();
			f.forEach(myFunc);

		},
		
		onAfterRendering : function() {
			var smartfilterbar = this.getView().byId("smartFilterBarClaims");
			var filterbutton = smartfilterbar.getContent()[0].getContent()[5];
			var showfilterbar = smartfilterbar.getContent()[0].getContent()[2];
			filterbutton.addStyleClass("sapMBtnInner"); 
			filterbutton.addStyleClass("sapMBtnEmphasized");
			filterbutton.setText("");
			filterbutton.setIcon("sap-icon://filter");
			showfilterbar.destroy();
			
			smartfilterbar.onAfterRendering = function() {
				var basicsearch = smartfilterbar.getContent()[0].getContent()[1];
				var showfilterelements = smartfilterbar.getContent()[1].getContent();
				showfilterelements.push(basicsearch); 
			};
			
		}

		// onInit: function() {
		// 	this.router = this.getOwnerComponent().getRouter();
		// 	this.router.getRoute("claims").attachPatternMatched(this._onRouteMatched, this);

		// },

		// _onRouteMatched: function(oEvent) {

		// 	if (oEvent.getParameter("arguments")) {
		// 		var claimYear = oEvent.getParameter("arguments").claimYear;
		// 		var claimsFilter = this.getView().byId("smartFilterBarClaims");
		// 		var filter =
		// 			'{"AccidentYear":{"value":null,"ranges":[{"exclude":false,"operation":"Contains","keyField":"AccidentYear","value1":"' + claimYear +'","value2":"","tokenText":"*2012*"}],"items":[]}}';

		// 		claimsFilter.setFilterDataAsString(filter, true);
		// 	}

		// }

	});

});