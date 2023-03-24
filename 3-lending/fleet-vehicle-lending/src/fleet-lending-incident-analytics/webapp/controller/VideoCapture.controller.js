sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History",
	"sap/ui/model/json/JSONModel"
], function(Controller, History, JSONModel) {
	"use strict";
var _timeout;
	return Controller.extend("com.sap.Aviation.controller.VideoCapture", {

		/**
		 * Called when a controller is instantiated and its View controls (if available) are already created.
		 * Can be used to modify the View before it is displayed, to bind event handlers and do other one-time initialization.
		 * @memberOf com.sap.Aviation.view.VideoCapture
		 */
		onInit: function() {
			var that=this;
			var videoPanel = this.getView().byId("videoPanel");
			
			var arr=[{name:'Overspeeding', desc:'4 wheeler MC5607 overtook above speed limit', icon:'image/feedIcon1.png'},
			{name:'Theft', desc:'2 wheeler identifies theft from Crime Id:461', icon:'image/feedIcon2.png'},
			{name:'Signal break', desc:'Vehicle cuts past pedestrin during red signal', icon:'image/feedIcon3.png'}];
			var func = function(i,that, arr){ 
				var oModel=that.getView().getModel();
				var offences = oModel.getProperty("/Offences");
				offences.unshift(arr[i]);
				oModel.setProperty("/Offences", offences);
				that.getView().byId("idProductList").setVisible(true);
			};
			var vid = this.getView().byId("liveStreamVideo");
			vid.addEventDelegate({
				"onAfterRendering": function () { 
		     		var $vid = this.getView().byId("liveStreamVideo").$()[0];
		          //	$vid.autoplay = true;
					$vid.onloadeddata = function() {
						videoPanel.setBusy(false);
					    setTimeout(function(){
							for(var i in arr){
								var t=parseInt(i);
							//	t+=4;
								setTimeout(func, (t)*4000, parseInt(i), that, arr);
							}
						}, 2000);
					};
			     }
			}, this);
		},

		/**
		 * Similar to onAfterRendering, but this hook is invoked before the controller's View is re-rendered
		 * (NOT before the first rendering! onInit() is used for that one!).
		 * @memberOf com.sap.Aviation.view.VideoCapture
		 */
		//	onBeforeRendering: function() {
		//
		//	},

		/**
		 * Called when the View has been rendered (so its HTML is part of the document). Post-rendering manipulations of the HTML could be done here.
		 * This hook is the same one that SAPUI5 controls get after being rendered.
		 * @memberOf com.sap.Aviation.view.VideoCapture
		 */
		onAfterRendering: function() {
			var videoPanel = this.getView().byId("videoPanel");
						var html1 = new sap.ui.core.HTML("htm1", {
                content:
                        "<video id='liveStreamVideo' width='100%' height='50%' autoplay>" +
                        "<source src='video/liveVideoCapture.mp4'>" +
                        "Your browser does not support the video tag." +
                        "</video>"
			        });
		//	videoPanel.addContent(html1);
			var data = {'Offences':[]};
			var arr=[{name:'Overspeeding', desc:'4 wheeler MC5607 overtook above speed limit', icon:'image/feedIcon1.png'},
			{name:'Theft', desc:'2 wheeler identifies theft from Crime Id:461', icon:'image/feedIcon2.png'},
			{name:'Signal break', desc:'Vehicle cuts past pedestrin during red signal', icon:'image/feedIcon3.png'}];
			var offenceModel = new JSONModel(data);
			this.getView().setModel(offenceModel);
			var that=this;
		},

		/**
		 * Called when the Controller is destroyed. Use this one to free resources and finalize activities.
		 * @memberOf com.sap.Aviation.view.VideoCapture
		 */
		//	onExit: function() {
		//
		//	}
	onNavBack: function(oEvent) {
			var oRoute = this.getOwnerComponent().getRouter();
			oRoute.navTo("navigate");
		},
				onOpenDialog: function(oEvent) {
			// instantiate dialog
			if (!this._dialog) {
				this._dialog = sap.ui.xmlfragment("com.sap.Aviation.view.fragment.MLDialog", this);
				this.getView().addDependent(this._dialog);
			}

			// open dialog
			jQuery.sap.syncStyleClass("sapUiSizeCompact", this.getView(), this._dialog);
			this._dialog.open();

			// simulate end of operation
			_timeout = jQuery.sap.delayedCall(3000, this, function() {
				this._dialog.close();
					this.getOwnerComponent().getRouter().navTo("create", {}, true);
			});
		},
		setVehicalDetails: function(){
		//set vehical details 	
	//	var hasData = "true";
		var comments = "Vehicle Number noted : N97315,  Vehicle Identified: Peugeot 207cc,  Owner's Name : Taif	Michael,  Contact Number: +97142574441 ";
		this.getOwnerComponent().getModel("MLModel").setProperty("/comments",comments);
		},
		handleIncidentCreate: function(){
			
		
			//show the busy indicator before moving 
			this.setVehicalDetails();
				this.onOpenDialog();
				

		}
	});

});