import './App.css';
import React from 'react';
import { promisified, invoke } from 'tauri/api/tauri'

// Rust calls
var installStep
var setDirectory
var addFeature

function DirectoryEntry(props) {
    return (
        <div class="directory-div">
            <p class="overtop-label">{props.title}</p>
            <p class="directory-text">{props.location}</p>
        </div>
    );
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
            <div>
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
        this.featureList = []

        addFeature = this.addFeature.bind(this)
    }

    addFeature(name) {
        this.featureList.push(<FeatureEntry name={name}/>)
    }

    render() {
        return (
            <div>
                <h3 class="feature-list-text">
                    Optional Features
                </h3>
                <div class="features-grid">
                    {this.featureList}
                </div>
            </div>
        );
    }
}

class App extends React.Component {
    constructor(props) {
        super(props)

        this.state = {}

        setDirectory = (type, path) => {
            const key = type + "Directory"
            this.setState({key: path})
        }
    }

    promptInstall() {
        promisified({
            "cmd": "install",
        })
    }

    render() {
        return (
            <div>
                  <img class="logo-image" src="logo.png"/>
                  <DirectoryEntry title="Installation Directory" location={this.state.programDirectory}/>
                  <DirectoryEntry title="Community Packages Directory" location={this.state.packageDirectory}/>
                  <OptionalFeatures/>
                  <button class="install-button" onClick={this.promptInstall.bind(this)}>Install</button>
            </div>
        );
    }
}

export default App;