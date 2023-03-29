sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History",
	"jquery.sap.global",
	"sap/m/MessageToast",
	"sap/ui/model/json/JSONModel",
	"sap/viz/ui5/api/env/Format",
	"sap/viz/ui5/controls/common/feeds/FeedItem",
	"sap/viz/ui5/data/DimensionDefinition"
], function(Controller, History, MessageToast, JSONModel, Format, FeedItem, DimensionDefinition) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.IncidentAnalysis", {

		onInit: function() {
			var self = this;
			var incidentArea = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/IncidentAnalysis.json"));
			var oVizFrameColumn = this.getView().byId("idVizFrameColumn");
			var columnModel = new sap.ui.model.json.JSONModel();
			//this.countryDimension = new DimensionDefinition({name : "Area", value : "{Area}"});
/*            this.feedColor = new FeedItem({
                    'uid': "color",
                    'type': "Dimension",
                    'values': ["Area"]
                });*/
			var columnData = {
				'IncidentBarAnalysis': [{
					"Incident": "Theft",
					"Value": "158"
				}, {
					"Incident": "Drugs & Vice",
					"Value": "231"
				}, {
					"Incident": "Kidnapping",
					"Value": "91"
				}, {
					"Incident": "Accident",
					"Value": "109"
				}, {
					"Incident": "Panic",
					"Value": "315"
				}, {
					"Incident": "Murder",
					"Value": "39"
				}, {
					"Incident": "Cyber Crime",
					"Value": "75"
				}]
			};
			columnModel.setData(columnData);
			var columnDataset = new sap.viz.ui5.data.FlattenedDataset({
				dimensions: [{
					name: 'Incident',
					value: "{Incident}"
				}],

				measures: [{
					name: 'Cases',
					value: '{Value}'
				}],

				data: {
					path: "/IncidentBarAnalysis"
				}
			});
			oVizFrameColumn.setDataset(columnDataset);
			oVizFrameColumn.setModel(columnModel);
			oVizFrameColumn.setVizType('column');

			oVizFrameColumn.setVizProperties({
				title: {
					text: "Cases per Incident Type"
				},
				plotArea: {
					colorPalette: d3.scale.category20().range()
				}
			});

			var feedValueAxis = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "valueAxis",
					'type': "Measure",
					'values': ["Cases"]
				}),
				feedCategoryAxis = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "categoryAxis",
					'type': "Dimension",
					'values': ["Incident"]
				});
			oVizFrameColumn.addFeed(feedValueAxis);
			oVizFrameColumn.addFeed(feedCategoryAxis);

			var oVizFrameLine = this.getView().byId("idVizFrameLine");

			var lineData = {
				"Theft": [{
					"Area": "Janakpuri East",
					"Resolved By": "Prabal Kumar",
					"Resolution(days)": 2,
					"Severity": 1,
					"Consumption": 76855.15368,
					"Country": "China"
				}, {
					"Area": "Mundaka",
					"Resolved By": "Sonu Yadav",
					"Resolution(days)": 6,
					"Severity": 2,
					"Consumption": 310292.22,
					"Country": "China"
				}, {
					"Area": "Karol Bagh",
					"Resolved By": "Manish Singh",
					"Resolution(days)": 5,
					"Severity": 2,
					"Consumption": 143432.18,
					"Country": "China"
				}, {
					"Area": "Dwarka",
					"Resolved By": "Vijaya Rajput",
					"Resolution(days)": 10,
					"Severity": 3,
					"Consumption": 487910.26,
					"Country": "China"
				}, {
					"Area": "Janakpuri East",
					"Resolved By": "Prabal Kumar",
					"Resolution(days)": 12,
					"Severity": 3,
					"Consumption": 267185.27,
					"Country": "France"
				}, {
					"Area": "Mundaka",
					"Resolved By": "vijaya Rajput",
					"Resolution(days)": 8,
					"Severity": 3,
					"Consumption": 304964.8856125,
					"Country": "France"
				}, {
					"Area": "Dwarka",
					"Resolved By": "Prabal Kumar",
					"Resolution(days)": 14,
					"Severity": 3,
					"Consumption": 291191.83,
					"Country": "France"
				}, {
					"Area": "Janakpuri East",
					"Resolved By": "Prabal Kumar",
					"Resolution(days)": 1,
					"Severity": 1,
					"Consumption": 98268.9597904,
					"Country": "France"
				}, {
					"Area": "Mundaka",
					"Resolved By": "Vijaya Rajut",
					"Resolution(days)": 7,
					"Severity": 2,
					"Consumption": 176502.5521223,
					"Country": "France"
				}, {
					"Area": "Dwarka",
					"Resolved By": "Manish Singh",
					"Resolution(days)": 3,
					"Severity": 1,
					"Consumption": 538515.47632832,
					"Country": "China"
				}],
				"vTable": [{
					"Key": "1",
					"Severity": "Low"

				}, {
					"Key": "2",
					"Severity": "Medium"

				}, {
					"Key": "3",
					"Severity": "High"

				}],
				"IncidentLineAnalysis": [{
					"Dummy": [{
						"Year": "2011",
						"Cases": "158"
					}, {
						"Year": "2012",
						"Cases": "231"
					}, {
						"Year": "2013",
						"Cases": "275"
					}, {
						"Year": "2014",
						"Cases": "109"
					}, {
						"Year": "2015",
						"Cases": "415"
					}, {
						"Year": "2017",
						"Cases": "139"
					}],
					"Kidnapping": [{
						"Year": "2011",
						"Cases": "112"
					}, {
						"Year": "2012",
						"Cases": "131"
					}, {
						"Year": "2013",
						"Cases": "75"
					}, {
						"Year": "2014",
						"Cases": "109"
					}, {
						"Year": "2015",
						"Cases": "315"
					}, {
						"Year": "2016",
						"Cases": "39"
					}, {
						"Year": "2017",
						"Cases": "91"
					}],
					"Theft": [{
						"Year": "2011",
						"Cases": "15"
					}, {
						"Year": "2012",
						"Cases": "31"
					}, {
						"Year": "2013",
						"Cases": "75"
					}, {
						"Year": "2014",
						"Cases": "6"
					}, {
						"Year": "2015",
						"Cases": "15"
					}, {
						"Year": "2016",
						"Cases": "48"
					}, {
						"Year": "2017",
						"Cases": "39"
					}],
					"Accident": [{
						"Year": "2011",
						"Cases": "102"
					}, {
						"Year": "2012",
						"Cases": "131"
					}, {
						"Year": "2013",
						"Cases": "75"
					}, {
						"Year": "2014",
						"Cases": "91"
					}, {
						"Year": "2015",
						"Cases": "35"
					}, {
						"Year": "2016",
						"Cases": "95"
					}, {
						"Year": "2017",
						"Cases": "60"
					}]

				}]
			};
			var label = "";
			var lineModel = new sap.ui.model.json.JSONModel(lineData);
			this.getView().setModel(lineModel); //lineModel.setData(lineData);
			//this.getView().byId("idVizFrameBubble").getDataset().setContext("Area");
			//this.flattenedDataset = this.getView().byId("idVizFrameBubble").getDataset();
			//this.flattenedDataset.addDimension(this.countryDimension);
			//this.flattenedDataset = this.getView().byId("idVizFrameBubble").getDataset();
			//this.getView().byId("idVizFrameBubble").setDataset(this.flattenedDataset);
			this.getView().byId("idVizFrameBubble").addFeed(this.feedColor);
			this.getView().byId("idVizFrameBubble").getDataset().setContext("Resolved By");
			this.getView().byId("idVizFrameBubble").setVizProperties({
				title: {
					visible: true,
					text: "Incidents Severity"
				}
			});
			oVizFrameLine.setVizProperties({
				plotArea: {
					marker: {

						visible: true

					},
					lineRenderer: function(oMarker) {
						oMarker.graphic.color = "#5cbae6";
					},
					markerRenderer: function(oMarker) {
						oMarker.graphic.fill = "#5cbae6";
						if (oMarker.ctx.Year == "2017") {
							oMarker.graphic.fill = "#FF0000";
						} else {
							oMarker.graphic.fill = "#008000";
						}
					}
				},
				valueAxis: {
					title: {
						visible: true,
						text: "Cases"
					}
				},
				categoryAxis: {
					title: {
						visible: true,
						text: "Year"
					}
				},
				title: {
					visible: true,
					text: 'Cases Records'
				},
				colorPalette: ['red'],
			});
			var detailModel = this.getView().getModel().getProperty("/IncidentLineAnalysis/0/Dummy");
			var jsonPie = new sap.ui.model.json.JSONModel(detailModel);
			oVizFrameLine.setModel(jsonPie);
			oVizFrameLine.getModel().refresh(true);

			incidentArea.attachRequestCompleted(function() {
				self.getView().setModel(incidentArea, "incidentAreaModel");
				var detailModelArea = self.getView().getModel("incidentAreaModel").getProperty("/Dummy");
				var jsonPieArea = new sap.ui.model.json.JSONModel(detailModelArea);
				self.getView().byId("idVizFrameColumn1").setModel(jsonPieArea);
				self.getView().byId("idVizFrameColumn1").getModel().refresh(true);
				self.getView().byId("idVizFrameColumn1").setVizProperties({
					title: {
						visible: true,
						text: "Incident Area wise analysis"
					}
				});
			});

		},

		onNavBack: function(oEvent) {
			this.getOwnerComponent().getRouter().navTo("incidentsanalysis", {}, true);
		},

		onAfterRendering: function() {
			this.initialized = true;
		},

		columnfunc: function(event) {
			var self = this;
			var clickedData = event.getParameters("data").data[0].data.Incident;
			var lineGraph = sap.ui.getCore().byId("idLineGraph");
			var oModel = this.getView().getModel().getProperty("/IncidentLineAnalysis/0/Theft");
			var jsonLine = new sap.ui.model.json.JSONModel(oModel);
			this.getView().byId("idVizFrameLine").setModel(jsonLine);
			this.getView().byId("idVizFrameLine").getModel().refresh(true);
			var prop = this.getView().byId("idVizFrameLine");
			prop.setVizProperties({
				title: {
					visible: true,
					text: clickedData + " for the last 7 years"
				}
			});
			var colGraph = sap.ui.getCore().byId("idVizFrameColumn1");
			var oModel1 = this.getView().getModel("incidentAreaModel").getProperty("/Theft");
			var jsonColGraph = new sap.ui.model.json.JSONModel(oModel1);
			this.getView().byId("idVizFrameColumn1").setModel(jsonColGraph);
			this.getView().byId("idVizFrameColumn1").getModel().refresh(true);
			var prop1 = this.getView().byId("idVizFrameColumn1");
			prop1.setVizProperties({
				title: {
					visible: true,
					text: "Incident Area wise " + "Theft" + " analysis"
				}
			});
		}

	});
});