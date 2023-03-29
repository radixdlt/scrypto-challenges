sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/m/MessageToast"
], function(Controller, MessageToast) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Operators", {

		onOperatorsButtonPress: function() {
			var obj = "Emirates";
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("operator", {
				operatorID: obj
			});
		},

		onOverviewButtonPress: function() {
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("overview", {
				industryOverview: "Aviation"
			});
		},

		onPressCompare: function(oEvent) {
			var oThis = this;
			var numofItems = 0;
			var m = this.getView().byId("operatorsTable");
			var items = m.getItems();
			var operatorCode = "";
			for (var itemIndex = 0; itemIndex < items.length; itemIndex++) {
				if (items[itemIndex].getCells()[4].getSelected()) {
					operatorCode = operatorCode + items[itemIndex].getCells()[0].getText()+ ",";
					numofItems ++;
				}
			}
			if(numofItems === 2)
			{
			oThis.getOwnerComponent().getRouter().navTo("operatorsComparison",{
				operatorCode : operatorCode
			});
			}
			else
			{
				MessageToast.show("Two items need to be selected in order to compare.");
			}
		},
		
		onAfterRendering : function(){
			var smartTableOperators = this.getView().byId("operatorSmartTable");
			var operatorToolBar = smartTableOperators.getItems()[0];
			operatorToolBar.destroy();
		}

	});

});