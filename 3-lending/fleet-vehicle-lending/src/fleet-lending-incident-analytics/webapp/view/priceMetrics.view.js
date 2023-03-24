//jQuery.sap.declare("com.sap.aviation.analyticspricemterics.utils.formatter");
sap.ui.jsview("com.sap.Aviation.view.priceMetrics", {

	/** Specifies the Controller belonging to this View. 
	 * In the case that it is not implemented, or that "null" is returned, this View does not have a Controller.
	 * @memberOf com.sap.aviation.analyticspricemterics.view.view.priceMetrics
	 */
	getControllerName: function() {
		return "com.sap.Aviation.controller.priceMetrics";
	},

	/** Is initially called once after the Controller has been instantiated. It is the place where the UI is constructed. 
	 * Since the Controller is given to this method, its event handlers can be attached right away. 
	 * @memberOf com.sap.aviation.analyticspricemterics.view.view.priceMetrics
	 */
	createContent: function(oController) {
		var airlineLogo=new sap.m.Image({
						id:"cLogo1",
						src: {parts:[{path:"claims>/FlightDetails/company"}],
						formatter:function(company){
							var t="./image/airlines/"+company+".png";
							return t;
						}
						}, // sap.ui.core.URI
						width: "100px", // sap.ui.core.CSSSize
						height: "60px", // sap.ui.core.CSSSize
						alt: "Emirates"
						}
				).addStyleClass("airLineLogo");
			var flightID = new sap.m.Text(
				{
					text: "{claims>/FlightDetails/flightID}", // string
					maxLines: 1 // int
				}
			).addStyleClass("flightId");
			var flightFrom=new sap.m.Text({text:"{claims>/FlightDetails/from}"}).addStyleClass("flightToFrom");
			var flightTo=new sap.m.Text({text:"{claims>/FlightDetails/to}"}).addStyleClass("flightToFrom");
			var flightFromFullName=new sap.m.Text({text:"{claims>/FlightDetails/from_fullname}"}).addStyleClass("flightToFromFull");
			var flightToFullName=new sap.m.Text({text:"{claims>/FlightDetails/to_fullname}"}).addStyleClass("flightToFromFull");
			var flightFromVlayout=new sap.ui.layout.VerticalLayout({
				content:[
					flightFrom,flightFromFullName
					]});
					var flightToVlayout=new sap.ui.layout.VerticalLayout({
				content:[
					flightTo,flightToFullName
					]});
				var flightIcon = new sap.ui.core.Icon({
						src: "sap-icon://flight", // sap.ui.core.URI
						size: "18px" // sap.ui.core.CSSSize
						}
				).addStyleClass("flightsIcon");
				
			var	flightInfoLayout = new sap.ui.layout.HorizontalLayout ( {
			content:[flightFromVlayout,flightIcon,flightToVlayout
				]
		}).addStyleClass("flightInfoLayout");
		var	airlineDetailLayout = new sap.ui.layout.HorizontalLayout ( {
			content:[airlineLogo,flightID,flightInfoLayout]
		}).addStyleClass("airlineDetailLayout");
		
		var plTitle=new sap.m.Label({text:"Profit/loss simulation"}).addStyleClass("plTitle");
		
		var oClaimsLineChart = new sap.viz.ui5.controls.VizFrame({
    width : "400px",
    height : "180px",
   
});
var   oClaimsCostBarChart= new sap.viz.ui5.controls.VizFrame({
    width : "400px",
    height : "180px",
   
});
	var oclaimsAmtDataset = new sap.viz.ui5.data.FlattenedDataset({
                    dimensions: [{
                    		axis:1,
                            name: "Year",
                            value: "{Year}"
                            
                        }],
                        measures: [{
                            name: "Claims",
                            value: '{avg_claim}'
                        },
                        {
                            name: "Estimated Claims",
                            value: '{est_claim}'
                        }],
                        data: {
                            path: "claims>/Claim/chartData"
                        }
                    });
                  //  oClaimsLineChart.setModel(oModel);
                 oClaimsLineChart.setDataset(oclaimsAmtDataset );
                 oClaimsLineChart.setVizType('dual_line');
  	var oclaimsCostDataset = new sap.viz.ui5.data.FlattenedDataset({
                    dimensions: [{
                    		axis:1,
                            name: "Year",
                            value: "{Year}"
                            
                        }],
                        measures: [{
                            name: "Cost Of Claims",
                            value: '{avg_claim_cost}'
                        },
                        {
                            name: "Estimated Claims Cost",
                            value: '{est_claim_cost}'
                        }],
                        data: {
                            path: "claims>/Claim/chartData"
                        }
                    });
                  //  oClaimsLineChart.setModel(oModel);
                 oClaimsCostBarChart.setDataset(oclaimsCostDataset);  
                     oClaimsCostBarChart.setVizType('dual_column');
	
			var vizProperties = {
			valueAxis: {
                          title: {
                            visible: true
                        }
			},
				valueAxis2: {
					visible:false,
                          title: {
                            visible: false
                        }
			},
			categoryAxis: {
						
                            title: {
                                visible: false
                            }
			 },
			legend: {
                           visible:false
                        },
                        title: {
                            visible: false
                        },
            plotArea: { colorPalette: ["red", "#d1d6e0"]}
            	};
			oClaimsLineChart.setVizProperties(vizProperties);
			oClaimsCostBarChart.setVizProperties(vizProperties);
		var feedValueAxis = new sap.viz.ui5.controls.common.feeds.FeedItem({
		      'uid': "valueAxis",
		      'type': "Measure",
		      'values': ["Claims"]
		    }), 
		    feedValueAxis0 = new sap.viz.ui5.controls.common.feeds.FeedItem({
		      'uid': "valueAxis2",
		      'type': "Measure",
		      'values': ["Estimated Claims"]
		    }), 
		    feedValueAxis1 = new sap.viz.ui5.controls.common.feeds.FeedItem({
		      'uid': "valueAxis",
		      'type': "Measure",
		      'values': ["Cost Of Claims"]
		    }), 
		    feedValueAxis2 = new sap.viz.ui5.controls.common.feeds.FeedItem({
		      'uid': "valueAxis2",
		      'type': "Measure",
		      'values': ["Estimated Claims Cost"]
		    }), 
		    feedCategoryAxis = new sap.viz.ui5.controls.common.feeds.FeedItem({
		      'uid': "categoryAxis",
		      'type': "Dimension",
		      'values': ["Year"]
		    }),
		      feedCategoryAxis1 = new sap.viz.ui5.controls.common.feeds.FeedItem({
		      'uid': "categoryAxis",
		      'type': "Dimension",
		      'values': ["Year"]
		    });
	oClaimsLineChart.addFeed(feedValueAxis);
	oClaimsLineChart.addFeed(feedValueAxis0);
	oClaimsLineChart.addFeed(feedCategoryAxis);
	 oClaimsCostBarChart.addFeed(feedValueAxis1);
	 oClaimsCostBarChart.addFeed(feedValueAxis2);
	  oClaimsCostBarChart.addFeed(feedCategoryAxis1);
	  var oLayout1 = new sap.ui.layout.form.ResponsiveGridLayout("L1_pc",{
	  	labelSpanL: 9,
			labelSpanM: 9,
			labelSpanS: 9,
			emptySpanL: 1,
			emptySpanM: 1,
			emptySpanS: 1,
			columns:2
	  });

  var claimHistoryDetailsForm = new sap.ui.layout.VerticalLayout ( {
		width: "400px",
			content:[
				new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Average amount of claims in past:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/Claim/avg_claim}",layoutData: new sap.ui.layout.GridData({span: "L2 M2 S2"})}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Average cost of claim in past:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/Claim/avg_claim_cost}"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Estimated amount of claims in {claims>/Claim/estimatedClaim/year}:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/Claim/estimatedClaim/amt}"}).addStyleClass("aRaContentResultValue")
					]}).addStyleClass("pls_separate_row"),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Estimated cost of claims in {claims>/Claim/estimatedClaim/year}:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:{
									path:'claims>/Claim/estimatedClaim/cost',
								formatter:function(s){
									return com.sap.aviation.analyticspricemterics.utils.formatter.numberPriceFormat(s);
									}}}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Insurance value:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/Claim/insurance_value}"}).addStyleClass("aRaContentResultValue")
					]}).addStyleClass("pls_separate_row"),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Loss ratio:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/Claim/loss_ratio}"}).addStyleClass("aRaContentResultValue")
					]}).addStyleClass("pls_separate_row pls_bold_row"),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Estimated loss:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/Claim/estimated_loss}"}).addStyleClass("aRaContentResultValue")
					]}).addStyleClass("pls_bold_row")
			]		
		
	}).addStyleClass("claimHistoryDetailsForm");
	  
		var claimAnalyticChartsLayout = new sap.ui.layout.VerticalLayout ( {
			content:[oClaimsLineChart, oClaimsCostBarChart ]}).addStyleClass("claimAnalyticChartsLayout");

		var claimsAnalyticsContent=new sap.ui.layout.HorizontalLayout ( {
			content:[claimAnalyticChartsLayout,claimHistoryDetailsForm ]
		}).addStyleClass("claimsAnalyticsContent");
		
		var claimsAnalyticsLayout=  new sap.ui.layout.VerticalLayout ( {
			content:[plTitle,claimsAnalyticsContent
				]
		}).addStyleClass("claimsAnalyticsLayout");
		var araTitle=new sap.m.Label({text:"AIRCRAFT RISK ANALYSIS"}).addStyleClass("plTitle");
		
		var aRaContentLeftTitle =new sap.m.Label({text:"Landing"}).addStyleClass("araTitle");
	
	var aRaContentLeftForm = new sap.ui.layout.VerticalLayout ( {
		width: "400px",
			content:[
				new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Runway length:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/FlightDetails/runway_length} m"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Landing margin:"}).addStyleClass("aRaContentResultText"),
							new sap.m.Text({text:"{claims>/FlightDetails/landing_margin} m"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Destination airport:"}).addStyleClass("aRaContentResultText"),
									new sap.m.Text({text:"{claims>/FlightDetails/from_fullname} ({claims>/FlightDetails/from}) "}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Size of operator's fleet:"}).addStyleClass("aRaContentResultText"),
					new sap.m.Text({text:"{claims>/FlightDetails/fleet_size}"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"% of time during night:"}).addStyleClass("aRaContentResultText"),
					new sap.m.Text({text:"{claims>/FlightDetails/night_percent}%"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Flight distance:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/FlightDetails/flight_distance} miles"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Hours of flight:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/FlightDetails/flight_hrs}"}).addStyleClass("aRaContentResultValue")
					]})
			]		
		
	}).addStyleClass("aRaContentLeftForm aRaContentForm");
				
				
		var aRaContentLeftFormResultBox= new sap.ui.layout.HorizontalLayout ( {
			content:[
				new sap.m.Text({
					textAlign : sap.ui.core.TextAlign.Right,
					text:"Landing Risk:"}).addStyleClass("aRaContentAnalysisText"),
				new sap.m.Text({text:"{claims>/FlightDetails/landing_risk}%"}).addStyleClass("aRaContentAnalysisValue")
				]}).addStyleClass("aRaContentLeftFormResultBox aRaContentFormResultBox");
	
		var aRaContentRightTitle =new sap.m.Label({text:"Colision / crash"}).addStyleClass("araTitle");
		
	
	  var aRaContentRightForm = new sap.ui.layout.VerticalLayout ( {
		width: "400px",
			content:[
				new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Operator accidents rate:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/FlightDetails/opp_accd_rate}%"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Aircraft accidents rate:"}).addStyleClass("aRaContentResultText"),
						new sap.m.Text({text:"{claims>/FlightDetails/aircraft_accd_rate}%"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Past flights accidents rate:"}).addStyleClass("aRaContentResultText"),
									new sap.m.Text({text:"{claims>/FlightDetails/past_flts_accd_rate}%"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Weather condition:"}).addStyleClass("aRaContentResultText"),
				new sap.m.Text({text:"{claims>/FlightDetails/weather_condition}"}).addStyleClass("aRaContentResultValue")
					]}),
					new sap.ui.layout.HorizontalLayout ( {
					content:[
						new sap.m.Text({text:"Route sector:"}).addStyleClass("aRaContentResultText"),
				new sap.m.Text({text:"{claims>/FlightDetails/route_sector}"}).addStyleClass("aRaContentResultValue")
					]})
			]		
		
	}).addStyleClass("aRaContentRightForm aRaContentForm");
	
		var aRaContentRightFormResultBox= new sap.ui.layout.HorizontalLayout ( {
			content:[
				new sap.m.Text({
						textAlign : sap.ui.core.TextAlign.Right,
						text:"Probability of crash:"}).addStyleClass("aRaContentAnalysisText"),
				new sap.m.Text({text:"{claims>/FlightDetails/crash_probablity}%"}).addStyleClass("aRaContentAnalysisValue")
				]}).addStyleClass("aRaContentRightFormResultBox aRaContentFormResultBox");
	
		
		var aircraftRiskAnalysisContentLeft= new sap.ui.layout.VerticalLayout ( {
			content:[aRaContentLeftTitle,aRaContentLeftForm,aRaContentLeftFormResultBox
				]
		}).addStyleClass("aircraftRiskAnalysisContentLeft");
		
		
		var aircraftRiskAnalysisContentRight= new sap.ui.layout.VerticalLayout ( {
			content:[aRaContentRightTitle,aRaContentRightForm,aRaContentRightFormResultBox
				]
		}).addStyleClass("aircraftRiskAnalysisContentRight");
		var aircraftRiskAnalysisContent = new sap.ui.layout.HorizontalLayout ( {
			content:[aircraftRiskAnalysisContentLeft,aircraftRiskAnalysisContentRight
				
				]
		}).addStyleClass("aircraftRiskAnalysisContent");
		var aircraftRiskAnalysislayout=  new sap.ui.layout.VerticalLayout ( {
			content:[araTitle,aircraftRiskAnalysisContent
				]
		}).addStyleClass("aircraftRiskAnalysislayout");
		
		/*Adding mapp Information*/
		
		var wraTitle=new sap.m.Label({text:"RISK PER WEATHER"}).addStyleClass("plTitle");
		var wraMap=new sap.ui.core.HTML(
			// string
			{
				id: "wraMap1",
				content:"<div id='wra-map_pm' class='wra-map'></div>"
			}
		);
		
		var weatherRiskContent= new sap.ui.layout.HorizontalLayout ( {
			content:[wraMap
				]
		}).addStyleClass("weatherRiskContent");
		var weatherRiskLayout=new sap.ui.layout.VerticalLayout ( {
			content:[wraTitle,weatherRiskContent
				]
		}).addStyleClass("weatherRiskLayout");
		var vLayout = new sap.ui.layout.VerticalLayout(
			{
					width: "100%", // sap.ui.core.CSSSize
				
				content: [airlineDetailLayout,claimsAnalyticsLayout,aircraftRiskAnalysislayout,weatherRiskLayout
				
					] // sap.ui.core.Control[]
			}
		);
		
		var oPage = new sap.m.Page({
			id:"oPage",
			showHeader:false,
			content: [vLayout]
		});

		var app = new sap.m.App( {
			initialPage: "oPage"
		});
		app.addPage(oPage);
		return app;
	}

});