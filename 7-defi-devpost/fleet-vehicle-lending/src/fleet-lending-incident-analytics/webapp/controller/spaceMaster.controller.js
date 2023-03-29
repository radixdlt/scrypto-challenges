sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.spaceMaster", {

	sideNavPress: function(oEvent) {
			
			var oRoute = this.getOwnerComponent().getRouter();
			var oModel = this.getView().getModel("sideNav").getProperty("/SideNavList");
			var oContext = oEvent.getSource().getBindingContextPath().split('/')[2];
			var obj = oModel[oContext].title;
			switch(obj){
				case "Dashboard" : oRoute.navTo("dashboardSpace"); break;
				case "Overview" : oRoute.navTo("overview", {industryOverview: "Space"}); break;
			}

		}

	});

});