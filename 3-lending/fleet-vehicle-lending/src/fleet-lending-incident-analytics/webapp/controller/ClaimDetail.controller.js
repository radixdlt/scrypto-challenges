sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.ClaimDetail", {

		onInit: function() {
			this.router = this.getOwnerComponent().getRouter();
			this.router.getRoute("claimDetail").attachPatternMatched(this._onRouteMatched, this);

		},

		_onRouteMatched: function(oEvent) {
			var oThis = this;
			if (oEvent.getParameter("arguments")) {
				var cId = oEvent.getParameter("arguments").claimMasterRef;
				var claimDetailModel = new sap.ui.model.json.JSONModel(
					"../destinations/Planner/Aviation/aviationservices.xsodata/claims?$filter=(ClaimMasterReference eq '" + cId + "')", "false");
				claimDetailModel.attachRequestCompleted(function() {
					oThis.getView().setModel("claimDetailModel", claimDetailModel);
					//alert(claimDetailModel.getProperty("/d/results/0/PolicyNumber"));
					oThis.getView().byId("id1").setTitle("Claim Master Reference : "+claimDetailModel.getProperty("/d/results/0/ClaimMasterReference"));
					oThis.getView().byId("claimAttr1").setText("Policy Number : "+claimDetailModel.getProperty("/d/results/0/PolicyNumber"));
					oThis.getView().byId("claimAttr2").setText("Assured Name : "+claimDetailModel.getProperty("/d/results/0/AssuredName"));
					oThis.getView().byId("claimStatus").setText("Status : "+claimDetailModel.getProperty("/d/results/0/ClaimStatus"));
					if(claimDetailModel.getProperty("/d/results/0/ClaimStatus") === "Finalised")
					{
						oThis.getView().byId("claimStatus").setState("Success");
					}
					else
					{
						oThis.getView().byId("claimStatus").setState("Warning");
					}
					oThis.getView().byId("form1").setText(claimDetailModel.getProperty("/d/results/0/BranchName"));
					oThis.getView().byId("form2").setText(claimDetailModel.getProperty("/d/results/0/ProductDepartmentName"));
					oThis.getView().byId("form3").setText(claimDetailModel.getProperty("/d/results/0/AccidentDate"));
					oThis.getView().byId("form4").setText(claimDetailModel.getProperty("/d/results/0/DateOfLossFrom"));
					oThis.getView().byId("form5").setText(claimDetailModel.getProperty("/d/results/0/AdvisedDate"));
					oThis.getView().byId("form6").setText(claimDetailModel.getProperty("/d/results/0/CauseofLossName"));
					oThis.getView().byId("form7").setText(claimDetailModel.getProperty("/d/results/0/CauseofLossCode"));
					oThis.getView().byId("form8").setText(claimDetailModel.getProperty("/d/results/0/GrossLossALAEPaidUSD"));
					oThis.getView().byId("form9").setText(claimDetailModel.getProperty("/d/results/0/GrossLossALAEReserveUSD"));
					oThis.getView().byId("form10").setText(claimDetailModel.getProperty("/d/results/0/GrossLossALAEIncurredUSD"));
					oThis.getView().byId("form11").setText(claimDetailModel.getProperty("/d/results/0/CurrencyCode"));
				});
			}

		}
	});

});