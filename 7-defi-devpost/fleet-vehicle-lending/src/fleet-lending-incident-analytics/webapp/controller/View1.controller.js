sap.ui.define([
  "sap/ui/core/mvc/Controller",
  "com/push/notifications/util/PushController"
], function(Controller, PushController) {
  "use strict";
  return Controller.extend("com.sap.Aviation.controller.View1", {
  onBeforeRendering: function() {
  if (!sap.ui.Device.system.desktop) {
  //push notifications
  PushController.registerForPush();
  alert("Registered for push!");
  }
  }
  });
});