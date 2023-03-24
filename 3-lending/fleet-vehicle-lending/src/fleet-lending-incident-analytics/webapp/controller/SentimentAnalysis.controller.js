jQuery.sap.require("com.sap.Aviation.graph.groupNetwork");
sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.SentimentAnalysis", {

		onInit: function() {
			var oThis = this;
			var oModel = new sap.ui.model.json.JSONModel("./model/infinityNetwork.json", false);
			oModel.attachRequestCompleted(function(data) {
				var graph = new com.sap.Aviation.graph.groupNetwork({
					graphData: data.oSource.getData().d.results,
					press: function(oEvent) {
						var graph = oEvent.getSource();
						var node = oEvent.getParameters();
						console.log("press event received:" + node.NAME);
					},
					hover: function(oEvent) {
						var graph = oEvent.getSource();
						var node = oEvent.getParameters();
						console.log("hover event received: " + node.NAME);
					},
					doublePress: function(oEvent) {
						var graph = oEvent.getSource();
						var node = oEvent.getParameters();
						var popoverListData = {
							results: []
						};
						var entry = {};
						if (node.LEVEL === 0) {
							for (var i = 0; i < node.children.length; i++) {
								entry = {};
								entry["title"] =node.children[i].NAME;
								if (node.children[i].children) {
									entry["value"] = node.children[i].children.length;
								}
								popoverListData.results.push(entry);
							}
						}
						if (node.LEVEL === 1) {
							for (var i = 0; i < node.children.length; i++) {
								entry = {};
								entry["title"] =".."+ node.children[i].SubCategory + "..";
								popoverListData.results.push(entry);
							}
						}
						// if (node._children != null) {
						// 	for (var i = 0; i < node._children.length; i++) {
						// 		entry = {};
						// 		entry["title"] = ".."+ node._children[i].SubCategory + "..";
						// 		if (node._children[i].children) {
						// 			entry["value"] = node._children[i].children.length;
						// 		}
						// 		popoverListData.results.push(entry);
						// 	}
						// }
						if (node.children === null || node.children === undefined || node.children.length === 0) {
							entry = {};
							entry["title"] = node.COMMENT;
							popoverListData.results.push(entry);
						}
						var popOverListModel = new sap.ui.model.json.JSONModel(
							popoverListData);
						var list = new sap.m.List({
							showSeparators: "None",
							items: {
								path: "/results",
								template: new sap.m.DisplayListItem({
									label: "{title}",
									value: "{value}"
								})
							}
						});
						list.setModel(popOverListModel);
						var popover;
						if (node.LEVEL === 0 || node.LEVEL === 1) {
								popover = new sap.m.Popover({
								title: node.NAME,
								placement: sap.m.PlacementType.Auto,
								content: list,
								footer: new sap.m.Bar({
									contentMiddle: [new sap.m.Button({
											text: "ViewDetails",
											icon: "sap-icon://detail-view",
											press: function(){
												oThis.getOwnerComponent().getRouter().navTo("sentimentDetails", {});
											}
										})
									]
								})
							}).addStyleClass("sapUiPopupWithPadding");
						} else {
							popover = new sap.m.Popover({
								title: node.NAME,
								placement: sap.m.PlacementType.Auto,
								content: list
							}).addStyleClass("sapUiPopupWithPadding");
						}
						popover.openBy(event.srcElement);

					}
				});

				graph.addEventDelegate({
					onAfterRendering: function() {
					}
				});
			
				oThis.getView().byId("oPage").addContent(graph);

			});

		},
		onNavBack: function(){
				this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true /*no history*/ );
		}
	});
});