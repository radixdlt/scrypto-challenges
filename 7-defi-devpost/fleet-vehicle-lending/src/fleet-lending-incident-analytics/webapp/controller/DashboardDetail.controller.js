sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.DashboardDetail", {
		mapOVPTilePress: function(oEvent) {
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("overview", {
				industryOverview: "Aviation"
			});
		},
		dashboardTilePress: function(oEvent) {
			var navRoute = oEvent.getSource().getProperty("title");
			var oRoute = this.getOwnerComponent().getRouter();
			switch (navRoute) {
				case "Social Media Sentiments":
					oRoute.navTo("sentiments");
					break;
					//location.href = "https://twitter.com/DubaiPoliceHQ"; break;
				case "ICM Tickets":
					oRoute.navTo("claims");
					break;
				case "Health Summary":
					oRoute.navTo("events");
					break;
				case "Safety Ratings":
					oRoute.navTo("risks");
					break;
				case "Analytics Dashboard":
					oRoute.navTo("analytics");
					break;
				case "Aircrafts":
					oRoute.navTo("aircrafts");
					break;
				case "Operators":
					oRoute.navTo("operators");
					break;
				case "Inbox":
					break;
				case "Flights":
					oRoute.navTo("flights");
					break;
				case "Create Incident":
					oRoute.navTo("create");
					break;
				case "Navigate Incidents":
					oRoute.navTo("navigate");
					break;
				case "SOS": //oRoute.navTo("sos");
					sap.m.MessageToast.show('Emergeny incident created and current location shared successfully');
					break;
				case "Incidents":
					oRoute.navTo("incident");
					break;
				case "Incidents Analysis":
					oRoute.navTo("incidentsanalysis");
					break;
				case "Analytics": 
					oRoute.navTo("analytics");
					break;
			/*	case "Incident Analysis":
					oRoute.navTo("incidentanalysis");
					break;*/
				case "My Incidents History":
					this.getOwnerComponent().getRouter().navTo("incidentStatus", {}, true);
					break;

			}

		},
		onInit: function() {
			this.aircraftTypesModel = new sap.ui.model.json.JSONModel("./model/aircraftTypes.json", false);
			this.flightModel = new sap.ui.model.json.JSONModel(
				"../destinations/Planner/Aviation/aviationservices.xsodata/flights?$filter=(TIMESTAMP eq '1488454699' and TO ne '' and FROM ne '')&$format=json",
				false);
		},

		onAfterRendering: function() {
			var x = this.getView().byId("videoPanel");
			var html1 = new sap.ui.core.HTML("", {
				content: "<video width='50%' height='50%' autoplay>" +
					"<source src='http://www.w3schools.com/html/movie1.mp4' type='video/mp4'>" +
					"Your browser does not support the video tag." +
					"</video>"
			});
			x.addContent(html1);

			/*	var mapViewID=this.getView().byId("videoPanel").getDomRef().lastChild.id;
				mapViewID.alert("Hi folks");*/
		}

	});

});