jQuery.sap.declare("com.sap.Aviation.util.PushController");
com.push.notifications.util.PushController = {
  regSuccess: function(result) {
  console.log("Successfully registered: " + JSON.stringify(result));
  },
  regFailure: function(errorInfo) {
  console.log("Error while registering.  " + JSON.stringify(errorInfo));
  },
  resetBadgeSuccess: function(result) {
  console.log("Badge has been reset: " + JSON.stringify(result));
  },
  processNotification: function(notification) {
  console.log("Received a notifcation: " + JSON.stringify(notification));
  if (sap.Push.isPlatformIOS()) {
  var notif_alert = JSON.parse(notification).payload.aps.alert;
  var notif_sound = JSON.parse(notification).payload.aps.sound;
  var notif_badge = JSON.parse(notification).payload.aps.badge;
  var notif_data = JSON.parse(notification).payload.data;
  } else {
  var notif_alert = notification.payload.alert;
  var notif_sound = notification.payload.sound;
  var notif_badge = notification.payload.badge;
  var notif_data = notification.payload.data;
  }
  jQuery.sap.require("sap.m.MessageBox");
  sap.m.MessageBox.show(
  notif_data, {
  icon: sap.m.MessageBox.Icon.INFORMATION,
  title: notif_alert,
  actions: [sap.m.MessageBox.Action.OK]
  }
  );
  if (sap.Push.isPlatformIOS()) {
  sap.Push.resetBadge(this.resetBadgeSuccess);
  }
  },
  registerForPush: function() {
  console.log("Device is = " + sap.ui.Device.os);
  var sender = (sap.ui.Device.os.android ? "XXXXXXXXXXXX" : "");
  console.log("Sender is [" + sender + "]");
  console.log("attempting to register for notifications");
  var nTypes = sap.Push.notificationType.SOUNDS | sap.Push.notificationType.BADGE | sap.Push.notificationType.ALERT;
  sap.Push.registerForNotificationTypes(nTypes, this.regSuccess, this.regFailure, this.processNotification, sender); //GCM Sender ID, null for APNS
  },
  unregCallback: function(result) {
  console.log("Successfully unregistered: " + JSON.stringify(result));
  },
  unregisterForPush: function() {
  sap.Push.unregisterForNotificationTypes(this.unregCallback);
  },
  processMissedNotification: function(notification) {
  if (notification) {
  console.log("Received a missed notification: " + JSON.stringify(notification));
  alert("Received a missed notification: " + JSON.stringify(notification));
  }
  if (sap.Push.isPlatformIOS()) {
  sap.Push.resetBadge(this.resetBadgeSuccess);
  }
  },
  checkForNotification: function(notification) {
  setTimeout(function() {
  console.log("Checking for notifications...");
  sap.Push.checkForNotification(this.processMissedNotification);
  // TODO: do your thing!
  }, 0);
  }
};