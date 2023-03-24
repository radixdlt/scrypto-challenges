sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History"
], function(Controller, History) {
	"use strict";
	var _timeout;
	return Controller.extend("com.sap.Aviation.controller.IncidentDetailView", {

		onInit: function() {
			var oModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incidentCollection.json"));
			this.model = oModel;
			var statusModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/incidentStatus.json"));
			
			var self = this;
			
			oModel.attachRequestCompleted(function() {
				var incidents = oModel.getData().IncidentCollection;
				var selectedIncident;
				
				for (var num_3 in incidents) {
					if (incidents[num_3].eventNum == self.event){
						selectedIncident = incidents[num_3];
					}
				}
				self.getView().setModel(new sap.ui.model.json.JSONModel(selectedIncident), "test");
			});

			this.getView().setModel(statusModel,"status");
			var oRouter = sap.ui.core.UIComponent.getRouterFor(this);
			oRouter.getRoute("tabPage").attachMatched(this._onRouteMatched, this);
		},

		onAfterRendering: function() {

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
		},

		_onRouteMatched: function(oEvent) {
			var oArgs;
			oArgs = oEvent.getParameter("arguments");
			if (oArgs.eventNum) {
				this.event = oArgs.eventNum;
			}
		},
		
		onPress: function(oEvent) {
			var oRouter = this.getOwnerComponent().getRouter();
			oRouter.navTo("live");          
		},
		
		onChangeStatusPress: function(oEvent) {
			var oButton = oEvent.getSource();

			// create action sheet only once
			if (!this._actionSheet) {
				this._actionSheet = sap.ui.xmlfragment("com.sap.Aviation.view.fragment.statusActionSheet",this);
				this.getView().addDependent(this._actionSheet);
			}

			this._actionSheet.openBy(oButton);      
		},
		
		statusChanged : function(oEvent){
			var that=this;
			var msgText = "Status Changed successfully";
			var msgState = "Success";
			var actionText = oEvent.getSource().getText();
			var model = this.getView().getModel("test");
			var status = model.getProperty("/status");
			var comments = model.getProperty("/comments");
			var chat = model.getProperty("/ChatCollection");
			var policeAssigned = model.getProperty("/policeAssigned");
			if(status===actionText){
				msgState = "Warning";
				msgText = "Already in "+status+" state";
			}else if(comments==="" || chat.length===0 || policeAssigned===false){
				msgState = "Error";
				msgText = "Please upload relevant evidences to proceed with the status change.";
			}else{
				model.setProperty("/status",actionText);
			}
			this.onOpenDialog(oEvent);
				setTimeout(function() {
					that.showStatusChangeDialog(msgState, msgText);
				}, 10000);
		},
		
		onOpenDialog: function(oEvent) {
			// instantiate dialog
			if (!this._dialog) {
				this._dialog = sap.ui.xmlfragment("com.sap.Aviation.view.fragment.statusChangeBusyDialog", this);
				this.getView().addDependent(this._dialog);
			}

			// open dialog
			jQuery.sap.syncStyleClass("sapUiSizeCompact", this.getView(), this._dialog);
			this._dialog.open();

			// simulate end of operation
			_timeout = jQuery.sap.delayedCall(10000, this, function() {
				this._dialog.close();
			});
		},
		
		showStatusChangeDialog: function(msgState, msgText){
				var dialog = new sap.m.Dialog({
					title: msgState,
					type: 'Message',
					state: msgState,
					content: new sap.m.Text({
						text: msgText
					}),
					beginButton: new sap.m.Button({
						text: 'OK',
						press: function () {
							dialog.close();
						}
					}),
					afterClose: function() {
						dialog.destroy();
					}
				});

			dialog.open();
		}

		/**
		 * Called when the Controller is destroyed. Use this one to free resources and finalize activities.
		 * @memberOf com.sap.Aviation.view.IncidentDetailView
		 */
		//	onExit: function() {
		//
		//	}

	});

});