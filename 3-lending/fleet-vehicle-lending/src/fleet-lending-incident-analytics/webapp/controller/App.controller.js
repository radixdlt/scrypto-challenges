sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/ui/model/json/JSONModel"
], function(Controller, JSONModel) {
	"use strict";

	return Controller.extend("com.sap.Aviation.controller.App", {

		onInit: function() {

			var userModel = new sap.ui.model.json.JSONModel;
			userModel.loadData("/services/userapi/currentUser", "", false);
			var user = userModel.getProperty("/firstName") + " " + userModel.getProperty("/lastName");
			this.getView().byId("userLabel").setText(user);

			var copilotMode = new sap.ui.model.json.JSONModel({
				text: "Initial",
				cont: "",
			});
			this.getView().setModel(copilotMode, "copilotMode");

		},

		navToPeopleProfile: function() {
			location.href =
				"https://dewdfglp01080.wdf.sap.corp:44377/sap/bc/ui5_ui5/ui2/ushell/shells/abap/FioriLaunchpad.html#PersonnelProfile-display";

		},
		handleChangeIndustry: function(oEvent) {

			if (!this._oPopover) {
				this._oPopover = sap.ui.xmlfragment("com.sap.Aviation.view.fragment.ChangeIndustry", this);
				this.getView().addDependent(this._oPopover);
			}

			if (!this._oPopover.isOpen()) {
				var oButton = oEvent.getSource();
				jQuery.sap.delayedCall(0, this, function() {
					this._oPopover.openBy(oButton);
				});
			} else {
				this._oPopover.close();
			}
		},

		handleIndustryClick: function(oEvent) {

			var oRoute = this.getOwnerComponent().getRouter();
			var obj = oEvent.getSource().getProperty("title");
			var model = this.getOwnerComponent().getModel("currentRoleModel");
			var sidenavModel = this.getOwnerComponent().getModel("sideNav");
			sidenavModel.setProperty("/SideNavList/1/visible", false);
			sidenavModel.setProperty("/SideNavList/3/visible", false);
			sidenavModel.setProperty("/SideNavList/5/visible", false);
			sidenavModel.setProperty("/SideNavList/6/visible", false);
			sidenavModel.setProperty("/SideNavList/7/visible", false);
			switch (obj) {
				case "Citizen":
					oRoute.navTo("dashboardDetail");
					model.setProperty("/CurrentRole", "Citizen");
					break;
				case "Police Officer":
					oRoute.navTo("incident");
					// model.setProperty("/CurrentRole", "Police Officer");
					break;
				case "Dispatcher":
					oRoute.navTo("navigate");
					// model.setProperty("/CurrentRole", "Dispatcher");
					break;
				case "Admin":
					oRoute.navTo("dashboardDetail");
					model.setProperty("/CurrentRole", "Admin");
					sidenavModel.setProperty("/SideNavList/1/visible", true);
					sidenavModel.setProperty("/SideNavList/3/visible", true);
					sidenavModel.setProperty("/SideNavList/5/visible", true);
					sidenavModel.setProperty("/SideNavList/6/visible", true);
					sidenavModel.setProperty("/SideNavList/7/visible", true);
					break;
			}
		},

		onCopilotHeaderItemPress: function(oEvent) {
			if (!this._CopilotPopover) {
				this._CopilotPopover = sap.ui.xmlfragment("com.sap.Aviation.view.fragment.TAFragment", this);
				this.getView().addDependent(this._CopilotPopover);
				var feedList = sap.ui.getCore().byId("feedList");

				var feedData = {
					"feedData": [{
						"author": "Copilot",
						"text": "Welcome !",
						"img": "sap-icon://travel-request"
					}]
				};

				var feedModel = new sap.ui.model.json.JSONModel();
				feedModel.setData(feedData);
				this.getOwnerComponent().setModel(feedModel, "feedModel");
				feedList.setModel(feedModel);
				feedList.bindItems({
					path: "/feedData",
					template: new sap.m.FeedListItem({
						sender: "{author}",
						text: "{text}",
						icon: "{img}"
					})
				});
			}

			if (!this._CopilotPopover.isOpen()) {
				var oButton = oEvent.getSource();
				jQuery.sap.delayedCall(0, this, function() {
					this._CopilotPopover.openBy(oButton);
				});
			} else {
				this._CopilotPopover.close();
			}
		},

		onMsgSubmit: function() {

			var oThis = this;
			var copilotMode = this.getView().getModel("copilotMode").getProperty("/text");
			var copilot_context = this.getView().getModel("copilotMode").getProperty("/cont");
			var msgArea = sap.ui.getCore().byId("msgArea");
			var msg = msgArea.getValue();

			var feedModel = this.getOwnerComponent().getModel("feedModel");
			var m = feedModel.getData();
			m.feedData.push({
				"author": "You",
				"text": msg
			});
			feedModel.setData(m);
			msgArea.setValue("");
			var copilotAction = function() {
				var contextFlag = -1;
				var actionFlag = -1;
				for (var i = 0; i < copilot_context.length; i++) {
					if (copilot_context[i].labelPath === "ACTION" && copilot_context[i].normalizedForm === "DISPLAY") {
						actionFlag = 0;
					}
					if (copilot_context[i].labelPath === "CONTEXT_OBJECT" && copilot_context[i].normalizedForm === "incident") {
						contextFlag = 0;
					}
					if (copilot_context[i].labelPath === "CONTEXT_OBJECT" && copilot_context[i].normalizedForm === "incidentStatus") {
						contextFlag = 1;
					}
					if (copilot_context[i].labelPath === "CONTEXT_OBJECT" && copilot_context[i].normalizedForm === "videoCapture") {
						contextFlag = 2;
					}
					if (copilot_context[i].labelPath === "CONTEXT_OBJECT" && copilot_context[i].normalizedForm === "sos") {
						contextFlag = 3;
					}
				}
				var oRoute = oThis.getOwnerComponent().getRouter();
				if (actionFlag === 0 && contextFlag === 0) {
					oRoute.navTo("create");
				}
				if (actionFlag === 0 && contextFlag === 1) {
					oRoute.navTo("incidentStatus");
				}
				if (contextFlag === 2) {
					oRoute.navTo("videoCapture");
				}
				if (contextFlag === 3) {
					sap.m.MessageToast.show('Emergeny incident created and current location shared successfully');
				}
			};
			if (copilotMode === "Initial") {

				var copilot_url = "../destinations/Feeds/COIL/dubaiPoliceParser.xsjs?input=" + msg;
				$.ajax({
					url: copilot_url,
					dataType: 'json',
					async: false,
					success: function(data) {
						copilot_context = JSON.parse(JSON.stringify(data));
					},
					error: function(data) {
						copilot_context = "error";
					}
				});

				m.feedData.push({
					"author": "Copilot",
					"text": "Are you sure ?",
					"img": "sap-icon://travel-request"
				});
				feedModel.setData(m);
				this.getView().getModel("copilotMode").setProperty("/text", "Action");
				this.getView().getModel("copilotMode").setProperty("/cont", copilot_context);
			}
			if (copilotMode === "Action" && (msg === "Yes" || msg === "yes" || msg === "Y" || msg === "y")) {
				copilotAction();
				m.feedData.push({
					"author": "Copilot",
					"text": "Action Completed.",
					"img": "sap-icon://travel-request"
				});
				feedModel.setData(m);
				this.getView().getModel("copilotMode").setProperty("/text", "Initial");
				this.getView().getModel("copilotMode").setProperty("/cont", "");
			}
			if (copilotMode === "Action" && (msg === "No" || msg === "no" || msg === "N" || msg === "n")) {
				m.feedData.push({
					"author": "Copilot",
					"text": "Noted ! Action cancelled.",
					"img": "sap-icon://travel-request"
				});
				feedModel.setData(m);
				this.getView().getModel("copilotMode").setProperty("/text", "Initial");
				this.getView().getModel("copilotMode").setProperty("/cont", "");
			}
		}

	});
});