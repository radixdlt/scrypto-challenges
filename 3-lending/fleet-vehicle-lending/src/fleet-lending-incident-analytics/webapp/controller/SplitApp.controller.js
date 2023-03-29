sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.SplitApp", {

		onInit: function() {
			var oModel = new sap.ui.model.json.JSONModel();
			oModel.loadData("sideNav");
			this.getView().byId("SideNavList").setModel(oModel);
			
		},

		sideNavPress: function(oEvent) {
			
			var oRoute = this.getOwnerComponent().getRouter();
			var oModel = this.getView().getModel("sideNav").getProperty("/SideNavList");
			var oContext = oEvent.getSource().getBindingContextPath().split('/')[2];
			var obj = oModel[oContext].title;
			switch(obj){
				case "Dashboard" : oRoute.navTo("dashboardDetail"); break;
				case "Sentiments" : oRoute.navTo("sentiments"); break;
				//case "ICM" : oRoute.navTo("operators");break;
				case "Health" :   oRoute.navTo("events");break;
				case "Aircrafts" : oRoute.navTo("aircrafts");break;
				//case "Analytics" : oRoute.navTo("analytics");break;
				case "Social" : oRoute.navTo("social");break;
				case "BlockChain" : window.open("https://blockchain-cockpit.cfapps.eu10.hana.ondemand.com/org/530d818a-de3f-4c52-8886-3793ec329c60/space/98fdbae0-0263-4eb8-b444-a93d825211f9/hyperledger/2b43e389-72d3-45fa-9794-9491352d629c/blockchain","_blank");break;
				case "ICM" : window.open("https://ldciq4x.wdf.sap.corp:44312/sap(bD1lbiZjPTUwNCZkPW1pbg==)/bc/bsp/sap/crm_ui_start/default.htm");break;
				case "Analytics" : window.open("https://demo-standard.eu1.sapbusinessobjects.cloud/sap/fpa/ui/tenants/demostandard/app.html#;view_id=boardroom-shell;agendaId=57C0A025539896C4E10000000A6C9A76") ;
				case "Predictive Analysis" : oRoute.navTo("predictiveAnalysis");break;
			}

		}

	});

});