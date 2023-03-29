sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History",
	"jquery.sap.global",
	"sap/m/MessageToast",
	"sap/ui/model/json/JSONModel",
	"com/sap/Aviation/js/markerfunc"
], function(Controller, History, MessageToast, JSONModel, MarkerFunc) {
	"use strict";

	var policeFlag = false;
	var dispatcherFlag = true;
	return Controller.extend("com.sap.Aviation.controller.Navigate", {

		onInit: function() {
			//var oModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incidentCollection.json"));
			var policeModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/police.json"));
			var areasModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/areas.json"));

			var oModel = this.getOwnerComponent().getModel("incidentCollection");
			this.getView().setModel(oModel);
			this.model = oModel;
			this.policeModel = policeModel;
			// var self = this;
			/*oModel.attachRequestCompleted(function() {
				this.getView().setModel(oModel);
				// self.loadMarkersOnMap(oModel);
			});*/

			this.markerList = new Array();
			this.fireList = new Array();
			this.theftList = new Array();
			this.drugsList = new Array();
			this.policemanList = new Array();
			this.assignedPolicemanList = new Array();

			this.getView().setModel(areasModel, "areas");
			this.legends = false;
		},

		createEligiblePolice: function() {
			var policeMen = this.policeModel.getData().PoliceList;
			var eligibleList = new Array();

			for (var num_3 in policeMen) {
				var selectedPolice = policeMen[num_3];
				if ((!selectedPolice.isAssigned) && (selectedPolice.healthStatus == 'fit')) {
					eligibleList.push(selectedPolice);
				}
			}
			return eligibleList;
		},

		addMarkerToMap: function(myLatLng, selectedIncident) {
			var self = this;
			var contentString = '<div id="flightDetailContainer"><div class="flightDetailContainerLeft1">' +
				'<p>' + "Incident No.: " + selectedIncident.eventNum + '</p>' +
				'<p>' + "Category: " + selectedIncident.category + '</p>' +
				'<p>' + "Comments: " + selectedIncident.comments + '</p>' +
				'<p>' + "Status: " + selectedIncident.status + '</p>' +
				'<p>' + "Officers Assigned: " + selectedIncident.officers + '</p>' +
				'</div><div class="flightDetailContainerRight1">' +
				'<select id = "policeSelect">';
			var policeMen = this.createEligiblePolice();
			for (var num_3 in policeMen) {
				var selectedPolice = policeMen[num_3];
				contentString += ('<option value="' + selectedPolice.badgeNum + '">' + selectedPolice.badgeNum + '-' + selectedPolice.name +
					'</option>');
			}
			contentString += '</select>';
			contentString += '<br><br>';
			contentString += this.addAssignButtonForIncident(0);
			contentString += '</div>';

			var infowindow = new google.maps.InfoWindow({
				content: contentString
			});

			var image;

			if ((selectedIncident.status == "New") && (selectedIncident.officers.length > 0)) {
				selectedIncident.status = "Pending";
			}

			if ((selectedIncident.status == "Pending") && (selectedIncident.officers.length == 0)) {
				selectedIncident.status = "New";
			}

			if (selectedIncident.status == "Pending") {
				if (selectedIncident.type == "Fire") {
					image = "image/fire_yellow.png"
				} else if (selectedIncident.type == "Accident") {
					image = "image/accident_yellow.png"
				} else if (selectedIncident.type == "Theft") {
					image = "image/theft_yellow.png";
				} else {
					image = "http://maps.google.com/mapfiles/ms/icons/yellow-dot.png";
				}
			} else if (selectedIncident.status == "New") {
				if (selectedIncident.type == "Fire") {
					image = "image/fire_red.png"
				} else if (selectedIncident.type == "Accident") {
					image = "image/accident_red.png"
				} else if (selectedIncident.type == "Theft") {
					image = "image/theft_red.png";
				} else {
					image = "http://maps.google.com/mapfiles/ms/icons/red-dot.png";
				}
			} else {
				if (selectedIncident.type == "Fire") {
					image = "image/fire_green.png"
				} else if (selectedIncident.type == "Accident") {
					image = "image/accident_green.png"
				} else if (selectedIncident.type == "Theft") {
					image = "image/theft_green.png";
				} else {
					image = "http://maps.google.com/mapfiles/ms/icons/green-dot.png";
				}
			}

			var marker = new google.maps.Marker({
				position: myLatLng,
				map: this.map,
				title: selectedIncident.category[0],
				icon: image
			});

			marker.addListener('click', function() {
				infowindow.open(this.map, marker);
				jQuery("#assignIncidentButton").on("click", self.assignToIncident.bind(self, selectedIncident, marker));
			});
			this.markerList.push(marker);
		},

		returnedAssignedIncident: function(police) {
			return police.isAssigned ? police.assignedTo : "NA";
		},

		addAssignButton: function(test) {
			if (test == 'fit') {
				return '<button id = "markerButton" float = "bottom" width = "100%">Assign</button>';
			}
			return '';
		},

		addAssignButtonForIncident: function(test) {
			if (test >= 0) {
				return '<button id = "assignIncidentButton" float = "bottom" width = "100%">Assign</button>';
			}
			return '';
		},

		addButtonString: function(test) {
			if (test) {
				return '<button id = "unassignButton">Unassign</button>';
			}
			return '';
		},

		addPoliceMarkerToMap: function(myLatLng, selectedPolice) {
			var self = this;
			var contentString = '<div class="flightDetailContainer">' +
				'<div class="flightDetailContainerLeft">' +
				'<div class="fdcFlightNo">' + selectedPolice.name + '</div>' +
				'<div class="fdcFlightOPName">' + selectedPolice.badgeNum + '</div>' +
				'<div class="fdcFlightOPName">Health Status: ' + selectedPolice.healthStatus + '</div>' +
				'<br>' +
				'<div class="fdcFlightNo">AssignedTo</div>' +
				'<div class="fdcFlightOPName">' + this.returnedAssignedIncident(selectedPolice) + '</div>' + this.addButtonString(selectedPolice.isAssigned) +
				'</div>' +
				'<div class="flightDetailContainerRight">' +
				'<select id = "incidentSelect">';
			var incidents = this.model.getData().IncidentCollection;
			for (var num_3 in incidents) {
				var selected_incident = incidents[num_3];
				contentString += '<option value="' + selected_incident.eventNum + '">' + selected_incident.eventNum + '-' + selected_incident.category +
					'</option>';
			}
			contentString += '</select>';
			contentString += '<br><br>';
			contentString += this.addAssignButton(selectedPolice.healthStatus);
			contentString += '</div>';

			var infowindow = new google.maps.InfoWindow({
				content: contentString
			});

			var image = (selectedPolice.isAssigned) ? "image/policeman_assigned.png" : "image/policeman.png";

			var marker = new google.maps.Marker({
				position: myLatLng,
				map: this.map,
				icon: image
			});

			marker.addListener('click', function() {
				infowindow.open(this.map, marker);
				jQuery("#markerButton").on("click", self.assignPoliceman.bind(self, selectedPolice, marker));
				jQuery("#unassignButton").on("click", self.unassignPoliceman.bind(self, selectedPolice, marker));
			});
			this.markerList.push(marker);
		},

		addPoliceMarkerToMap2: function(myLatLng, selectedPolice) {
			var self = this;
			var contentString = '<div style="width: 250px height: 130px">' +
				'<div class="flightDetailContainerLeft">' +
				'<div class="fdcFlightNo">' + selectedPolice.name + '</div>' +
				'<div class="fdcFlightOPName">' + selectedPolice.badgeNum + '</div>' +
				'<div style="font-size: 12px margin-top: 6px font-style: bold font-weight: bold">Health Status: ' + selectedPolice.healthStatus + '</div>' ;
				

			var infowindow = new google.maps.InfoWindow({
				content: contentString
			});

			if(selectedPolice.healthStatus === "unfit")
			{
			var image = "image/policeman_unfit.png";
			}
			else{
				var image = "image/policeman_fit.png";
			}

			var marker = new google.maps.Marker({
				position: myLatLng,
				map: this.map,
				icon: image
			});
			marker.addListener('click', function() {
				infowindow.open(this.map, marker);
			});
			this.markerList.push(marker);
		},
		
		deleteAllMarkers: function() {
			for (var i = 0; i < this.markerList.length; i++) {
				this.markerList[i].setMap(null);
			}
			this.markerList = [];
		},

		onAfterRendering: function() {
			this.initialized = true;
			this.geocoder = new google.maps.Geocoder();
			var mapOptions = {
				center: new google.maps.LatLng(28.6331308, 77.0517014),
				zoom: 12,
				mapType: "terrain",
				mapTypeId: google.maps.MapTypeId.ROADMAP
			};
			var map = new google.maps.Map(this.getView().byId("map_canvas1").getDomRef(),
					mapOptions),
				directionsService = new google.maps.DirectionsService,
				directionsDisplay = new google.maps.DirectionsRenderer({
					map: map
				});
			this.map = map;
			var centerControlDiv = document.createElement('div');
			var centerControl = new this.CenterControl(centerControlDiv, map, this);
			centerControlDiv.index = 1;
			map.controls[google.maps.ControlPosition.TOP_CENTER].push(centerControlDiv);
			if (!this.legends) {
				var sid = this.getView().byId("legend").getId();
				var legend = document.getElementById(sid);
				var icons = this.createLegends("1")[0];
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
			map.controls[google.maps.ControlPosition.RIGHT_BOTTOM].push(legend);
			/*var oVizFrame = this.getView().byId("idVizFrame");

			oVizFrame.setVizProperties({
				legend: {
					title: {
						visible: false
					},
					visible: false
				},
				title: {
					visible: true
				}
			});*/

			var pieModel = new sap.ui.model.json.JSONModel();
			var data = {
				'IncidentStatus': [{
					"Status": "Closed",
					"Number": "30"
				}, {
					"Status": "In Progress",
					"Number": "40"
				}, {
					"Status": "Open",
					"Number": "30"
				}]
			};
			pieModel.setData(data);

			var oDataset = new sap.viz.ui5.data.FlattenedDataset({
				dimensions: [{
					name: 'Status',
					value: "{Status}"
				}],

				measures: [{
					name: 'Number',
					value: "{Number}"
				}],

				data: {
					path: "/IncidentStatus"
				}
			});

			/*			oVizFrame.setDataset(oDataset);
						oVizFrame.setModel(pieModel);

						oVizFrame.setVizProperties({
							title: {
								text: "Incident Status"
							},
							plotArea: {
								colorPalette: d3.scale.category20().range(),
								drawingEffect: "glossy",
								height: "10%",
								width: "10%"
							}
						});*/

			/*var feedSize = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "size",
					'type': "Measure",
					'values': ["Number"]
				}),
				feedColor = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "color",
					'type': "Dimension",
					'values': ["Status"]
				});*/
			/*oVizFrame.addFeed(feedSize);
			oVizFrame.addFeed(feedColor);

			var oVizFrame2 = this.getView().byId("idVizFrame2");

			oVizFrame2.setVizProperties({
				legend: {
					title: {
						visible: false
					},
					visible: false
				},
				title: {
					visible: true
				}
			});
*/
			/*var pieModel2 = new sap.ui.model.json.JSONModel();
			var data2 = {
				'SentimentStatus': [{
					"Sentiment": "Positive",
					"Number": "70"
				}, {
					"Sentiment": "Neagtive",
					"Number": "20"
				}, {
					"Sentiment": "Neutral",
					"Number": "10"
				}]
			};
			pieModel2.setData(data2);

			var oDataset2 = new sap.viz.ui5.data.FlattenedDataset({
				dimensions: [{
					name: 'Sentiment',
					value: "{Sentiment}"
				}],

				measures: [{
					name: 'Number',
					value: "{Number}"
				}],

				data: {
					path: "/SentimentStatus"
				}
			});

			oVizFrame2.setDataset(oDataset2);
			oVizFrame2.setModel(pieModel2);

			oVizFrame2.setVizProperties({
				title: {
					text: "Sentiment Status"
				},
				plotArea: {
					colorPalette: d3.scale.category20().range(),
					drawingEffect: "glossy",
					height: "10%",
					width: "10%"
				}
			});

			var feedSize2 = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "size",
					'type': "Measure",
					'values': ["Number"]
				}),
				feedColor2 = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "color",
					'type': "Dimension",
					'values': ["Sentiment"]
				});
			oVizFrame2.addFeed(feedSize2);
			oVizFrame2.addFeed(feedColor2);

			var oVizFrame3 = this.getView().byId("idVizFrame3");

			oVizFrame3.setVizProperties({
				legend: {
					title: {
						visible: false
					},
					visible: false
				},
				title: {
					visible: true
				}
			});

			var pieModel3 = new sap.ui.model.json.JSONModel();
			var data3 = {
				'FitnessStatus': [{
					"Fitness": "Fit",
					"Number": "5"
				}, {
					"Fitness": "Unfit",
					"Number": "1"
				}]
			};
			pieModel3.setData(data3);

			var oDataset3 = new sap.viz.ui5.data.FlattenedDataset({
				dimensions: [{
					name: 'Fitness',
					value: "{Fitness}"
				}],

				measures: [{
					name: 'Number',
					value: "{Number}"
				}],

				data: {
					path: "/FitnessStatus"
				}
			});

			oVizFrame3.setDataset(oDataset3);
			oVizFrame3.setModel(pieModel3);

			oVizFrame3.setVizProperties({
				title: {
					text: "Fitness Status"
				},
				plotArea: {
					colorPalette: d3.scale.category20().range(),
					drawingEffect: "glossy",
					height: "6%",
					width: "6%"
				}
			});

			var feedSize3 = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "size",
					'type': "Measure",
					'values': ["Number"]
				}),
				feedColor3 = new sap.viz.ui5.controls.common.feeds.FeedItem({
					'uid': "color",
					'type': "Dimension",
					'values': ["Fitness"]
				});
			oVizFrame3.addFeed(feedSize3);
			oVizFrame3.addFeed(feedColor3);*/

		},

		createLegends: function(a) {
			if (a === "1") {
				var legend1 = new Object();
				legend1.name = "Incident Closed";
				legend1.icon = "image/green_dot.png";

				var legend2 = new Object();
				legend2.name = "Incident Open";
				legend2.icon = "image/red_dot.png";

				var legend3 = new Object();
				legend3.name = "Incident In Progress";
				legend3.icon = "image/yellow_dot.png";
			}
			if (a === "2") {
				var legend1 = new Object();
				legend1.name = "Active Officer";
				legend1.icon = "image/green_dot.png";

				var legend2 = new Object();
				legend2.name = "Inactive Officer";
				legend2.icon = "image/red_dot.png";
			}
			/*var legend4 = new Object();
			legend4.name = "Policeman (Unassigned)";
			legend4.icon = "image/policeman.png";

			var legend5 = new Object();
			legend5.name = "Policeman (Assigned)";
			legend5.icon = "image/policeman_assigned.png";
			
			var legend6 = new Object();
			legend6.name = "Fire Incident (Open)";
			legend6.icon = "image/fire_red.png";
			
			var legend7 = new Object();
			legend7.name = "Fire Incident (In Progress)";
			legend7.icon = "image/fire_yellow.png";
			
			var legend8 = new Object();
			legend8.name = "Fire Incident (Closed)";
			legend8.icon = "image/fire_green.png";
			
			var legend9 = new Object();
			legend9.name = "Traffic Incident (Open)";
			legend9.icon = "image/accident_red.png";
			
			var legend10 = new Object();
			legend10.name = "Traffic Incident (In Progress)";
			legend10.icon = "image/accident_yellow.png";
			
			var legend11 = new Object();
			legend11.name = "Traffic Incident (Closed)";
			legend11.icon = "image/accident_green.png";
			
			var legend12 = new Object();
			legend12.name = "Theft Incident (Open)";
			legend12.icon = "image/theft_red.png";
			
			var legend13 = new Object();
			legend13.name = "Theft Incident (In Progress)";
			legend13.icon = "image/theft_yellow.png";
			
			var legend14 = new Object();
			legend14.name = "Theft Incident (Closed)";
			legend14.icon = "image/theft_green.png";*/

			//return new Array([legend4, legend5, legend6, legend7, legend8, legend9, legend10, legend11, legend12, legend13, legend14]);
			return new Array([legend1, legend2, legend3]);
		},

		onNavBack: function(oEvent) {
			this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true);
		},

		loadMarkersOnMap: function(oModel, policeModel) {
			if (!this.policeFlag) {
				var incidents = oModel.getData().IncidentCollection;
				for (var num_3 in incidents) {
					var selected_incident = incidents[num_3];
					var myLatLng = {
						lat: selected_incident.lat,
						lng: selected_incident.long
					};
					this.addMarkerToMap(myLatLng, selected_incident);
				}

				var policemen = policeModel.getData().PoliceList;
				for (var num_4 in policemen) {
					var selected_policeman = policemen[num_4];
					var policeLatLng = {
						lat: selected_policeman.lat,
						lng: selected_policeman.long
					};
					this.addPoliceMarkerToMap(policeLatLng, selected_policeman);
				}
			} else {
				var policemen = this.policeModel.getData().PoliceList;
				for (var num_4 in policemen) {
					var selected_policeman = policemen[num_4];
					var policeLatLng = {
						lat: selected_policeman.lat,
						lng: selected_policeman.long
					};
					this.addPoliceMarkerToMap2(policeLatLng, selected_policeman);
				}
			}
		},

		CenterControl: function(controlDiv, map, self) {

			var controlUI = document.createElement('div');
			controlUI.style.backgroundColor = '#fff';
			controlUI.style.border = '2px solid #fff';
			controlUI.style.borderRadius = '3px';
			controlUI.style.boxShadow = '0 2px 6px rgba(0,0,0,.3)';
			controlUI.style.cursor = 'pointer';
			controlUI.style.marginBottom = '22px';
			controlUI.style.textAlign = 'center';
			controlUI.title = 'Click to Load Data';
			controlDiv.appendChild(controlUI);

			var controlText = document.createElement('div');
			controlText.style.color = 'rgb(25,25,25)';
			controlText.style.fontFamily = 'Roboto,Arial,sans-serif';
			controlText.style.fontSize = '16px';
			controlText.style.lineHeight = '38px';
			controlText.style.paddingLeft = '5px';
			controlText.style.paddingRight = '5px';
			controlText.innerHTML = 'Load Data';
			controlUI.appendChild(controlText);

			controlUI.addEventListener('click', function() {
				self.loadMarkersOnMap(self.model, self.policeModel);
			});

		},

		assignToIncident: function(selectedIncident, marker, oEvent) {
			var index = jQuery("#policeSelect")[0].selectedIndex;
			var selectedPolice = this.createEligiblePolice()[index];
			selectedPolice.isAssigned = true;
			selectedPolice.assignedTo = selectedIncident.eventNum;
			selectedIncident.officers.push(selectedPolice.badgeNum);
			this.deleteAllMarkers();
			this.loadMarkersOnMap(this.model, this.policeModel);
		},

		assignPoliceman: function(selectedPolice, marker, oEvent) {
			this.assign(selectedPolice, marker);
			this.deleteAllMarkers();
			this.loadMarkersOnMap(this.model, this.policeModel);
		},

		unassignPoliceman: function(selectedPolice, marker, oEvent) {
			this.unassign(selectedPolice, marker);
			this.deleteAllMarkers();
			this.loadMarkersOnMap(this.model, this.policeModel);
		},

		assign: function(selectedPolice, marker) {
			if (selectedPolice.isAssigned) {
				this.unassign(selectedPolice, marker);
			}
			selectedPolice.isAssigned = true;
			var selectedIndex = jQuery("#incidentSelect")[0].selectedIndex;
			var incidents = this.model.getData().IncidentCollection;
			var selectedIncident = incidents[selectedIndex];
			selectedPolice.assignedTo = selectedIncident.eventNum;
			selectedIncident.officers.push(selectedPolice.badgeNum);
		},

		unassign: function(selectedPolice, marker) {
			selectedPolice.isAssigned = false;
			var incidents = this.model.getData().IncidentCollection;
			var selectedIncident;
			for (var index in incidents) {
				if (incidents[index].eventNum == selectedPolice.assignedTo) {
					selectedIncident = incidents[index];
				}
			}
			selectedPolice.assignedTo = "";
			selectedIncident.officers.splice(selectedIncident.officers.indexOf(selectedPolice.assignedTo), 1);

		},
		navToLiveStream: function() {
			this.getOwnerComponent().getRouter().navTo("videoCapture", {}, true);
		},

		onLoadOfficerView: function(oEvent) {
			var viewID = this.getView().byId("viewType").getText();
			if (viewID === "Officer View") {
				this.policeFlag = true;
				this.dispatcherFlag = false;
				this.getView().byId("viewType").setText("Dispatcher View");
				this.deleteAllMarkers();
				this.loadMarkersOnMap(this.model, this.policeModel);
				this.legends = false;
				if (!this.legends) {
				var sid = this.getView().byId("legend").getId();
				document.getElementById(sid).innerHTML = "";
				var legend = document.getElementById(sid);
				var icons = this.createLegends("2")[0];
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
			this.map.controls[google.maps.ControlPosition.RIGHT_BOTTOM].push(legend);
			} else {
				this.dispatcherFlag = true;
				this.policeFlag = false;
				this.getView().byId("viewType").setText("Officer View");
				this.deleteAllMarkers();
				this.loadMarkersOnMap(this.model, this.policeModel);
				this.legends = false;
				if (!this.legends) {
				var sid = this.getView().byId("legend").getId();
				document.getElementById(sid).innerHTML = "";
				var legend = document.getElementById(sid);
				var icons = this.createLegends("1")[0];
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
			this.map.controls[google.maps.ControlPosition.RIGHT_BOTTOM].push(legend);
				
			}
		}

	});
});