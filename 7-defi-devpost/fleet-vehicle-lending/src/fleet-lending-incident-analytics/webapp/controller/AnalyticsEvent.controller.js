sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/model/json/JSONModel"
], function(Controller, JSONModel) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.AnalyticsEvent", {

		/**
		 * Called when a controller is instantiated and its View controls (if available) are already created.
		 * Can be used to modify the View before it is displayed, to bind event handlers and do other one-time initialization.
		 * @memberOf com.sap.Aviation.view.AnalyticsEvent
		 */
		onInit: function() {
			var oModel = new JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/officerHealth.json"));
			var self = this;
			oModel.attachRequestCompleted(function() {
				self.getView().setModel(oModel);
				var oVizFrame = self.getView().byId("idVizFrameBar");
				var oVizFrameC = self.getView().byId("idVizFrameBarC");
				var oVizFrameHR = self.getView().byId("idVizFrameBarHR");
				var oVizFrameCH = self.getView().byId("idVizFrameBarCH");
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
					valueAxis: {
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
				oVizFrameHR.setVizProperties({
					plotArea: {
						showGap: true
					},
					categoryAxis: {
						title: {
							visible: true,
							text: label
						}
					},
					valueAxis: {
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
				oVizFrameCH.setVizProperties({
					plotArea: {
						showGap: true
					},
					categoryAxis: {
						title: {
							visible: true,
							text: label
						}
					},
					valueAxis: {
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
				oVizFrameC.setVizProperties({
					legend: {
						title: {
							visible: false
						}
					},
					title: {
						visible: false
					}
				});

				//setting monthly report detail chart as Jan by Default
				var monthDetailModel = oModel.getProperty("/Injury/0/Jan");
				var jsonPie = new JSONModel(monthDetailModel);
				oVizFrameC.setModel(jsonPie);
				oVizFrameC.getModel().refresh(true);
			}); // end of attachRequestcompleted

		},
			onNavBack: function(){
				this.getOwnerComponent().getRouter().navTo("events", {}, true);
		},
		onComboSelect: function(oEvent) {
			var selection = oEvent.getSource().getSelectedItem().getText();
			if (selection.includes("Hear Rate Details")) {
				this.getView().byId("chartFixFlexHR").setVisible(true);
				this.getView().byId("idBPTable").setVisible(true);
				this.getView().byId("chartFixFlexCH").setVisible(false);
				this.getView().byId("idCholesTable").setVisible(false);
			}
			if (selection.includes("Cholestrol Level")) {
				this.getView().byId("chartFixFlexCH").setVisible(true);
				this.getView().byId("idCholesTable").setVisible(true);
				this.getView().byId("chartFixFlexHR").setVisible(false);
				this.getView().byId("idBPTable").setVisible(false);
			}
		},

		/**
		 * Similar to onAfterRendering, but this hook is invoked before the controller's View is re-rendered
		 * (NOT before the first rendering! onInit() is used for that one!).
		 * @memberOf com.sap.Aviation.view.AnalyticsEvent
		 */
		//	onBeforeRendering: function() {
		//
		//	},

		/**
		 * Called when the View has been rendered (so its HTML is part of the document). Post-rendering manipulations of the HTML could be done here.
		 * This hook is the same one that SAPUI5 controls get after being rendered.
		 * @memberOf com.sap.Aviation.view.AnalyticsEvent
		 */
		onAfterRendering: function() {
			this.getView().byId("cbhealth").setSelectedKey("1");
		},

		/**
		 * Called when the Controller is destroyed. Use this one to free resources and finalize activities.
		 * @memberOf com.sap.Aviation.view.AnalyticsEvent
		 */
		onChartClickHandler: function(oEvent) {
			var clickedData = oEvent.getParameter("data")[0].data.Month;
			var oModel = this.getView().getModel().getProperty("/Injury/0/" + clickedData);
			var jsonPie = new JSONModel(oModel);
			this.getView().byId("idVizFrameBarC").setModel(jsonPie);
			this.getView().byId("idVizFrameBarC").getModel().refresh(true);
			this.getView().byId("chartContainerCholes").setTitle("Report Details : " + clickedData);
		}

	});

});