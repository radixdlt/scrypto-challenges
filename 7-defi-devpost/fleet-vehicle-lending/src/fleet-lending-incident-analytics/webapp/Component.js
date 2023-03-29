sap.ui.define([
	"sap/ui/core/UIComponent",
	"sap/ui/Device",
	"com/sap/Aviation/model/models"
], function(UIComponent, Device, models) {
	"use strict";

	return UIComponent.extend("com.sap.Aviation.Component", {

		metadata: {
			manifest: "json"
		},

		/**
		 * The component is initialized by UI5 automatically during the startup of the app and calls the init method once.
		 * @public
		 * @override
		 */
		init: function() {
			// call the base component's init function
			UIComponent.prototype.init.apply(this, arguments);

			// set the device mode
			this.setModel(models.createDeviceModel(), "device");
			var oModel = new sap.ui.model.json.JSONModel({
				"TotalItems": 0,
				"IncidentSet": []
			});
			this.setModel(oModel, "incidentStatusModel");
			//initialize router
			this.getRouter().initialize();  
			
			//userapi
			var userModel = new sap.ui.model.json.JSONModel("/services/userapi/currentUser");  
    		this.setModel(userModel, "userapi");
    		var currentRole = new sap.ui.model.json.JSONModel({
				"CurrentRole": "Citizen"
			});
			this.setModel(currentRole, "currentRoleModel"); 
			 var mlData = new sap.ui.model.json.JSONModel({
				"comments": ""
			});
			this.setModel(mlData, "MLModel");
			var sidenavModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/sideNav.json"));  
    		this.setModel(sidenavModel, "sideNav");
		}
	});
});