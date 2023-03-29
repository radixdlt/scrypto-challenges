sap.ui.define([
	"sap/ui/core/mvc/Controller"
], function(Controller) {
	"use strict";
	var self;
	var areasModel;
	return Controller.extend("com.sap.Aviation.controller.predictiveAnalysis", {

		/**
		 * Called when a controller is instantiated and its View controls (if available) are already created.
		 * Can be used to modify the View before it is displayed, to bind event handlers and do other one-time initialization.
		 * @memberOf com.sap.Aviation.view.predictiveAnalysis
		 */
		onInit: function() {
			self = this;
			areasModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/areas.json"));
			this.getView().setModel(areasModel, "areas");

			areasModel = new sap.ui.model.json.JSONModel(jQuery.sap.getModulePath("com.sap.Aviation.model", "/areas.json"));
			this.getView().setModel(areasModel, "areas");
		},

		onNavBack: function() {
			this.getOwnerComponent().getRouter().navTo("dashboardDetail", {}, true /*no history*/ );
		},

		onSliderChange: function(oEvent) {
			var value1 = this.getView().byId("idS1").getValue(); //oEvent.getSource();
			var value2 = this.getView().byId("idS2").getValue();
			var value3 = this.getView().byId("idS3").getValue();
			var value = parseInt(value1) + parseInt(value2) - parseInt(value3);
			value = Math.sqrt(value);
			value = (value / 100) * 100;
			value = Math.round(value);
			value = value + " % ";
			this.getView().byId("idCrimeRate").setText(value);
			//Math.floor((Math.random() * 10) + 1);
		},
		onCrimeRateSliderChange: function() {
			var value1 = this.getView().byId("idSCR").getValue();
			var value = parseInt(value1);
			//var ceilValue = Math.ceil(value/10);
			if(value == "15"){
			this.getView().byId("idSL").setText("");
			this.getView().byId("idPC").setText("");
			this.getView().byId("idNRI").setText("");
				this.getView().byId("cromeMicroChart").setPercentage(0);
			return;
			}
			else if(value>15){
				var sign = "+";
					//var diff = 83-value;
					var ceilValue = value/10;//parseFloat(diff*0.1);
			}else if(value<15){
					var sign = "-";
					var diff = 15-value;
					var ceilValue = diff/4;
			}
		
			if(sign==="+"){
			this.getView().byId("idSL").setText("(-"+  Math.abs(ceilValue*5) +")");
			this.getView().byId("idPC").setText("(-"+  Math.abs(ceilValue*10) +")");
			this.getView().byId("idNRI").setText("(+"+  Math.abs(ceilValue*3) +")");
			}else{
				this.getView().byId("idSL").setText("(+"+  Math.abs(ceilValue*5) +")");
			this.getView().byId("idPC").setText("(+"+  Math.abs(ceilValue*10) +")");
			this.getView().byId("idNRI").setText("(-"+  Math.abs(ceilValue*3) +")");
			}
			this.getView().byId("cromeMicroChart").setPercentage(parseInt((Math.abs(ceilValue*5)+Math.abs(ceilValue*10)+ Math.abs(ceilValue*3))/3));
		},
		onHappinessIndexSliderChange: function() {
			var value1 = this.getView().byId("idSHI").getValue();
			var value = parseInt(value1);
			var ceilValue = Math.ceil(value/10);
			
				if(value == "83"){
			this.getView().byId("idtraffic").setText("");
			this.getView().byId("idACC").setText("");
			this.getView().byId("idIFC").setText("");	
			this.getView().byId("HappinessMicroChart").setPercentage(0);
			return;
			}
			else if(value>83){
				var sign = "+";	
					var diff = 83-value;
					var ceilValue = parseFloat(diff/4);
			}else if(value<83){
					var sign = "-";
					var diff = 83-value;
					var ceilValue = parseFloat(diff/4);
			}
			
			if(sign==="+"){
			this.getView().byId("idtraffic").setText("(-"+ Math.abs(ceilValue*5) +"%)");
			this.getView().byId("idACC").setText("(-"+ Math.abs(ceilValue*10) +"%)");
			this.getView().byId("idIFC").setText("(-"+ Math.abs(ceilValue*10) +"%)");
			}else{
				this.getView().byId("idtraffic").setText("(+"+ Math.abs(ceilValue*5) +"%)");
			this.getView().byId("idACC").setText("(+"+ Math.abs(ceilValue*10) +"%)");
			this.getView().byId("idIFC").setText("(+"+ Math.abs(ceilValue*10) +"%)");
			}
				this.getView().byId("HappinessMicroChart").setPercentage(parseInt((Math.abs(ceilValue*5)+Math.abs(ceilValue*10)+ Math.abs(ceilValue*10))/3));
		}

		/**
		 * Similar to onAfterRendering, but this hook is invoked before the controller's View is re-rendered
		 * (NOT before the first rendering! onInit() is used for that one!).
		 * @memberOf com.sap.Aviation.view.predictiveAnalysis
		 */
		//	onBeforeRendering: function() {
		//
		//	},

		/**
		 * Called when the View has been rendered (so its HTML is part of the document). Post-rendering manipulations of the HTML could be done here.
		 * This hook is the same one that SAPUI5 controls get after being rendered.
		 * @memberOf com.sap.Aviation.view.predictiveAnalysis
		 */
		//	onAfterRendering: function() {
		//
		//	},

		/**
		 * Called when the Controller is destroyed. Use this one to free resources and finalize activities.
		 * @memberOf com.sap.Aviation.view.predictiveAnalysis
		 */
		//	onExit: function() {
		//
		//	}

	});

});