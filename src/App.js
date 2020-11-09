import './App.css';
import React from 'react';
import { promisified, invoke } from 'tauri/api/tauri'

class DirectoryEntry extends React.Component {
    constructor(props) {
        super(props)

        this.state = {}
        this.browsing = false
    }

    browseHover() {
        document.body.style.cursor = "pointer"
        this.setState({"hover": true})
    }

    browseLeave() {
        document.body.style.cursor = "default"
        this.setState({"hover": false})
    }

    browseClick() {
        this.props.onBrowse()
    }

    render() {
        return (
            <div class="directory-div">
                <p class="overtop-label">{this.props.title}</p>
                <div class="directory-inner-div">
                    <p class="directory-text">{this.props.location}</p>
                    <img class="browse-button" src={this.state.hover ? "folder-open.svg" : "folder.svg"} onMouseEnter={this.browseHover.bind(this)} onMouseLeave={this.browseLeave.bind(this)} onMouseDown={this.browseClick.bind(this)}></img>
                </div>
            </div>
        );
    }
}

class FeatureEntry extends React.Component {
    constructor(props) {
        super(props)
        this.state = {}
    }

    onHover() {
        this.setState({"hover": true})
    }

    onLeave() {
        this.setState({"hover": false})
    }

    onClick() {
        this.setState({"checked": !this.state.checked})
    }

    isChecked() {
        return this.state.checked
    }

    generateClassName() {
        var classString = "custom-checkbox"
        
        if (this.state.hover) {
            classString += " checkbox-hover"
        }

        return classString
    }

    render() {
        return (
            <div class="feature-div">
                <div class={this.generateClassName()} onMouseEnter={this.onHover.bind(this)} onMouseLeave={this.onLeave.bind(this)} onMouseDown={this.onClick.bind(this)}>
                    <img src="check.png" class="checkbox-image" hidden={this.state.checked}/>
                </div>
                <span class="feature-text">{this.props.name}</span>
            </div>
        )
    };
}

class OptionalFeatures extends React.Component {
    constructor(props) {
        super(props)
    }

    render() {
        return (
            <div class="feature-list">
                <h3 class="feature-list-text">
                    Optional Features
                </h3>
                <div class="feature-list-grid">
                    {this.props.featureList != null ? this.props.featureList.map((featureName) => <FeatureEntry name={featureName}/>) : []}
                </div>
            </div>
        );
    }
}

class App extends React.Component {
    constructor(props) {
        super(props)

        this.state = {
            "programDirectory": "Unknown",
            "packageDirectory": "Unknown",
        }
        this.browsing = false

        
    }

    componentDidMount() {
        promisified({
            "cmd": "startup"
        }).then((args) => {
            this.setState(args)
        })
    }

    promptInstall() {
        
    }

    onDirectoryBrowse(type) {
        if (this.browsing) {return}

        this.browsing = true

        promisified({
            "cmd": "browse"
        }).then((location) => {
            this.setState({[type + "Directory"]: location})
        }).finally(() => {
            this.browsing = false
        })
    }

    render() {
        return (
            <div>
                  <img class="logo-image" src="logo.png"/>
                  <DirectoryEntry title="Installation Directory" location={this.state.programDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "program")}/>
                  <DirectoryEntry title="Community Packages Directory" location={this.state.packageDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "package")}/>
                  <OptionalFeatures featureList={this.state.featureList}/>
                  <button class="install-button" onClick={this.promptInstall}>Install</button>
            </div>
        );
    }
}

export default App;