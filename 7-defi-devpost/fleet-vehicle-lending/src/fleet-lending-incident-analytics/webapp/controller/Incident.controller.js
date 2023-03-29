sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History"
], function(Controller, History) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.Incident", {

		onInit: function() {
			var oModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incidentCollection.json"));
			this.model = oModel;
			oModel.attachRequestCompleted(function() {
				this.getView().setModel(oModel);
			});
			this.markerList = new Array();
			this.legends = false;
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
			var map = new google.maps.Map(this.getView().byId("map_canvas").getDomRef(),
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

			this.setMyLocation();
			if(!this.legends){
			var sid = this.getView().byId("legend").getId();
			var legend = document.getElementById(sid);
			
			var icons = this.createLegends()[0];
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
		},
		
		createLegends: function() {
			var legend1 = new Object();
			legend1.name = "Incident Closed";
			legend1.icon = "image/green_dot.png";

			var legend2 = new Object();
			legend2.name = "Incident Open";
			legend2.icon = "image/red_dot.png";

			var legend3 = new Object();
			legend3.name = "Incident In Progress";
			legend3.icon = "image/yellow_dot.png";
			/*var legend6 = new Object();
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
			legend14.icon = "image/theft_green.png";
			*/

			return new Array([legend1, legend2, legend3]);
		},

		onNavBack: function(oEvent) {
			var oHistory, sPreviousHash;
			oHistory = History.getInstance();
			sPreviousHash = oHistory.getPreviousHash();
			if (sPreviousHash !== undefined) {
				window.history.go(-1);
			} else {
				this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true /*no history*/ );
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
				self.loadMarkersOnMap(self.model);
			});

		},

		loadMarkersOnMap: function(oModel) {
			var incidents = oModel.getData().IncidentCollection;
			for (var num_3 in incidents) {
				var selected_incident = incidents[num_3];
				var myLatLng = {
					lat: selected_incident.lat,
					lng: selected_incident.long
				};
				this.addMarkerToMap(myLatLng, selected_incident);
			}
		},

		addMarkerToMap: function(myLatLng, selectedIncident) {
			var self = this;
			var contentString = '<div id="content">' +
				'<p>' + "Incident No.: " + selectedIncident.eventNum + '</p>' +
				'<p>' + "Category: " + selectedIncident.category + '</p>' +
				'<p>' + "Status: " + selectedIncident.status + '</p>' +
				'<p>' + "Comments: " + selectedIncident.comments + '</p><br><br>' +
				'<div class="fdc_link"><a href="#tabPage/'+selectedIncident.eventNum+'"><span class="fdc_ref_icon"></span><span class="fdc_ref_text">Details</span></a></div>' +
				'</div>';

			var infowindow = new google.maps.InfoWindow({
				content: contentString
			});
			
			var image;
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
				icon:image
			});

			marker.addListener('click', function() {
				infowindow.open(this.map, marker);
				jQuery("#assignBtn").on("click", self.assignMe.bind(myLatLng, self));

			});
			this.markerList.push(marker);
		},

		assignMe: function(myLatLng, event) {
			sap.m.MessageToast.show("You have been assigned successfully");

			var directionsService = new google.maps.DirectionsService;
			var directionsDisplay = new google.maps.DirectionsRenderer;
			
			var end = new google.maps.LatLng(myLatLng.lat, myLatLng.long);
			var start = new google.maps.LatLng(this.pos.lat, this.pos.long);
			directionsService.route({
				origin: start,
				destination: end,
				travelMode: 'DRIVING'
			}, function(response, status) {
				if (status === 'OK') {
					directionsDisplay.setDirections(response);
				} else {
					window.alert('Directions request failed due to ' + status);
				}
			});
		},

		setMyLocation: function() {

			var self = this;
			var marker = new google.maps.Marker({
				map: this.map,
				icon: "image/current_loc.png"
			});

			if (navigator.geolocation) {
				navigator.geolocation.getCurrentPosition(function(position) {
					var pos = {
						lat: position.coords.latitude,
						lng: position.coords.longitude
					};
					self.pos = pos;
					pos.lat = 25.117369;
					pos.lng =  55.279894;
					marker.setPosition(pos);
					self.map.setCenter(pos);
				}, function() {
					self.handleLocationError(true, infoWindow, self.map.getCenter());
				});
			} else {
				self.handleLocationError(false, infoWindow, self.map.getCenter());
			}
		},

		handleLocationError: function(browserHasGeolocation) {
			alert(browserHasGeolocation ?
				'Error: The Geolocation service failed.' :
				'Error: Your browser doesn\'t support geolocation.');
		},

		calculateAndDisplayRoute: function(directionsService, directionsDisplay, myLatLng) {
			directionsService.route({
				origin: this.pos,
				destination: myLatLng,
				travelMode: 'DRIVING'
			}, function(response, status) {
				if (status === 'OK') {
					directionsDisplay.setDirections(response);
				} else {
					window.alert('Directions request failed due to ' + status);
				}
			});
		}

	});

});