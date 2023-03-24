sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/model/json/JSONModel"
], function(Controller, JSONModel) {
	"use strict";
	var _timeout;
	return Controller.extend("com.sap.Aviation.controller.policeDetails", {
		
		onInit: function(){
			var oModel = new JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/training.json"));
			this.getView().setModel(oModel,"trainings");
		},
		onEnrollTraining: function(oEvent) {
		//	var that = this;
				//	that.showStatusChangeDialog("Success", "Enrolled Successfully");
			 window.open("https://flpnwc-a83bd0407.dispatcher.hana.ondemand.com/sites?siteId=3f244cbf-387d-4b5c-a91f-9dee357f34dd");
		},
		showStatusChangeDialog: function(msgState, msgText){
				var dialog = new sap.m.Dialog({
					title: msgState,
					type: 'Message',
					state: msgState,
					content: new sap.m.Text({
						text: msgText
					}),
					beginButton: new sap.m.Button({
						text: 'OK',
						press: function () {
							dialog.close();
						}
					}),
					afterClose: function() {
						dialog.destroy();
					}
				});

			dialog.open();
		},
		
		onNavBack: function(oEvent) {
			this.getOwnerComponent().getRouter().navTo("events", {}, true);
		},
			
	});

});