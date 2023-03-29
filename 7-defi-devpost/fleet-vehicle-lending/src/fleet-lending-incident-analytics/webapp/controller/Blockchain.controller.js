sap.ui.define(["sap/ui/core/mvc/Controller", "sap/ui/core/routing/History"], function(Controller, History) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Blockchain", {
			onNavBack: function(oEvent) {
			var oHistory, sPreviousHash;
			oHistory = History.getInstance();
			sPreviousHash = oHistory.getPreviousHash();
			if (sPreviousHash !== undefined) {
				window.history.go(-1);
			} else {
				this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true /*no history*/ );
			}
		}
	});
});