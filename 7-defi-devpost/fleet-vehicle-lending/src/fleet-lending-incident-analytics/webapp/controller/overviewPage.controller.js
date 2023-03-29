sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.overviewPage", {

		/**
		 * Called when a controller is instantiated and its View controls (if available) are already created.
		 * Can be used to modify the View before it is displayed, to bind event handlers and do other one-time initialization.
		 * @memberOf com.sap.aviation.analyticspricemterics.view.view.overviewPage
		 */
		onInit: function() {
			this.zoomLevel = 8;
			this.position = {
				lat: 28.6331308,
				lng: 77.0517014
			};
			this.aircraftTypesModel = new sap.ui.model.json.JSONModel("./model/aircraftTypes.json", false);
			// this.flightModel = new sap.ui.model.json.JSONModel( "../elseco/Aviation/aviationservices.xsodata/flights?$filter=(FLIGHT_NO ne '' and TO ne '' and FROM ne '' and LONG gt '"+(this.position.lng-5)+"' and LONG lt '"+(this.position.lng+5)+"')&$top=50&$format=json" ,false);
			//this.flightModel = new sap.ui.model.json.JSONModel( "../elseco/Aviation/aviationservices.xsodata/flightRadar?$orderby=TIMESTAMP asc&$format=json" ,false);
			this.flightModel = new sap.ui.model.json.JSONModel(
				"../destinations/Planner/Aviation/aviationservices.xsodata/flights?$filter=(TIMESTAMP eq '1488454699' and TO ne '' and FROM ne '')&$format=json",
				false);

			this.router = this.getOwnerComponent().getRouter();
			this.router.getRoute("overview").attachPatternMatched(this._onRouteMatched, this);
		},

		_onRouteMatched: function(oEvent) {
			if (oEvent.getParameter("arguments")) {
				var industryType = oEvent.getParameter("arguments").industryOverview;
				var industryModel = new sap.ui.model.json.JSONModel({
					text: industryType
				});
				this.getView().setModel(industryModel, "industryModel");
			}

		},

		onAfterRendering: function() {

			var industryType = this.getView().getModel("industryModel").getProperty("/text");
			var mapViewID = this.getView().byId("wra-map").getId();
			this.renderMap(mapViewID, industryType);

		},

		renderMap: function(id, industryType) {

			// 		this.oModel.attachRequestCompleted(function(){
			// 			var d = that.oModel.getData();
			// 		var	data= that.oModel.getProperty("/response/data/AIRSIGMET/")
			// for (x in data.length){
			// var pathData= that.oModel.getProperty("/response/data/AIRSIGMET/"+x+"/area/point");
			// var xd=[];
			// for (y in pathData.length){
			// var path=[that.oModel.getProperty("/response/data/AIRSIGMET/"+x+"/area/point/"+y+"latitude"),that.oModel.getProperty("/response/data/AIRSIGMET/"+x+"/area/point/"+y+"longitude")];
			// xd.push[path];
			// }
			// console.log(xd);
			// }
			// 		});
			var map = new GMaps({
				div: '#' + id,
				lat: 25.2531793,
				lng: 55.3634841,
				zoom: 5,
				mapType: 'terrain',
				streetView: false
			});
			var path = [
				[25.253179, 55.3634841],
				[13.6932357, 100.7551442],
				[-12.050047116528843, -77.02448169303511],
				[-12.044804866577001, -77.02154422636042]
			];
			var getTile = function(coord, zoom, ownerDocument) {
				var div = ownerDocument.createElement('div');
				div.innerHTML = "";
				div.style.width = this.tileSize.width + 'px';
				div.style.height = this.tileSize.height + 'px';
				div.style.background = "rgba(0,0,0,0.65)";
				div.style.textAlign = 'center';
				div.style.lineHeight = this.tileSize.height + 'px';
				return div;
			};

			map.addOverlayMapType({
				index: 0,
				tileSize: new google.maps.Size(256, 256),
				getTile: getTile
			});

			var that = this;
			var searchAircraftType = function(typesArray, valuetofind) {
				for (var i = 0; i < typesArray.length; i++) {
					if (typesArray[i]['Model'] === valuetofind) {
						return typesArray[i];
					}
				}
				return -1;
			};
			if (industryType === "Aviation") {
				this.flightModel.attachRequestCompleted(function() {
					var poths = [];
					var data = that.flightModel.getProperty("/d/results");

					// 		var	data= that.oModel.getProperty("/response/data/AIRSIGMET/");
					var flightPath = [];
					map.setCenter(data[0]["LAT"], data[0]["LONG"]);
					for (var x = 0; x < data.length; x++) {
						var image2 = new google.maps.MarkerImage(
							'https://www.hscripts.com/freeimages/icons/traffic/regulatory-signs/plane/plane11.gif',
							new google.maps.Size(45, 90),
							new google.maps.Point(0, 0), //origin
							new google.maps.Point(0, 25) //anchor point
						);
						if (data[x].TO === '') {
							data[x].TO = "N/A";
						}
						if (data[x].FROM === '') {
							data[x].FROm = "N/A";
						}
						var aircraftFullModel="";
						var aircraft_model =data[x].AIRCRAFT_MAN_MODEL;
						if(aircraft_model.charAt(0).toLowerCase()==='b'){
							aircraftFullModel=aircraftFullModel+"BOEING "+ data[x].AIRCRAFT_MAN_MODEL.slice(1);
						}
							if(aircraft_model.charAt(0).toLowerCase()==='a'){
							aircraftFullModel=aircraftFullModel+"AIRBUS "+ data[x].AIRCRAFT_MAN_MODEL.slice(1);
						}
						map.addMarker({
							lat: data[x]["LAT"],
							lng: data[x]["LONG"],
							title: data[x]["FLIGHT_NO"],
							icon: "image/airplane-7-24.png",
							//click: function(e) {
							//  alert('You clicked in this marker');
							//},
							infoWindow: {
								content: '<div class="flightDetailContainer"><div class="flightDetailContainerLeft"><div class="fdcFlightNo">' + data[x][
										'FLIGHT_NO'
									] + '</div><div class="fdcFlightOPName">' + data[x]['OPERATED_BY'] + '</div>' +
									'<div class="fdc_To_FromCont"><div class="fdc_to_cont"><div class="fdc_to_short">' + data[x]['FROM'] +
									'</div><div class="fdc_to_full">'+data[x]['FROM']+'</div></div> <span data-sap-ui-icon-content="\e075" class="fdc-icon-flight">&#57461;</span><div class="fdc_to_cont"><div class="fdc_from_short">' +
									data[x]['TO'] +
									'</div><div class="fdc_from_full">'+data[x]['TO']+'</div></div></div><div class="fdc_aircraft_TypeCont"><div class="fdc_aircraft_Type_short">Type <span>' +
									data[x].AIRCRAFT_MAN_MODEL + '</span></div><div class="fdc_aircraftType_fullInfo">'+aircraftFullModel+'</div></div>' + '</div>' +
									'<div class="flightDetailContainerRight"><div class="fdc_link"><a href="#operators"class="fdc_ref"><span class="fdc_ref_icon"></span><span class="fdc_ref_text">Operator</span></a></div><div class="fdc_link"><a href="#operators"class="fdc_ref"><span class="fdc_ref_icon fdc_ref_icon_flight_details"></span><span class="fdc_ref_text">Flight Details</span></a></div><div class="fdc_link"><a href="#operators"class="fdc_ref"><span class="fdc_ref_icon fdc_ref_icon_aircraft"></span><span class="fdc_ref_text">Aircraft</span></a></div><div></div></div>'
							}
						});

						// if(x>0){
						// var polygon = map.drawPolyline({
						//   path: [[data[x-1]["LAT"],data[x-1]["LONG"]],[data[x]["LAT"],data[x]["LONG"]]], // pre-defined polygon shape
						//   strokeColor: '#fff',
						//   strokeOpacity: 0.6,
						//   strokeWeight: 6
						// });

						// }
					}

					// if(data[x].hazard._type==='TURB'){
					// poths.push(xd);
					// map.drawPolygon({
					//   paths: xd, // pre-defined polygon shape
					//   strokeColor: 'blue',
					//   strokeOpacity: 1,
					//   strokeWeight: 3,
					//   fillColor: 'red',
					//   fillOpacity: 0.4
					// });

					// }
					// }
				});

			};

		}

	});

});