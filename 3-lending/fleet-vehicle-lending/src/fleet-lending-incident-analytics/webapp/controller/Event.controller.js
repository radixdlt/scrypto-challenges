sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Event", {
		onInit: function() {
			this.router = this.getOwnerComponent().getRouter();
			this.router.getRoute("eventDetails").attachPatternMatched(this._onRouteMatched, this);

		},

		_onRouteMatched: function(oEvent) {
			var oThis = this;
			if (oEvent.getParameter("arguments")) {
				var eId = oEvent.getParameter("arguments").eventId;
				var eventDetailModel = new sap.ui.model.json.JSONModel(
					"../destinations/Planner/Aviation/aviationservices.xsodata/events(EventId='"+ eId +"')", "false");
				eventDetailModel.attachRequestCompleted(function() {
					oThis.getView().setModel("eventDetailModel", eventDetailModel);
					//alert(claimDetailModel.getProperty("/d/results/0/PolicyNumber"));
					oThis.getView().byId("id1").setTitle("Event Name : "+eventDetailModel.getProperty("/d/EventName"));
					oThis.getView().byId("eventAttr1").setText("Event Classification : "+eventDetailModel.getProperty("/d/EventClassification"));
					oThis.getView().byId("eventAttr2").setText("Event Sub Classification : "+eventDetailModel.getProperty("/d/EventSubClassification"));
					//oThis.getView().byId("claimStatus").setText("Status : "+eventDetailModel.getProperty("/d/results/0/ClaimStatus"));
					// if(eventDetailModel.getProperty("/d/results/0/ClaimStatus") === "Finalised")
					// {
					// 	oThis.getView().byId("claimStatus").setState("Success");
					// }
					// else
					// {
					// 	oThis.getView().byId("claimStatus").setState("Warning");
					// }
					oThis.getView().byId("form1").setText(eventDetailModel.getProperty("/d/DoL"));
					oThis.getView().byId("form2").setText(eventDetailModel.getProperty("/d/RegNo"));
					oThis.getView().byId("form3").setText(eventDetailModel.getProperty("/d/Location"));
					oThis.getView().byId("form4").setText(eventDetailModel.getProperty("/d/CrewOnBoard"));
					oThis.getView().byId("form5").setText(eventDetailModel.getProperty("/d/PaxOnBoard"));
					oThis.getView().byId("form6").setText(eventDetailModel.getProperty("/d/CrewFatal"));
					oThis.getView().byId("form7").setText(eventDetailModel.getProperty("/d/PaxFatal"));
					oThis.getView().byId("form8").setText(eventDetailModel.getProperty("/d/TotalHullPaid"));
					oThis.getView().byId("form9").setText(eventDetailModel.getProperty("/d/TotalHullOutstanding"));
					oThis.getView().byId("form10").setText(eventDetailModel.getProperty("/d/TotalLiabPaid"));
					oThis.getView().byId("form11").setText(eventDetailModel.getProperty("/d/TotalLiabOutstanding"));
				});
			}

		}
	});
});