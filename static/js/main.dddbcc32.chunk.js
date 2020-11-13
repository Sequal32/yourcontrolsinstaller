(this.webpackJsonpyourcontrolinstaller=this.webpackJsonpyourcontrolinstaller||[]).push([[0],{136:function(t,e,i){"use strict";i.r(e);var n=i(0),s=i(1),a=i.n(s),o=i(47),c=i.n(o),r=(i(55),i(48)),l=i(2),u=i(3),h=i(5),d=i(4),b=(i(8),i(9)),p=function(t){Object(h.a)(i,t);var e=Object(d.a)(i);function i(){return Object(l.a)(this,i),e.apply(this,arguments)}return Object(u.a)(i,[{key:"onAcknowledge",value:function(){this.props.callback&&this.props.callback()}},{key:"render",value:function(){return Object(n.jsxs)("div",{class:"dialog-div",style:this.props.hidden?{top:"25%"}:{top:"-50%"},children:[Object(n.jsx)("h1",{children:this.props.title}),Object(n.jsx)("p",{children:this.props.description}),Object(n.jsx)("button",{class:"generic-button dialog-button",onClick:this.onAcknowledge.bind(this),children:this.props.buttonText})]})}}]),i}(a.a.Component),f=function(t){Object(h.a)(i,t);var e=Object(d.a)(i);function i(t){var n;return Object(l.a)(this,i),(n=e.call(this,t)).state={},n.browsing=!1,n}return Object(u.a)(i,[{key:"browseHover",value:function(){document.body.style.cursor="pointer",this.setState({hover:!0})}},{key:"browseLeave",value:function(){document.body.style.cursor="default",this.setState({hover:!1})}},{key:"browseClick",value:function(){this.props.onBrowse()}},{key:"render",value:function(){return Object(n.jsxs)("div",{class:"directory-div",children:[Object(n.jsx)("p",{class:"overtop-label",children:this.props.title}),Object(n.jsxs)("div",{class:"directory-inner-div",children:[Object(n.jsx)("p",{class:"directory-text",children:this.props.location}),Object(n.jsx)("img",{class:"browse-button",src:this.state.hover?"folder-open.svg":"folder.svg",onMouseEnter:this.browseHover.bind(this),onMouseLeave:this.browseLeave.bind(this),onMouseDown:this.browseClick.bind(this)})]})]})}}]),i}(a.a.Component),v=function(t){Object(h.a)(i,t);var e=Object(d.a)(i);function i(t){var n;return Object(l.a)(this,i),(n=e.call(this,t)).state={hover:!1},n}return Object(u.a)(i,[{key:"onHover",value:function(){this.setState({hover:!0})}},{key:"onLeave",value:function(){this.setState({hover:!1})}},{key:"onClick",value:function(){this.props.callback&&this.props.callback()}},{key:"generateClassName",value:function(){var t="custom-checkbox";return this.state.hover&&(t+=" checkbox-hover"),t}},{key:"render",value:function(){return Object(n.jsxs)("div",{class:"feature-div",children:[Object(n.jsx)("div",{class:this.generateClassName(),onMouseEnter:this.onHover.bind(this),onMouseLeave:this.onLeave.bind(this),onMouseDown:this.onClick.bind(this),checked:this.state.checked,children:Object(n.jsx)("img",{src:"check.png",class:"checkbox-image",hidden:!this.props.checked})}),Object(n.jsx)("span",{class:"feature-text",children:this.props.name})]})}}]),i}(a.a.Component),g=function(t){Object(h.a)(i,t);var e=Object(d.a)(i);function i(t){var n;return Object(l.a)(this,i),(n=e.call(this,t)).groupMap={},n.selectedFeatures=new Set,n}return Object(u.a)(i,[{key:"componentDidUpdate",value:function(t){t.features!=this.props.features&&(this.selectedFeatures.clear(),this.groupMap.clear())}},{key:"selectFeature",value:function(t,e){if(this.selectedFeatures.has(t))this.selectedFeatures.delete(t);else{if(e){var i=this.groupMap[e];i&&this.selectedFeatures.delete(i),this.groupMap[e]=t}this.selectedFeatures.add(t)}this.props.callback&&this.props.callback(this.selectedFeatures),this.forceUpdate()}},{key:"render",value:function(){var t=this;return this.features=null!=this.props.featureList?this.props.featureList.map((function(e){return Object(n.jsx)(v,{name:e.name,callback:t.selectFeature.bind(t,e.name,e.group),checked:t.selectedFeatures.has(e.name)})})):[],Object(n.jsxs)("div",{class:"feature-list",children:[Object(n.jsx)("h3",{children:"Mod Compatibility"}),Object(n.jsx)("p",{children:"This program modifies files that other mods may depend on. Enable these if you would like shared cockpit functionality in these mods, or uncheck if you experience issues."}),Object(n.jsx)("div",{class:"feature-list-grid",children:this.features})]})}}]),i}(a.a.Component),j=i(49),k=function(t){Object(h.a)(i,t);var e=Object(d.a)(i);function i(t){var n;return Object(l.a)(this,i),(n=e.call(this,t)).state={finished:!1},n}return Object(u.a)(i,[{key:"getKeyFrames",value:function(){return[{opacity:0},{opacity:.6}]}},{key:"getTiming",value:function(){return{duration:500,direction:this.props.hidden?"normal":"reverse",fill:"forwards"}}},{key:"componentDidUpdate",value:function(t){this.props.hidden!=t.hidden&&this.setState({finished:!1})}},{key:"onFinish",value:function(){this.props.hidden||this.setState({finished:!0})}},{key:"render",value:function(){return Object(n.jsx)(j.Animated.div,{hidden:this.state.finished,class:"overlay",keyframes:this.getKeyFrames(),timing:this.getTiming(),onFinish:this.onFinish.bind(this)})}}]),i}(a.a.Component),y=function(t){Object(h.a)(i,t);var e=Object(d.a)(i);function i(t){var n;return Object(l.a)(this,i),(n=e.call(this,t)).state={programDirectory:"Unknown",packageDirectory:"Unknown",browsing:!1,installing:!1,currentDialog:{title:"",description:"",buttonText:""},dialogActive:!1},n.selectedFeatures=[],n}return Object(u.a)(i,[{key:"componentDidMount",value:function(){var t=this;Object(b.a)({cmd:"startup"}).then((function(e){if(null!==e.releaseData){var i=new Date(1e3*e.releaseData.date);t.setState({currentDialog:{title:"Release "+e.releaseData.tagName,description:"Released on ".concat(i.toDateString()),buttonText:"OK"},dialogActive:!0})}t.setState(e)}))}},{key:"featuresCallback",value:function(t){this.selectedFeatures=Array.from(t)}},{key:"promptInstall",value:function(){var t=this;this.setState({installing:!0}),Object(b.a)({cmd:"install",features:this.selectedFeatures}).then((function(){t.setState({currentDialog:{title:"Installation Successful",description:"The program has been successfully installed. Get flying!",buttonText:"OK"},dialogActive:!0})})).catch((function(e){t.setState({currentDialog:{title:"Installation Failed",description:e+"\n\nMore info available in Log.txt.",buttonText:"OK"},dialogActive:!0})})).finally((function(){t.setState({installing:!1})}))}},{key:"promptUninstall",value:function(){var t=this;this.setState({uninstalling:!0}),Object(b.a)({cmd:"uninstall"}).then((function(){t.setState({currentDialog:{title:"Uninstalling Successful",description:"The community package has been successfully uninstalled. If you'd like to remove the application, delete the program folder.",buttonText:"OK"},dialogActive:!0})})).catch((function(e){t.setState({currentDialog:{title:"Uninstalling Failed",description:e+"\n\nMore info available in Log.txt.",buttonText:"OK"},dialogActive:!0})})).finally((function(){t.setState({uninstalling:!1})}))}},{key:"onDirectoryBrowse",value:function(t){var e=this;this.state.browsing||(this.setState({browsing:!0}),Object(b.a)({cmd:"browse",browse_for:t}).then((function(i){e.setState(Object(r.a)({},t+"Directory",i))})).finally((function(){e.setState({browsing:!1})})))}},{key:"dialogButtonClicked",value:function(){this.setState({dialogActive:!1})}},{key:"render",value:function(){return Object(n.jsxs)("div",{children:[Object(n.jsx)("img",{class:"logo-image",src:"logo.png"}),Object(n.jsx)(f,{title:"Program Installation Directory",location:this.state.programDirectory,onBrowse:this.onDirectoryBrowse.bind(this,"program")}),Object(n.jsx)(f,{title:"Community Packages Directory",location:this.state.packageDirectory,onBrowse:this.onDirectoryBrowse.bind(this,"package")}),Object(n.jsx)(g,{featureList:this.state.featureList,callback:this.featuresCallback.bind(this)}),Object(n.jsx)("button",{class:"generic-button install-button",onClick:this.promptInstall.bind(this),disabled:this.state.installing,children:this.state.installing?"Installing":"Install"}),Object(n.jsx)("button",{class:"generic-button uninstall-button",onClick:this.promptUninstall.bind(this),disabled:this.state.uninstalling,children:this.state.uninstalling?"Uninstalling":"Uninstall"}),Object(n.jsx)(k,{hidden:this.state.dialogActive}),Object(n.jsx)(p,{hidden:this.state.dialogActive,title:this.state.currentDialog.title,description:this.state.currentDialog.description,buttonText:this.state.currentDialog.buttonText,callback:this.dialogButtonClicked.bind(this)})]})}}]),i}(a.a.Component),O=function(t){t&&t instanceof Function&&i.e(3).then(i.bind(null,137)).then((function(e){var i=e.getCLS,n=e.getFID,s=e.getFCP,a=e.getLCP,o=e.getTTFB;i(t),n(t),s(t),a(t),o(t)}))};c.a.render(Object(n.jsx)(a.a.StrictMode,{children:Object(n.jsx)(y,{})}),document.getElementById("root")),O()},55:function(t,e,i){},8:function(t,e,i){}},[[136,1,2]]]);
//# sourceMappingURL=main.dddbcc32.chunk.js.map