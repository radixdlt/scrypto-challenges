jQuery.sap.require("com.sap.Aviation.utils.formatter");
sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/core/routing/History",
	'sap/m/Dialog',
	'sap/m/Label',
	'sap/m/MessageToast',
	'sap/m/TextArea',
	'sap/m/Button',
	'sap/m/RatingIndicator'
], function(Controller, History, Dialog, Label, MessageToast, TextArea, Button, RatingIndicator) {
	"use strict";
	var _timeout;
	return Controller.extend("com.sap.Aviation.controller.IncidentCustomer", {

		onInit: function() {

			var oRouter = sap.ui.core.UIComponent.getRouterFor(this);
			oRouter.getRoute("incidentCustomer").attachMatched(this._onRouteMatched, this);
		},
		_onRouteMatched: function(oEvent) {
			var oModel;
			var selectedIncident;
			var oArgs = oEvent.getParameter("arguments");
			var incidents;
			oModel = this.getOwnerComponent().getModel("IncidentsData");
			incidents = oModel.getData().IncidentsReported;
			for (var num_3 in incidents) {
				if (incidents[num_3].eventNum == oArgs.eventId) {
					selectedIncident = incidents[num_3];
				}
			}
			this.getView().setModel(new sap.ui.model.json.JSONModel(selectedIncident), "test");
			// oModel.attachRequestCompleted(function() {
			// 	incidents = oModel.getData().IncidentsReported;
			// 	self.setIncidentDetails(self, incidents, oArgs.eventId);
			// }
		},

		onChangeStatusPress: function() {
			var dialog = new Dialog({
				title: 'Rating',
				type: 'Message',
				content: [
					new RatingIndicator('customerrating', {
						maxValue: 5,
						value: 4,
						tooltip: "Rating Tooltip"
					}),
					new TextArea('submitDialogTextarea', {
						liveChange: function(oEvent) {
							var sText = oEvent.getParameter('value');
							var parent = oEvent.getSource().getParent();

							parent.getBeginButton().setEnabled(sText.length > 0);
						},
						width: '100%',
						placeholder: 'Add note (required)'
					})
				],
				beginButton: new Button({
					text: 'Submit',
					enabled: false,
					press: function() {
						var sText = sap.ui.getCore().byId('submitDialogTextarea').getValue();
						MessageToast.show("Thanks for Reviews");
						dialog.close();
					}
				}),
				endButton: new Button({
					text: 'Cancel',
					press: function() {
						dialog.close();
					}
				}),
				afterClose: function() {
					dialog.destroy();
				}
			});

			dialog.open();
		},
		onNavBack: function(oEvent) {
			var chatArea = this.getView().byId("chatHistory");
			chatArea.setVisible(false);
			chatArea.setValue("");
			var saveButton = this.getView().byId("saveIncident");
			saveButton.setEnabled(false);
			var oHistory, sPreviousHash;
			oHistory = History.getInstance();
			sPreviousHash = oHistory.getPreviousHash();
			this.getOwnerComponent().getRouter().navTo("incidentStatus", {}, true);
		},
		onEditIncident: function(oEvent) {
			var chatArea = this.getView().byId("chatHistory");
			chatArea.setVisible(true);
		},

		onchatActions: function(oEvent) {
			var saveButton = this.getView().byId("saveIncident");
			saveButton.setEnabled(true);
		},
		onSaveSubmit: function() {
			var chatArea = this.getView().byId("chatHistory");
			var page = this.getView().byId("incidentCustomer");
			var existingChat = page.getModel("test").getProperty("/ChatCollection");
			var user = existingChat[0].Author;
			existingChat.unshift({
				"Author": user,
				"AuthorPicUrl": "",
				"Type": "Request",
				"Date": "Jan 13 2018",
				"Text": chatArea.getValue()

			});
			this.showBusyIndicator(1000, 0);
			chatArea.setValue("");
			page.getModel("test").refresh();

		},
		hideBusyIndicator: function() {
			sap.ui.core.BusyIndicator.hide();
			var saveButton = this.getView().byId("saveIncident");
			saveButton.setEnabled(false);
		},

		showBusyIndicator: function(iDuration, iDelay) {
			sap.ui.core.BusyIndicator.show(iDelay);

			if (iDuration && iDuration > 0) {
				if (this._sTimeoutId) {
					jQuery.sap.clearDelayedCall(this._sTimeoutId);
					this._sTimeoutId = null;
				}

				this._sTimeoutId = jQuery.sap.delayedCall(iDuration, this, function() {
					this.hideBusyIndicator();
				});
			}
		}

	});
});