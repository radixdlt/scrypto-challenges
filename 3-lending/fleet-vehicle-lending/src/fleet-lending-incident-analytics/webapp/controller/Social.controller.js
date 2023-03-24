sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Social", {

		/**
		 * Called when a controller is instantiated and its View controls (if available) are already created.
		 * Can be used to modify the View before it is displayed, to bind event handlers and do other one-time initialization.
		 * @memberOf com.sap.Aviation.view.Social
		 */
		onInit: function() {

			var socialSearchModel = new sap.ui.model.json.JSONModel(
				"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$format=json", false);
			this.getView().setModel(socialSearchModel, "socialSearchModel");
			var socialFullSearchModel = new sap.ui.model.json.JSONModel(
				"../destinations/Planner/Aviation/aviationservices.xsodata/event2?$filter(Narative ne '')&$format=json", false);
			this.getView().setModel(socialFullSearchModel, "socialFullSearchModel");
			var searchTable = this.getView().byId("searchTable");
			searchTable.setVisible(false);
			
			var searchLabel = this.getView().byId("searchLabel");
			searchLabel.setText("Showing all articles ...");

		},

		// onSocialSearch: function() {

		// 	var searchText = this.getView().byId("searchId").getValue();
		// 	var socialSearchModel = this.getView().getModel("socialSearchModel");
		// 	var normalizedForm;
		// 	var tempSearchModel = new sap.ui.model.json.JSONModel();
		// 	var loadUrl = "../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=substringof(TA_TOKEN,'" +
		// 		searchText + "')&$format=json";
		// 	tempSearchModel.loadData(loadUrl);
		// 	tempSearchModel.attachRequestCompleted(function() {
		// 		var items = tempSearchModel.oData.d.results;
		// 		var countOfItems = 0;
		// 		for (countOfItems = 0; countOfItems < items.length; countOfItems++) {
		// 			normalizedForm = items[countOfItems].TA_NORMALIZED;
		// 			if (normalizedForm !== undefined) {
		// 				break;
		// 			}
		// 		}
		// 		if (normalizedForm !== undefined || normalizedForm !== null) {
		// 			loadUrl =
		// 				"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=substringof(TA_NORMALIZED,'" +
		// 				normalizedForm + "')&$format=json";
		// 			socialSearchModel.loadData(loadUrl);
		// 		} else {
		// 			countOfItems = 0;
		// 			var tokenType;
		// 			for (countOfItems = 0; countOfItems < items.length; countOfItems++) {
		// 				tokenType = items[countOfItems].TA_TYPE;
		// 				if (tokenType !== undefined) {
		// 					break;
		// 				}
		// 			}
		// 			if (tokenType === "AVIATION_FATALITIES_COUNT") {
		// 				loadUrl =
		// 					"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=substringof(TA_TYPE,'" +
		// 					tokenType + "')&$format=json";
		// 				socialSearchModel.loadData(loadUrl);
		// 			}
		// 		}

		// 	});

		// },

		onSearch:function() {

			var searchTable = this.getView().byId("searchTable");
			var searchFullTable = this.getView().byId("searchFullTable");
			var searchLabel = this.getView().byId("searchLabel");
			
			var searchText = this.getView().byId("searchId").getValue();
			if (searchText === "") {

				searchTable.setVisible(false);
				searchFullTable.setVisible(true);
				searchLabel.setText("Showing all articles ...");

			} else {

				searchTable.setVisible(true);
				searchFullTable.setVisible(false);
				searchLabel.setText("Showing articles for : '"+searchText +"'");

				var socialSearchModel = this.getView().getModel("socialSearchModel");
				searchText = searchText.split(" ");
				if (searchText.length === 1) {
					var tempSearchModel = new sap.ui.model.json.JSONModel();
					var loadUrl =
						"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=(TA_TOKEN eq '" +
						searchText + "')&$format=json";
					tempSearchModel.loadData(loadUrl);
					tempSearchModel.attachRequestCompleted(function() {
						//var items = tempSearchModel.oData.d.results;
						var normalizedForm = tempSearchModel.getProperty("/d/results/0/TA_NORMALIZED");
						loadUrl =
							"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=substringof(TA_NORMALIZED,'" +
							normalizedForm + "')&$format=json";
						socialSearchModel.loadData(loadUrl);
					});

				} else {
					var numKilled = 0;
					var moreThan = 0;
					var lessThan = 0;
					var count = 0;
					for (var i = 0; i < searchText.length; i++) {
						if (searchText[i] === "killed" || searchText[i] === "died" || searchText[i] === "Killed" || searchText[i] === "dead" ||
							searchText[i] === "Dead") {
							numKilled = 1;
						} else if (searchText[i] === "more" || searchText[i] === "More" || searchText[i] === "above" || searchText[i] === "Above" ||
							searchText[
								i] === "greater" || searchText[i] === "Greater") {
							moreThan = 1;
						} else if (searchText[i] === "less" || searchText[i] === "Less" || searchText[i] === "Fewer" || searchText[i] === "fewer" ||
							searchText[
								i] === "below" || searchText[i] === "Below") {
							lessThan = 1;
						} else {

							if (searchText[i] !== "than" || searchText[i] === "Than") {
								count = searchText[i];
							}
						}
					}
					if (numKilled === 1 && moreThan === 0 && lessThan === 0) {
						loadUrl =
							"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=(TA_TOKEN eq '" +
							count + "' and TA_TYPE eq 'AVIATION_FatalCount')&$format=json";
						socialSearchModel.loadData(loadUrl);

					}
					// if (numKilled === 1 && moreThan === 1) {
					// 	var tempSearchModel = new sap.ui.model.json.JSONModel();
					// 	var loadUrl =
					// 		"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=(TA_TOKEN eq '" +
					// 		count + "' and TA_TYPE eq 'AVIATION_FatalCount')&$format=json";
					// 	socialSearchModel.loadData(loadUrl);

					// }
					// if (numKilled === 1 && lessThan === 1) {
					// 	var tempSearchModel = new sap.ui.model.json.JSONModel();
					// 	var loadUrl =
					// 		"../destinations/Planner/Aviation/aviationservices.xsodata/aviationTA?$expand=eventv2&$filter=(TA_TOKEN eq '" +
					// 		count + "' and TA_TYPE eq 'AVIATION_FatalCount')&$format=json";
					// 	socialSearchModel.loadData(loadUrl);

					// }
				}
			}
		}

	});

});