var view = null;
var that = null;
sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History",
	"sap/m/MessageToast",
	"sap/ui/model/json/JSONModel"
], function(Controller, History, MessageToast, JSONModel) {
	"use strict";

	var latc;
	var longc;
	var _timeout;

	return Controller.extend("com.sap.Aviation.controller.createIncident", {
		count: 0,
		view: null,
		_data: {
			"date": new Date()
		},
		onInit: function(evt) {
			// create model
			var oModel = new JSONModel();
			oModel.setData({
				dateValue: new Date()
			});
			this.getView().setModel(oModel);

			this.byId("TP1").setDateValue(new Date());
			this.byId("DP1").setDateValue(new Date());
		
			var incidentModel = new JSONModel({
				"type": "",
				"image": "",
				"emergency": true,
				"date": new Date(),
				"time": new Date(),
				"comments": "",
				"status": "In Progress"
			});

			var incidentData = new JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incidentData.json"));
			this.inciData = incidentData;
			var incident1 = new JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incident1.json"));
			this.getView().setModel(incident1, "incident1");

			this.getView().setModel(incidentModel, "incidentModel");
			this.getView().setModel(incidentData, "incidentData");
			view = this.getView();
			that = this;
			navigator.geolocation.getCurrentPosition(function(position) {
				that.handleLocation(position);
			}, this.onGeoError, {
				enableHighAccuracy: true
			});
			var oRouter = sap.ui.core.UIComponent.getRouterFor(this);
			oRouter.getRoute("create").attachMatched(this._onRouteMatched, this);
		},
				_onRouteMatched: function(oEvent){
		//	this.getOwnerComponent().getModel("IncidentsData").refresh();
			var comments = this.getCommentsData();
			that.oView.getModel("incidentModel").setProperty("/comments", comments);
		},
		getCommentsData: function(){
		return this.getOwnerComponent().getModel("MLModel").getProperty("/comments");	
		},
		callICMCreateIncident: function() {
			//begin of service call 
			var oModel = new sap.ui.model.json.JSONModel();
			var loadUrl =
				"../destinations/CRZ_756_CLA_HTTP/sap/bc/srt/wsdl/flv_10002A111AD1/bndg_url/sap/bc/srt/rfc/sap/zcreateicminci/756/zcreateincident/zbinding?sap-client=756";

			var self = this;
			$.ajax({
				url: loadUrl,
				type: "POST",
				dataType: "json",
				success: function(data) {
					alert("ICM Incidnet Created");
				},
				error: function() {
						alert("Could not create ICM Incident");
					}
					//,
					//xhrFields: {
					//	withCredentials: true
					//}
			});
		},
		handleLocation: function(position) {
			console.log('code: ' + position.coords.latitude);

			latc = position.coords.latitude;
			longc = position.coords.longitude;
			console.log('latc:' + latc + longc);
			this.setMap();
			//sap.ui.getCore().byId("txtLatitude").setText(position.coords.latitude);
			//this.getView.byId("txtLatitude").setText("position.coords.latitude");
			view.byId("txtLatitude").setText(position.coords.latitude);
			view.byId("txtLongitude").setText(position.coords.longitude);
			//view.byId("txtAltitude").setText(position.coords.altitude);
			console.log("latc:" + latc + longc);
		},
		onGeoError: function() {
			console.log('code: ' + error.code + '\n' + 'message: ' + error.message + '\n');
		},
		onPhotoDataSuccess: function(imageData) {
			var myImage = view.byId("myImage");
			myImage.setSrc("data:image/jpeg;base64," + imageData);
			that.oView.getModel("incidentModel").setProperty("/image", imageData);
		},
		onPhotoURISuccess: function(imageURI) {
			var myImage = view.byId("myImage");
			myImage.setSrc(imageURI);
			that.oView.getModel("incidentModel").setProperty("/image", imageURI);
		},
		onFail: function(message) {
			console.log("Failed because: " + message);
		},
		getPhoto: function() {
			var oNav = navigator.camera;
			oNav.getPicture(this.onPhotoURISuccess, this.onFail, {
				quality: 50,
				destinationType: oNav.DestinationType.FILE_URI,
				sourceType: oNav.PictureSourceType.PHOTOLIBRARY
			});
		},
		capturePhoto: function() {
			var oNav = navigator.camera;
			oNav.getPicture(that.onPhotoDataSuccess, that.onFail, {
				quality: 10,
				destinationType: oNav.DestinationType.DATA_URL
			});
		},
		onNavBack: function(oEvent) {
			var oHistory, sPreviousHash;
			oHistory = History.getInstance();
			sPreviousHash = oHistory.getPreviousHash();
			if (sPreviousHash !== undefined) {
				window.history.go(-1);
			} else {
				this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true);
			}
			this.getOwnerComponent().getModel("MLModel").setProperty("/comments","");
		},

		updateType: function(oEvent) {
			var type = oEvent.getSource().getSelectedKey();
			this.type = type;
			that.oView.getModel("incidentModel").setProperty("/type", type);
		},

		cancelPress: function() {
			this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true);
		},
		onCreate: function(oEvent) {
			var self = this;
			var incident_json = this.getView().getModel("incident1").oData;
			var latn = this.getView().byId("txtLatitude").getText();
			var longn = this.getView().byId("txtLongitude").getText();
			incident_json.lat = latn;
			incident_json.long = longn;
			incident_json.comment = this.getView().byId("TA1").getValue();
			incident_json.date = this.getView().byId("DP1").getValue();
			incident_json.em_type = this.getView().byId("cbIncident").getValue();
			incident_json = JSON.stringify(incident_json);
			this.onOpenDialog(oEvent);

			var incidentStatusModel = this.getOwnerComponent().getModel("incidentStatusModel");
			var incidentsDataModel = new JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/IncidentsData.json"));
			//this.getView().setModel(inc)
			var incidentSet = incidentStatusModel.getProperty("/IncidentSet");
			var randomnumber = Math.floor(Math.random() * 100) + 1;
			//var incidentModel = that.oView.getModel("incidentModel");
			var comments = this.getView().byId("TA1");
			var type = this.getView().byId("cbIncident").getValue();
			var createIncidentModel = new JSONModel({
				"type": type,
				"image": "",
				"eventNum": randomnumber,
				"emergency": true,
				"date": new Date(),
				"time": new Date(),
				"comments": "",
				"status": "New",
				"ChatCollection": [{
					"Author": "Mr. Sandeep",
					"AuthorPicUrl": "",
					"Type": "Request",
					"Date": new Date(),
					"Text": comments.getValue()
				}]
			});

			var IncidentModel1 = new JSONModel({
				"lat": 25.2466026,
				"long": 55.3500956,
				"address": "Current Location",
				"eventNum": randomnumber,
				"category": type,
				"type": type,
				"date": new Date(),
				"time":new Date(),
				"comments": type,
				"priority": 3,
				"status": "New",
				"ambulanceNeeded": false,
				"fireTruckNeeded": true,
				"policeAssigned": false,
				"officerInCharge": "Mr. Atif Khan",
				"officers": [],
				"ChatCollection": [{
					"Author": "Mr. Sandeep",
					"AuthorPicUrl": "",
					"Type": "Request",
					"Date": new Date(),
					"Text": comments.getValue()
				}]
			});
			incidentSet.push(createIncidentModel.getData());
			incidentsDataModel.attachRequestCompleted(function() {
				var incidents = incidentsDataModel.getProperty("/IncidentsReported");
				incidents.push(createIncidentModel.getData());
			});
			this.getView().setModel(incidentsDataModel);
			incidentStatusModel.setProperty("/IncidentSet", incidentSet);

			console.log(incidentSet);

			setTimeout(function() {
				$.ajax({
					url: 'https://hyperledger-api.cfapps.eu10.hana.ondemand.com/invoke',
					async: false,
					type: "POST",
					contentType: "application/json",
					headers: {
						"apiKey": "7RhNlTkJYr5cmPqB4c2s262SvYIedsYYp6v56B5PPJYLau9iGcnP97vgXdjM63ShLKXzLdvjf99c1pUs8lmtaJbjUtulJIYx2CYdRHVCMY7iWlUVI83ZvljStR3I9fBkZjMPcOyFxweOF09CKJdP4a5xtJz8Y7dA6HRYhjQjjsOjbxbTtFljvJajIVmfO7M6eZCU93EOv9TBSlwcEBhXZfsePLOjnh4QU1PU9k06mfZmnxYh4jlpY4PU9D6iBAP5Cei7Cew9To1moAMArMwxJ6qdwR37h5qqtrnuPzTU4YYH5dxQkZfPVf6fLrzaViUDeVjdS250zY4EorWJJxlRYnh4gtHc3edoctF81F2p1g5Y7RBHCmKUOZk6SSqTBBh43iXm4cyz3mCnqnpKh83PfKtCAyQZkk8Wl2bE6neM5RbigK0JFhT1etNo8t4wBdAJ9Y01IW8GhhgqHtW2ZlNelLa9mfMcd4AiiG9HnMqN9heLm2cDtvjUMEwjhFo1Vcan"
					},
					dataType: "json",
					data: JSON.stringify({
						"chaincodeId": "cadd43e72e6aa8299827a94b7a2955a0",
						"fcn": "append_data",
						"args": [incident_json]
					}),
					success: function(data) {
						//alert("success and the data response" + data.Response); 
						sap.m.MessageToast.show("Entry Created successfully", {
							duration: 5000 // default width: "15em", // default my: "center bottom", // default at: "center bottom", // default of: window, // default offset: "0 0", // default collision: "fit fit", // default onClose: null, // default autoClose: true, // default animationTimingFunction: "ease", // default animationDuration: 1000, // default closeOnBrowserNavigation: true // default 
						});
						var createData = createIncidentModel.oData;
						var createIncidentDispatcher = IncidentModel1.oData;
						self.getOwnerComponent().getModel("IncidentsData").getProperty("/IncidentsReported").push(createData);
						self.getOwnerComponent().getModel("incidentCollection").getProperty("/IncidentCollection").push(createIncidentDispatcher);
						self.getOwnerComponent().getRouter().navTo("incidentStatus", {}, true);
						//	self.callICMCreateIncident();
						sap.m.MessageToast.show('Emergency incident created successsfully');
						self.getOwnerComponent().getModel("MLModel").setProperty("/comments","");
					},
					error: function(data) {
						sap.m.MessageToast.show("Entry Creation Failed( " + data.statusText + " )", {
							duration: 3000 // default width: "15em", // default my: "center bottom", // default at: "center bottom", // default of: window, // default offset: "0 0", // default collision: "fit fit", // default onClose: null, // default autoClose: true, // default animationTimingFunction: "ease", // default animationDuration: 1000, // default closeOnBrowserNavigation: true // default 
						});
						self.getOwnerComponent().getModel("MLModel").setProperty("/comments","");
					}
				});
			}, 3000);
			this.getOwnerComponent().getModel("MLModel").setProperty("comments","");
		},

		onAfterRendering: function(oEvent) {
			var oModel = new JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incidentData.json"));
			this.getView().setModel(oModel);

		},
		setMap: function() {
			this.geocoder = new google.maps.Geocoder();
			this.getView().byId("DP1").setDateValue(new Date());

			var mapOptions = {
				center: new google.maps.LatLng(parseFloat(latc), parseFloat(longc)),
				zoom: 20,
				mapType: "terrain",
				mapTypeId: google.maps.MapTypeId.ROADMAP
			};

			var map = new google.maps.Map(this.getView().byId("currentLocation").getDomRef(),
					mapOptions),
				directionsService = new google.maps.DirectionsService,
				directionsDisplay = new google.maps.DirectionsRenderer({
					map: map
				});

			var myLatLng = {
				lat: parseFloat(latc),
				lng: parseFloat(longc)
			};
			/*var marker = new google.maps.Marker({
				position: myLatLng,
				map: map,
				title: 'Current Position'
			});*/

			var geocoder = new google.maps.Geocoder();
			var infowindow = new google.maps.InfoWindow();

			geocoder.geocode({
				'location': myLatLng
			}, function(results, status) {
				if (status === 'OK') {
					if (results[0]) {
						map.setZoom(11);
						var marker = new google.maps.Marker({
							position: myLatLng,
							map: map
								/*,
								                draggable: true*/
						});
						infowindow.setContent(results[0].formatted_address);
						infowindow.open(map, marker);
						// Marker.setDraggable(true);
					} else {
						window.alert('No results found');
					}
				} else {
					window.alert('Geocoder failed due to: ' + status);
				}
			});
		},
		onHistoryPress: function() {
			this.getOwnerComponent().getRouter().navTo("incidentStatus", {}, true);
		},
		setIncidentStatusModel: function(input) {
			var type = that.oView.byId(input).getText();
			var incidentStatusModel = this.getOwnerComponent().getModel("incidentStatusModel");
			var incidentSet = incidentStatusModel.getProperty("/IncidentSet");
			//var incidentModel = that.oView.getModel("incidentModel");
			var incidentModel = new JSONModel({
				"type": "",
				"image": "",
				"emergency": true,
				"date": new Date(),
				"time": new Date(),
				"comments": "",
				"status": "In Progress"
			});
			incidentModel.setProperty("/type", type);
			incidentSet.push(incidentModel.getData());
			incidentStatusModel.setProperty("/IncidentSet", incidentSet);
			incidentModel.refresh(true);
			incidentStatusModel.refresh(true);
		},
		handleImagePress: function(oEvent) {
			var selectedItem = oEvent.getSource().getId().split("--")[1];
			var comboBox = this.getView().byId("cbIncident");
			comboBox.setSelectedKey(this.inciData.getData().incidentCollection[selectedItem].type);
		},
		handleImage8Press: function() {
			var comboBox = this.getView().byId("cbIncident");
			comboBox.setSelectedKey(this.inciData.getData().incidentCollection[7].type);
			this.setIncidentStatusModel("others");

			this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true);
			sap.m.MessageToast.show('Other type emergency incident created successsfully');
		},
		onOpenDialog: function(oEvent) {
			// instantiate dialog
			if (!this._dialog) {
				this._dialog = sap.ui.xmlfragment("com.sap.Aviation.view.fragment.busyDialog", this);
				this.getView().addDependent(this._dialog);
			}

			// open dialog
			jQuery.sap.syncStyleClass("sapUiSizeCompact", this.getView(), this._dialog);
			this._dialog.open();

			// simulate end of operation
			_timeout = jQuery.sap.delayedCall(3000, this, function() {
				this._dialog.close();
			});
		}
	});
});