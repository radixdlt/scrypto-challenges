sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History"
], function(Controller, History) {
	"use strict";

	var self;
	return Controller.extend("com.sap.Aviation.controller.IncidentsAnalysis", {

		onInit: function() {
			self = this;
			var oModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incidentCollection.json"));
			this.model = oModel;
			oModel.attachRequestCompleted(function() {
				this.getView().setModel(oModel);
			});
			this.markerList = new Array();
			this.legends = false;
		},

		onAfterRendering: function() {
			var self = this;
			this.initialized = true;
			this.geocoder = new google.maps.Geocoder();
			var mapOptions = {
				center: new google.maps.LatLng(28.6331308, 77.0517014),
				zoom: 10,
				mapType: "terrain",
				mapTypeId: google.maps.MapTypeId.ROADMAP
			};
			var citymap = {
				tilakNagar: {
					center: {
						lat: 28.6331308,
						lng: 77.0517014
					},
					population: 2714856
				},
				rajouri: {
					center: {
						lat: 28.6447483,
						lng: 77.0893887
					},
					population: 2714156
				}
			};
			var citymap1 = {

				mundaka: {
					center: {
						lat: 28.6810769,
						lng: 76.9999138
					},
					population: 2114156
				},
				golemarket: {
					center: {
						lat: 28.6106567,
						lng: 77.1692671
					},
					population: 2115156
				}

			};

			var map = new google.maps.Map(this.getView().byId("map_canvas").getDomRef(),
					mapOptions),
				directionsService = new google.maps.DirectionsService,
				directionsDisplay = new google.maps.DirectionsRenderer({
					map: map
				});
			this.map = map;
			for (var city in citymap) {
				// Add the circle for this city to the map.
				var cityCircle = new google.maps.Circle({
					strokeColor: '#FF0000',
					strokeOpacity: 0.8,
					strokeWeight: 2,
					fillColor: '#FF0000',
					fillOpacity: 0.35,
					map: map,
					center: citymap[city].center,
					radius: Math.sqrt(citymap[city].population) * 10
				});
				cityCircle.addListener('click', self.showNewRect.bind(self));
			}
			for (var city1 in citymap1) {
				// Add the circle for this city to the map.
				var cityCircle1 = new google.maps.Circle({
					strokeColor: '#00FF00',
					strokeOpacity: 0.8,
					strokeWeight: 2,
					fillColor: '#00FF00',
					fillOpacity: 0.35,
					map: map,
					center: citymap1[city1].center,
					radius: Math.sqrt(citymap1[city1].population) * 10
				});
				cityCircle1.addListener('click', self.showNewRect.bind(self));
			}

			//var centerControlDiv = document.createElement('div');
			//var centerControl = new this.CenterControl(centerControlDiv, map, this);

			//centerControlDiv.index = 1;
			//map.controls[google.maps.ControlPosition.TOP_CENTER].push(centerControlDiv);

			this.setMyLocation();
			if (!this.legends) {
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
				map.controls[google.maps.ControlPosition.RIGHT_BOTTOM].push(legend);
				this.legends = true;
			}
		},

		createLegends: function() {
			var legend1 = new Object();
			legend1.name = "Medium";
			legend1.icon = "http://maps.google.com/mapfiles/ms/icons/green-dot.png";

			var legend2 = new Object();
			legend2.name = "Critical";
			legend2.icon = "http://maps.google.com/mapfiles/ms/icons/red-dot.png";

			var legend3 = new Object();
			legend3.name = "Normal";
			legend3.icon = "http://maps.google.com/mapfiles/ms/icons/yellow-dot.png";

			var legend4 = new Object();
			legend4.name = "My Location";
			legend4.icon = "image/current_loc.png";

			return new Array([legend1, legend2, legend3]);
		},

		onNavBack: function(oEvent) {
			this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true /*no history*/ );
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
				'<div class="fdc_link"><a href="#tabPage/' + selectedIncident.eventNum +
				'"><span class="fdc_ref_icon"></span><span class="fdc_ref_text">Details</span></a></div>' +
				'</div>';

			var infowindow = new google.maps.InfoWindow({
				content: contentString
			});

			var image;
			if (selectedIncident.status == "pending") {
				image = "http://maps.google.com/mapfiles/ms/icons/yellow-dot.png";
			} else if (selectedIncident.status == "new") {
				image = "http://maps.google.com/mapfiles/ms/icons/red-dot.png";
			} else {
				image = "http://maps.google.com/mapfiles/ms/icons/green-dot.png";
			}

			var marker = new google.maps.Marker({
				position: myLatLng,
				map: this.map,
				title: selectedIncident.category[0],
				icon: image
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
						pos.lat = 28.6810769;
						pos.lng = 76.9999138;
					marker.setPosition(pos);
					self.map.setCenter(pos);
				}, function() {
					self.handleLocationError(true, infoWindow, self.map.getCenter());
				});
			} else {
				self.handleLocationError(false, infoWindow, self.map.getCenter());
			}
		},

		showNewRect: function(event) {
			//alert("click");
			var oRoute = self.getOwnerComponent().getRouter();
			oRoute.navTo("incidentanalysis");
		},
		handleLocationError: function(browserHasGeolocation) {
			alert(browserHasGeolocation ?
				'Error: The Geolocation service failed.' :
				'Error: Your browser doesn\'t support geolocation.');
		}

	});

});