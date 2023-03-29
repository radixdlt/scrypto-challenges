sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/model/json/JSONModel"
], function(Controller, JSONModel) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.sentiments", {

		onInit: function() {
			//begin of service call 
			// var oModel = new sap.ui.model.json.JSONModel();
			// var loadUrl = "../destinations/Feeds/COIL/data.xsodata/feeds";

			// var self = this;
			// $.ajax({
			// 	url: loadUrl,
			// 	type: "GET",
			// 	dataType: "json",
			// 	success: function(data){
			// 	oModel.setData(data.d.results);
			// 	self.setModel(oModel);
			// 	},
			// 	error: function() {

			// 	},
			// 	xhrFields: {
			// 		withCredentials: true
			// 	}
			// });
			//end of service call
			var oModel = new sap.ui.model.json.JSONModel("./model/infinityNetwork.json", false);
			var graphModel = new sap.ui.model.json.JSONModel();
			var globalModel = new sap.ui.model.json.JSONModel();
			var self = this;
			oModel.attachRequestCompleted(function() {
				var sentiments = oModel.getData();
				var graphData = self.customizeGraphData(sentiments.d.results);
				graphModel.setData(graphData);
				globalModel.setData(sentiments.d);
				self.getView().setModel(oModel, "infinityNetwork");
				var oVizFrame = self.getView().byId("idVizFramePie");
				var label = "";
				oVizFrame.setVizProperties({
					plotArea: {
						showGap: true
					},
					categoryAxis: {
						title: {
							visible: true,
							text: label
						}
					},
					title: {
						visible: true,
						text: "" + label
					}
				});
				self.getView().setModel(graphModel, "graph");
				self.globalModel = globalModel;
				self.graphModel = graphModel;
			});

		},
		// removeDuplicates: function(originalArray, prop) {
		// 	var newArray = [];
		// 	var lookupObject = {};

		// 	for (var i in originalArray) {
		// 		lookupObject[originalArray[i][prop]] = originalArray[i];
		// 	}

		// 	for (i in lookupObject) {
		// 		newArray.push(lookupObject[i]);
		// 	}
		// 	return newArray;
		// },
		onNavBack: function() {
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("sentiments");
		},
		customizeGraphData: function(data) {
			//customize graph data
			var graphData = [];
			var i = 0;
			var structure = {};
			var positiveCount = 0,
				negativeCount = 0,
				neutralCount = 0;

			for (i = 0; i < data.length; i++) {
				if (data[i].MainCategory === "Positive") {
					positiveCount++;
				} else if (data[i].MainCategory === "Negetive") {
					negativeCount++;
				} else if (data[i].MainCategory === "Neutral") {
					neutralCount++;
				}
			}

			structure.type = "POSITIVE";
			structure.count = positiveCount.toString();
			graphData.push(structure);
			structure = {};
			structure.type = "NEGATIVE";
			structure.count = negativeCount.toString();
			graphData.push(structure);
			structure = {};
			structure.type = "NEUTRAL";
			structure.count = neutralCount.toString();
			graphData.push(structure);
			return graphData;

		},

		onTableFilter: function(oEvent) {
			var person = oEvent.getSource()._getSelectedItemText();
			var itemTable = this.getView().byId("idSocialMediaTable");
			var oBinding = itemTable.getBinding("items");
			var oFilter;
			switch (person) {
				case "Sheikh Hamadan":
				case "Abdullah Khalifa Al Merri":
					oFilter = new sap.ui.model.Filter("PERSONNAME", sap.ui.model.FilterOperator.EQ, person);
					break;
				case "#dubaipolice":
				case "#security":
				case "#defense":
				case "#safety":
					oFilter = new sap.ui.model.Filter("HASHTAG", sap.ui.model.FilterOperator.EQ, person);
					break;
			}
			var oFilters = [];
			oFilters.push(oFilter);
			oBinding.filter(oFilters);

			var graphData = [];
			var data = oBinding.aLastContextData;
			for (var i = 0; i < data.length; i++) {
				var item = JSON.parse(data[i]);
				graphData.push(item);
			}
			data = this.customizeGraphData(graphData);
			this.graphModel.setData(data);
		},

		onFilterChange: function(oEvent) {
			var type = oEvent.getSource().getSelectedItem().getText();
			var list = this.getView().byId("idSocialMediaTable");
			var aFilter = [],
				filterArr = [];
			if (type !== "All") {
				filterArr.push(new sap.ui.model.Filter("POST_MESSAGE",
					sap.ui.model.FilterOperator.Contains, type));
				aFilter.push(new sap.ui.model.Filter(filterArr, false));
			} else {
				aFilter = null;
			}
			// update list binding	
			var binding = list.getBinding("items");
			binding.filter(aFilter, "Application");

			//call the graph funciton for filtered data 
			var globalData = this.globalModel.getData();
			var parameter = "POST_MESSAGE";
			var filtered_Data;
			if (type !== "All") {
				filtered_Data = this.getFilterData(globalData, parameter, type);
			} else {
				filtered_Data = globalData;
			}
			var filtered_graphData = this.customizeGraphData(filtered_Data);
			this.graphModel.setData(filtered_graphData);
		},
		onTableUpdateFinished: function(oEvent) {

		},
		getFilterData: function(globalData, aParameter, sQuery) {
			if ((!globalData || !Array.isArray(globalData) || !aParameter)) {
				return;
			}

			var expression = "";

			var paramString = "(i." + aParameter + "&& " + "i." + aParameter + ".toLowerCase().indexOf('" + sQuery.toLowerCase() + "') !== -1)";

			expression = paramString;

			var i = 0;
			var filteredData = globalData.filter(function(i) {
				return (eval(expression));
			});
			return filteredData;
		}

	});

});