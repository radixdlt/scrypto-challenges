sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	var self;
	var areasModel;
	var map1;
	return Controller.extend("com.sap.Aviation.controller.Analytics", {
		onInit: function() {
			self = this;
			areasModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/areas.json"));
			this.getView().setModel(areasModel, "areas");
			var oVizFrameColumn = this.getView().byId("idVizFrameColumn");
			var columnModel = new sap.ui.model.json.JSONModel();
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

			//var oVizFrameLine = this.getView().byId("idVizFrameLine");
			this.onSliderChange(this);
			this.getView().byId("idCrimeCell").onAfterRendering = function() {
				if (sap.ui.layout.BlockLayoutCell.prototype.onAfterRendering) {
					sap.ui.layout.BlockLayoutCell.prototype.onAfterRendering.apply(self, arguments);
				}
				//alert("hi");
				if (!this.initialized) {
					this.initialized = true;
					this.geocoder = new google.maps.Geocoder();
					var mapOptions = {
						center: new google.maps.LatLng(28.6387293, 77.0853497),
						zoom: 10,
						mapTypeId: google.maps.MapTypeId.ROADMAP
					};

					var sid = self.getView().byId("map_analytic").getId();
					// //var map_analytic = document.getElementById(sid);

					var map = new google.maps.Map(sap.ui.getCore().byId(sid).getDomRef(),
							mapOptions),
					//	directionsService = new google.maps.DirectionsService,
						directionsDisplay = new google.maps.DirectionsRenderer({
							map: map
						});

					//self.addMarkerToMap(map);
					self.map1 = map;
				}
				if (!this.legends) {
					var sid = self.getView().byId("legend").getId();
					var legend = document.getElementById(sid);
					var icons = self.createLegends()[0];
					for (var key in icons) {
						var type = icons[key];
						var name = type.name;
						var icon = type.icon;
						var div = document.createElement('div');
						div.innerHTML = '<img src="' + icon + '"> ' + name;
						legend.appendChild(div);
					}
					this.legends = true;
				}
				map.controls[google.maps.ControlPosition.LEFT_TOP].push(legend);
			};

			areasModel.attachRequestCompleted(function() {
				self.addMarkerToMap(self.map1);
			});

		},

		addMarkerToMap: function(map) {
			var image = "";
			var incidents = areasModel.getData().survArea;
			for (var num_3 in incidents) {
				var selected_incident = incidents[num_3];
				var myLatLng = {
					lat: selected_incident.lat,
					lng: selected_incident.long
				};
				if (selected_incident.name == "drone") {
					image = "image/drone.png";
				} else if (selected_incident.name == "vehicle") {
					image = "image/vehicle1.png";
				} else if (selected_incident.name == "autonomous") {
					image = "image/autonomous.png";
				} else if (selected_incident.name == "police") {
					image = "image/policeman.png";
				}
				var marker = new google.maps.Marker({
					position: myLatLng,
					map: map,
					title: selected_incident.name,
					icon: image
				});
			}
		},

		createLegends: function() {
			var legend1 = new Object();
			legend1.name = "Drone";
			legend1.icon = "image/drone.png";

			var legend2 = new Object();
			legend2.name = "Vehicle";
			legend2.icon = "image/vehicle1.png";

			var legend3 = new Object();
			legend3.name = "Autonomous";
			legend3.icon = "image/autonomous.png";

			var legend4 = new Object();
			legend4.name = "Police Patrol";
			legend4.icon = "image/policeman.png";

			return new Array([legend1, legend2, legend3, legend4]);
		},

		onBOPress: function() {
			window.open(
				"https://demo-standard.eu1.sapbusinessobjects.cloud/sap/fpa/ui/tenants/demostandard/app.html#;view_id=boardroom-shell;agendaId=57C0A025539896C4E10000000A6C9A76"
			);
		},
		myOnClickHandler: function(oEvent) {
			var clickedData = oEvent.getParameter("data")[0].data;
			console.log("clickedData");
		},
		onNavBack: function() {
			this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true);
		},

		onAfterRendering: function() {
			if (!self.initialized) {
				self.initialized = true;
				self.geocoder = new google.maps.Geocoder();
				var mapOptions = {
					center: new google.maps.LatLng(28.6387293, 77.0853497),
					zoom: 12,
					mapTypeId: google.maps.MapTypeId.ROADMAP
				};
				// var sid = this.getView().byId("map_analytic").getId();
				// //var map_analytic = document.getElementById(sid);

				// var map = new google.maps.Map(sap.ui.getCore().byId(sid).getDomRef(),
				// 		mapOptions),
				// 	directionsService = new google.maps.DirectionsService,
				// 	directionsDisplay = new google.maps.DirectionsRenderer({
				// 		map: map
				// 	});
			}
		},

		onSliderChange: function(oEvent) {
			var value1 = this.getView().byId("idS1").getValue(); //oEvent.getSource();
			var value2 = this.getView().byId("idS2").getValue();
			var value3 = this.getView().byId("idS3").getValue();
			var value = parseInt(value1) + parseInt(value2) - parseInt(value3);
			value = Math.sqrt(value);
			value = (value / 100) * 100;
			value = Math.round(value);
			value = value + " % ";
			this.getView().byId("idCrimeRate").setText(value);
		}
	});

});