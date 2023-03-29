sap.ui.core.Control
		.extend(
				"com.sap.Aviation.graph.groupNetwork",
				{
					// metadata
					metadata : {
						properties : {
							graphData : {
								type : "object"
							}
						},
						events : {
							press : {
								enablePreventDefault : true
							},
							hover : {
								enablePreventDefault : true
							},
							doublePress : {
								enablePreventDefault : true
							}
						}
					},

					// init function
					init : function() {

					}, // renderer
					renderer : function(oRm, oCtrl) {

						oRm.write("<div ");
						oRm.writeControlData(oCtrl);
						oRm.addStyle("width", "80%");// oCtrl.getWidth());
						oRm.addStyle("height", "80%");// oCtrl.getHeight());
						oRm.writeStyles();
						oRm.writeClasses();
						oRm.write(">");
						oRm.write("</div>");

					},
					onAfterRendering : function() {
						var oCtrl = this;
						this.avg; // window.innerWidth provides the inner
						// width of a
						// window's content area excluding scrollbars.
						var w = window.innerWidth - 32, // scroll 20px + 2px
						// border(left+right) +
						// 10px margin
						h = window.innerHeight - 102, node, link, root, lastNodeId = 0, colorCounter = 0, colorMap = {};

						var values = [];// node values collection for range
						// calculation
						var nodeType = [];
						var colorList = [ '#ff7f0e', '#2ca02c', '#d62728',
								'#9467bd', '#8c564b', '#e377c2', '#7f7f7f',
								'#bcbd22', '#17becf' ];

						var boundaryMargin = 35;
						var DELAY = 300, clicks = 0, timer = null;

						var force = d3.layout.force().on("tick", tick)
						// .linkDistance(50)
						.size([ w, h ]);

						var vis = d3.select("#" + oCtrl.getId()).append("svg")
								.attr("width", w).attr("height", h);

						var dataArray = oCtrl.getGraphData();
						var data = convertData(dataArray);
						root = data[0];

						// root = json;
						root.x = w / 2;
						root.y = h / 2;

						setValues(root);

						var min, max;
						if (values.length > 0) {
							min = d3.min(values);
							max = d3.max(values);
						} else {
							min = 100;
							max = 200;
						}
						this.avg = (min + max) / 2;
						if (nodeType.length > 0) {
							nodeType = array_unique(nodeType);
							for ( var i = 0; i < nodeType.length; i++) {
								colorMap[nodeType[i]] = getColor();
							}

						}

						scale = d3.scale.linear().domain([ min, max ])// min
						// and
						// max
						// value
						// of
						// node
						// value
						.range([ 0.5, 1.5 ]);

						force
								.charge(function(d) {
									return -parseFloat(scale(d.VALUE ? d.VALUE
											: 400)) * 3500;
								});

						var nodes = flatten(root);

						formatCollapseNodes(root);
						root.lastNodeId = lastNodeId;
						update();

						function update(type) {
							var nodes = flatten(root, type);
							var links = d3.layout.tree().links(nodes);

							// releaseSticky(nodes);

							// Restart the force layout.
							force.nodes(nodes).links(links).start();

							drag = force.drag().on("dragstart", dragstart);

							// Update the linksï¿½
							link = vis.selectAll("line.link").data(links,
									function(d) {
										return d.target.id;
									});

							// Enter any new links.
							link.enter().insert("line", ".node").attr("class",
									"link");
							// Exit any old links.
							link.exit().remove();

							/*
							 * // Update the linksï¿½ text =
							 * vis.selectAll("text.label").data(nodes,
							 * function(d) { return d.id; });
							 */

							node = vis.selectAll(".node").data(nodes,
									function(d) {
										return d.id;
									});

							var nodeEnter = node.enter().append("g").attr(
									"class", "node").on("click",
									onClickNodeHandler) // Event Handlers for
							// click, double click
							// and mouse hover.
							.on("mouseover", onMousemoveNodeHandler).on(
									"dblclick", onDoubleClickNodeHandler).call(
									drag);

							nodeEnter.append("circle").attr("r", function(d) {
								return d.r + 'em';
							});

							nodeEnter.append("text").attr("dy", function(d) {
								return -(d.r+0.25) + 'em';
							}).insert("tspan").attr("class", "headerLabel")
									.text(function(d) {
										if(d.SubCategory){
											return ".." + d.SubCategory + "..";
										}else{
											return d.NAME;
										}
											
									});

							node.select("circle").style("fill", color);
							node.exit().remove();
						}

						// handler for mouse event
						function onMousemoveNodeHandler(d) {
							oCtrl.fireHover(d);
							// console.log('onMousemoveNodeHandler: pos X '+ d.x
							// + " pos Y " + d.y);
						}
						// handler for double click event
						function onDoubleClickNodeHandler(d) {
							oCtrl.fireDoublePress(d);
							// console.log('onDoubleClickNodeHandler: pos X '+
							// d.x + " pos Y " + d.y);
						}

						function dragstart(d) {
							// if(root.name == d.name)
							// releaseSticky();

							d3.select(this).classed("fixed", d.fixed = true);
						}

						function tick() {
							/*
							 * node.attr("transform", function(d) { return
							 * "translate(" + Math.max(boundaryMargin,
							 * Math.min(w - boundaryMargin, d.x)) + "," +
							 * Math.max(boundaryMargin, Math.min(h -
							 * boundaryMargin, d.y)) + ")"; });
							 */

							node.select("circle").attr(
									"cx",
									function(d) {
										return d.x = Math.max(boundaryMargin,
												Math.min(w - boundaryMargin,
														d.x));
									}).attr(
									"cy",
									function(d) {
										return d.y = Math.max(boundaryMargin,
												Math.min(h - boundaryMargin,
														d.y));
									});
							node.select("text").attr("x", function(d) {
								return d.x;
							}).attr("y", function(d) {
								return d.y;
							});

							link.attr("x1", function(d) {
								return d.source.x;
							}).attr("y1", function(d) {
								return d.source.y;
							}).attr("x2", function(d) {
								return d.target.x;
							}).attr("y2", function(d) {
								return d.target.y;
							});
							link.style("stroke",'#000000');

						}

						// Color leaf nodes orange, and packages white or blue.
						function color(d) {
							if (d.loadDynamic)
								return "#3182bd";

							if (d._children)
								return "#3182bd";
							else
								return d.color;
						}

						// Toggle children on click.
						function onClickNodeHandler(d) {

							if (d3.event.defaultPrevented)
								return; // click suppressed
							// dynamic loading
							clicks++;

							if (clicks === 1) {

								timer = setTimeout(
										function() {

											oCtrl.firePress(d);

											if (d.loadDynamic) {
												d3
														.json(
																d.url,
																function(json) {
																	d.children = [];
																	if (json) {
																		for (i = 0; i < json.RESULTS.length; i++) {
																			updateUniqueId(json.RESULTS[i]);
																			d.children
																					.push(json.RESULTS[i]);

																		}

																		d.loadDynamic = false;
																		formatCollapseNodes(root);
																		setValues(root);

																		var min, max;
																		if (values.length > 0) {
																			min = d3
																					.min(values);
																			max = d3
																					.max(values);
																		} else {
																			min = 100;
																			max = 200;
																		}
																		oCtrl.avg = (min + max) / 2;
																		colorCounter = 0;
																		if (nodeType.length > 0) {
																			nodeType = array_unique(nodeType);
																			for ( var i = 0; i < nodeType.length; i++) {
																				colorMap[nodeType[i]] = getColor();
																			}

																		}

																		scale = d3.scale
																				.linear()
																				.domain(
																						[
																								min,
																								max ])
																				.range(
																						[
																								0.5,
																								2.0 ]);

																		update();
																	}
																});
											} else {
												collapseOrExpand(d);
												// removeId(root);
												// releaseSticky();
												update();
											}
											clicks = 0;

										}, DELAY);

							} else {

								clearTimeout(timer);

								clicks = 0;
							}

						}

						function updateUniqueId(node) {
							if (node.children)
								node.children.reduce(function(p, v) {
									return updateUniqueId(v);
								}, 0);

							if (!node.NODEID) {
								node.id = ++root.lastNodeId;
							} else {
								node.id = node.NODEID;
								root.lastNodeId = node.id;
							}

							return;
						}

						function collapseOrExpand(d) {
							// collapse
							if (d.children && d.children.length > 0) {
								d._children = d.children;
								d.children = null;
								if (d.COLLAPSE)
									d.COLLAPSE = false;
							}
							// expand
							else {
								d.children = d._children;
								d._children = null;
							}
						}

						function releaseSticky(nodes) {
							for ( var i = 0; i < nodes.length; i++) {
								nodes[i].fixed = false;
							}
						}

						function setValues(node) {
							if (node.children)
								node.children.reduce(function(p, v) {
									return setValues(v);
								}, 0);

							if (node.VALUE)
								values.push(parseInt(node.VALUE));

							if (node.NODETYPE)
								nodeType.push(node.NODETYPE);

							return;
						}

						function removeId(node) {

							if (node.children)
								node.children.reduce(function(p, v) {
									return removeId(v);
								}, 0);
							if (node._children)
								node._children.reduce(function(p, v) {
									return removeId(v);
								}, 0);

							delete node.id;
							return;
						}

						function formatCollapseNodes(node) {

							if (node.children)
								node.children.reduce(function(p, v) {
									return formatCollapseNodes(v);
								}, 0);

							if (node.COLLAPSE) {
								if (node.children) {
									node._children = node.children;
									node.children = null;
								}
							}
							return;
						}

						// Returns a list of all nodes under the root.
						function flatten(root, type) {
							var nodes = [], i = 0;

							function recurse(node) {

								if (node.children)
									node.children.reduce(function(p, v) {
										return recurse(v);
									}, 0);

								if (!node.id)
									node.id = ++i;

								var r = scale(node.VALUE ? node.VALUE
										: oCtrl.avg);
								node.r = r;

								Object
										.keys(colorMap)
										.forEach(
												function(key) {
													// console.log(key,
													// detail[key]);
													if (node.NODETYPE
															&& node.NODETYPE == key)
														node.color = colorMap[key];
													else if (node.NODETYPE
															&& !colorMap[node.NODETYPE])
														node.color = 'gray';
												});

								nodes.push(node);
								return;
							}

							recurse(root);
							lastNodeId = i;
							return nodes;
						}

						// var ele = document.getElementById("add");
						// ele.addEventListener("click", addNode, false);
						//
						// var el = document.getElementById("remove");
						// el.addEventListener("click", removeNode, false);

						var generatedNodesIdx = []; // storing newly generated
						// nodes index

						function addNode() {
							// removeId(root);

							var newNode = {};
							newNode.name = "Node"
									+ Math.floor((Math.random() * 100) + 1);
							newNode.VALUE = 100;
							var component = root["children"]; // considering
							// component
							// node to add
							// new nodes as
							// a test.
							component.push(newNode);
							generatedNodesIdx.push(component.length - 1);// tracking
							// newly
							// generated
							// nodes
							// to
							// remove
							update();
						}

						function removeNode() {

							var nIdx = generatedNodesIdx[generatedNodesIdx.length - 1];
							if (!nIdx)
								return;

							var component = root["children"];
							component.splice(nIdx, 1);
							generatedNodesIdx.splice(
									generatedNodesIdx.length - 1, 1);
							update();
						}

						function convertData(array) {
							var map = {};
							for ( var i = 0; i < array.length; i++) {
								var obj = array[i];
								obj.children = [];

								map[obj.NODEID] = obj;

								var PARENT = obj.PARENT || '-';
								if (!map[PARENT]) {
									map[PARENT] = {
										children : []
									};
								}
								map[PARENT].children.push(obj);
							}

							return map['-'].children;
						}

						function array_unique(arr) {
							var result = [];
							for ( var i = 0; i < arr.length; i++) {
								if (result.indexOf(arr[i]) == -1) {
									result.push(arr[i]);
								}
							}
							return result;
						}

						function getColor() {
							var color = colorList[colorCounter++];
							if (colorCounter == colorList.length)
								colorCounter = 0;
							return color;
						}

					}
				});