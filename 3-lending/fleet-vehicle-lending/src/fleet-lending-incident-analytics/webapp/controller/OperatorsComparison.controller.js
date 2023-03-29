sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.OperatorsComparison", {

		onInit: function() {
			this.router = this.getOwnerComponent().getRouter();
			this.router.getRoute("operatorsComparison").attachPatternMatched(this._onRouteMatched, this);

		},

		_onRouteMatched: function(oEvent) {
			var oId;
			var oThis = this;
			if (oEvent.getParameter("arguments")) {
				oId = oEvent.getParameter("arguments").operatorCode;
			}
			oId = oId.split(",");
			oThis.getView().byId("columnHeader1").setText(oId[0]);
			oThis.getView().byId("columnHeader2").setText(oId[1]);
			var testmodel = this.getOwnerComponent().getModel("operatorsComparison");
			//testmodel.attachRequestCompleted(function() {
				var operatorComparisonModel = new sap.ui.model.json.JSONModel(
					"../destinations/Planner/Aviation/aviationservices.xsodata/operators?$filter=(OperatorCode eq '" + oId[0] + "')&$top=1", false
				);
				operatorComparisonModel.attachRequestCompleted(function() {
					/*alert(operatorComparisonModel.oData.d.results[0].JOSOperatorAreaCode);
					alert(testmodel.oData);*/
					testmodel.oData.operatorCollection[0].value1 = operatorComparisonModel.oData.d.results[0].OperatorCode; 
					testmodel.oData.operatorCollection[1].value1 = operatorComparisonModel.oData.d.results[0].OperatorName;
					testmodel.oData.operatorCollection[2].value1 = operatorComparisonModel.oData.d.results[0].OperatorAreaCode;
					testmodel.oData.operatorCollection[3].value1 = operatorComparisonModel.oData.d.results[0].OperatorAreaDesc;
					testmodel.oData.operatorCollection[4].value1 = operatorComparisonModel.oData.d.results[0].location;
					testmodel.oData.operatorCollection[5].value1 = operatorComparisonModel.oData.d.results[0].country;
					testmodel.oData.operatorCollection[6].value1 = operatorComparisonModel.oData.d.results[0].OperatorCategoryCode;
					testmodel.oData.operatorCollection[7].value1 = operatorComparisonModel.oData.d.results[0].OperatorcategoryDesc;
					testmodel.oData.operatorCollection[8].value1 = operatorComparisonModel.oData.d.results[0].OperatorCountryCode;
					testmodel.oData.operatorCollection[9].value1 = operatorComparisonModel.oData.d.results[0].OperatorCountry;
					
				
				var operatorComparisonModel1 = new sap.ui.model.json.JSONModel(
					"../destinations/Planner/Aviation/aviationservices.xsodata/operators?$filter=(OperatorCode eq '" + oId[1] + "')&$top=1", false
				);
				operatorComparisonModel1.attachRequestCompleted(function() {
					/*alert(operatorComparisonModel.oData.d.results[0].JOSOperatorAreaCode);
					alert(testmodel.oData);*/
					testmodel.oData.operatorCollection[0].value2 = operatorComparisonModel1.oData.d.results[0].OperatorCode; 
					testmodel.oData.operatorCollection[1].value2 = operatorComparisonModel1.oData.d.results[0].OperatorName;
					testmodel.oData.operatorCollection[2].value2 = operatorComparisonModel1.oData.d.results[0].OperatorAreaCode;
					testmodel.oData.operatorCollection[3].value2 = operatorComparisonModel1.oData.d.results[0].OperatorAreaDesc;
					testmodel.oData.operatorCollection[4].value2 = operatorComparisonModel1.oData.d.results[0].location;
					testmodel.oData.operatorCollection[5].value2 = operatorComparisonModel1.oData.d.results[0].country;
					testmodel.oData.operatorCollection[6].value2 = operatorComparisonModel1.oData.d.results[0].OperatorCategoryCode;
					testmodel.oData.operatorCollection[7].value2 = operatorComparisonModel1.oData.d.results[0].OperatorcategoryDesc;
					testmodel.oData.operatorCollection[8].value2 = operatorComparisonModel1.oData.d.results[0].OperatorCountryCode;
					testmodel.oData.operatorCollection[9].value2 = operatorComparisonModel1.oData.d.results[0].OperatorCountry;
					oThis.getView().byId("idProductsTable").setModel(testmodel,"testmodel");
					oThis.getView().byId("idProductsTable").getModel(testmodel).refresh(true);
				});
				});
			//});
		}

	});

});